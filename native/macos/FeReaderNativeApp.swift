import SwiftUI
import AppKit
import CryptoKit
import Foundation
import PDFKit
import UniformTypeIdentifiers

// MARK: - Verification

private struct VerificationOptions {
    let outputPath: String
    let fixturePath: String?
    let enabled: Bool

    static func parse(arguments: [String] = CommandLine.arguments) -> VerificationOptions {
        var outputPath = "/tmp/fe-reader-native-preview.png"
        var fixturePath: String?
        var enabled = FileManager.default.fileExists(atPath: "/tmp/fe-reader-capture-preview")
        var index = 1

        while index < arguments.count {
            switch arguments[index] {
            case "--verify-output":
                if index + 1 < arguments.count {
                    outputPath = arguments[index + 1]
                    enabled = true
                    index += 1
                }
            case "--verify-fixture":
                if index + 1 < arguments.count {
                    fixturePath = arguments[index + 1]
                    enabled = true
                    index += 1
                }
            default:
                break
            }
            index += 1
        }

        return VerificationOptions(outputPath: outputPath,
                                   fixturePath: fixturePath,
                                   enabled: enabled)
    }
}

private let verificationOptions = VerificationOptions.parse()

// MARK: - Design Tokens

private enum AppColors {
    static let window = Color(NSColor.windowBackgroundColor)
    static let sidebar = Color(NSColor.controlBackgroundColor)
    static let canvas = Color(NSColor.textBackgroundColor)
    static let panel = Color(NSColor.controlBackgroundColor)
    static let separator = Color(NSColor.separatorColor)
}

// MARK: - App Entry Point

@main
struct FeReaderApp: App {
    @StateObject private var appState = AppState()
    @NSApplicationDelegateAdaptor(AppDelegate.self) private var appDelegate

    var body: some Scene {
        WindowGroup {
            ShellView()
                .environmentObject(appState)
                .frame(minWidth: 980, minHeight: 640)
                .onAppear {
                    appState.openVerificationFixtureIfNeeded()
                    scheduleCaptureIfRequested()
                }
        }
        .windowResizability(.contentSize)
        .commands {
            CommandGroup(after: .newItem) {
                Divider()
                OpenFileButton(appState: appState)
                    .keyboardShortcut("O", modifiers: .command)
            }

            CommandMenu("View") {
                Button("Previous Page") { appState.previousPage() }
                    .keyboardShortcut(.upArrow, modifiers: [])
                    .disabled(appState.document.phase != .open)
                Button("Next Page") { appState.nextPage() }
                    .keyboardShortcut(.downArrow, modifiers: [])
                    .disabled(appState.document.phase != .open)
                Divider()
                Button("Zoom In") { appState.zoomIn() }
                    .keyboardShortcut("+", modifiers: .command)
                    .disabled(appState.document.phase != .open)
                Button("Zoom Out") { appState.zoomOut() }
                    .keyboardShortcut("-", modifiers: .command)
                    .disabled(appState.document.phase != .open)
                Button("Fit Width") { appState.fitWidth() }
                    .keyboardShortcut("9", modifiers: .command)
                    .disabled(appState.document.phase != .open)
                Button("Actual Size") { appState.actualSize() }
                    .keyboardShortcut("8", modifiers: .command)
                    .disabled(appState.document.phase != .open)
                Divider()
                Toggle("Toggle Inspector", isOn: $appState.showInspector)
                    .keyboardShortcut("i", modifiers: [.command, .option])
            }

            CommandMenu("Document") {
                Button("Inspect") { appState.inspectDocument() }
                    .keyboardShortcut("I", modifiers: [.command, .shift])
                    .disabled(appState.document.phase != .open)
                Button("Export Copy...") { appState.exportDocument() }
                    .keyboardShortcut("E", modifiers: [.command, .shift])
                    .disabled(appState.document.phase != .open)
                Button("Validate") { appState.validateDocument() }
                    .keyboardShortcut("V", modifiers: [.command, .shift])
                    .disabled(appState.document.phase != .open)
                Divider()
                Button("Redact...") { appState.redactDocument() }
                    .disabled(true)
            }
        }
    }
}

// MARK: - Sidebar Items

enum SidebarItem: String, CaseIterable, Hashable, Identifiable {
    case library = "Library"
    case recent = "Recent"
    case evidence = "Evidence"

    var id: String { rawValue }

    var iconName: String {
        switch self {
        case .library: return "books.vertical.fill"
        case .recent: return "clock.fill"
        case .evidence: return "checkmark.shield.fill"
        }
    }
}

enum ReaderFitMode: String {
    case fitWidth = "Fit width"
    case actualSize = "Actual size"
    case custom = "Custom zoom"
}

final class DocumentState: ObservableObject {
    enum Phase: Equatable {
        case empty
        case loading
        case open
        case error(String)
    }

    @Published var phase: Phase = .empty
    @Published var documentURL: URL?
    @Published var documentName = ""
    @Published var fileSize: Int64 = 0
    @Published var pageCount: Int?
    @Published var author: String?
    @Published var creationDate: Date?
    @Published var documentHashPrefix: String?
    @Published var pdfDocument: PDFDocument?
    @Published var pageIndex = 0
    @Published var zoomScale: CGFloat = 1.0
    @Published var fitMode: ReaderFitMode = .fitWidth
    @Published var searchQuery = ""
    @Published var searchHitCount: Int?
    @Published var highlightedSelections: [PDFSelection] = []

    var fileSizeFormatted: String {
        let formatter = ByteCountFormatter()
        formatter.countStyle = .file
        return formatter.string(fromByteCount: fileSize)
    }

    var pageLabel: String {
        guard phase == .open, let pageCount else { return "No page" }
        return "Page \(min(pageIndex + 1, pageCount)) of \(pageCount)"
    }
}

struct DocumentOpenResult {
    let fileSize: Int64
    let pageCount: Int
    let author: String?
    let creationDate: Date?
    let hashPrefix: String
    let pdfDocument: PDFDocument
}

enum NativeDocumentAdapter {
    static func open(url: URL) throws -> DocumentOpenResult {
        let attrs = try FileManager.default.attributesOfItem(atPath: url.path)
        let size = (attrs[.size] as? Int64) ?? 0
        guard let pdf = PDFDocument(url: url) else {
            throw CocoaError(.fileReadCorruptFile)
        }
        let attrsDict = pdf.documentAttributes
        let author = attrsDict?[PDFDocumentAttribute.authorAttribute] as? String
        let creationDate = attrsDict?[PDFDocumentAttribute.creationDateAttribute] as? Date
        return DocumentOpenResult(fileSize: size,
                                  pageCount: pdf.pageCount,
                                  author: author,
                                  creationDate: creationDate,
                                  hashPrefix: try sha256Prefix(url: url),
                                  pdfDocument: pdf)
    }

    private static func sha256Prefix(url: URL) throws -> String {
        let handle = try FileHandle(forReadingFrom: url)
        defer { try? handle.close() }

        var hasher = SHA256()
        while true {
            let chunk = try handle.read(upToCount: 1024 * 1024) ?? Data()
            if chunk.isEmpty { break }
            hasher.update(data: chunk)
        }

        return hasher.finalize()
            .prefix(8)
            .map { String(format: "%02x", $0) }
            .joined()
    }
}

private enum RecentDocumentStore {
    private static let key = "FeReaderRecentDocumentPaths"

    static func load() -> [URL] {
        let paths = UserDefaults.standard.stringArray(forKey: key) ?? []
        return paths.map { URL(fileURLWithPath: $0) }
    }

    static func save(_ urls: [URL]) {
        UserDefaults.standard.set(urls.map(\.path), forKey: key)
    }
}

struct CliOutcome {
    let status: String
    let detail: String
}

enum NativeCliAdapter {
    static func resolveCliURL() -> URL? {
        if let explicit = ProcessInfo.processInfo.environment["FE_READER_CLI_PATH"],
           FileManager.default.isExecutableFile(atPath: explicit) {
            return URL(fileURLWithPath: explicit)
        }
        if let bundled = Bundle.main.url(forResource: "fe-reader", withExtension: nil),
           FileManager.default.isExecutableFile(atPath: bundled.path) {
            return bundled
        }
        let candidates = [
            FileManager.default.currentDirectoryPath + "/target/debug/fe-reader",
            FileManager.default.currentDirectoryPath + "/target/release/fe-reader"
        ]
        return candidates
            .map { URL(fileURLWithPath: $0) }
            .first { FileManager.default.isExecutableFile(atPath: $0.path) }
    }

    static func run(arguments: [String]) throws -> CliOutcome {
        guard let cliURL = resolveCliURL() else {
            return CliOutcome(status: "CLI unavailable",
                              detail: "Build with script/build_and_run.sh or set FE_READER_CLI_PATH.")
        }

        let process = Process()
        let stdout = Pipe()
        let stderr = Pipe()
        process.executableURL = cliURL
        process.arguments = arguments
        process.standardOutput = stdout
        process.standardError = stderr
        try process.run()
        process.waitUntilExit()

        let out = String(data: stdout.fileHandleForReading.readDataToEndOfFile(), encoding: .utf8) ?? ""
        let err = String(data: stderr.fileHandleForReading.readDataToEndOfFile(), encoding: .utf8) ?? ""
        guard process.terminationStatus == 0 else {
            let detail = err.isEmpty ? out : err
            return CliOutcome(status: "CLI failed", detail: sanitize(detail))
        }
        return summarize(output: out, command: arguments.first ?? "command")
    }

    private static func summarize(output: String, command: String) -> CliOutcome {
        guard let data = output.data(using: .utf8),
              let object = try? JSONSerialization.jsonObject(with: data),
              let json = object as? [String: Any] else {
            return CliOutcome(status: "\(command) complete", detail: "Read-only command completed.")
        }

        if let hits = json["hits"] as? [Any] {
            return CliOutcome(status: "Search complete", detail: "\(hits.count) deterministic CLI hit(s).")
        }
        if let report = json["report"] as? [String: Any],
           let findings = report["findings"] as? [Any] {
            return CliOutcome(status: "Accessibility complete", detail: "\(findings.count) accessibility finding(s).")
        }
        if let summary = json["summary"] as? [String: Any],
           let parser = summary["parser"] as? [String: Any] {
            let pageCount = parser["page_count"] ?? "unknown"
            let encrypted = parser["encrypted"] ?? "unknown"
            return CliOutcome(status: "\(command) complete",
                              detail: "Parser pages: \(pageCount); encrypted: \(encrypted).")
        }
        if let lab = json["lab"] as? [String: Any] {
            return CliOutcome(status: "Validation complete",
                              detail: "Lab keys: \(lab.keys.sorted().joined(separator: ", ")).")
        }
        return CliOutcome(status: "\(command) complete", detail: "Read-only JSON evidence received.")
    }

    private static func sanitize(_ value: String) -> String {
        let collapsed = value
            .split(whereSeparator: \.isNewline)
            .prefix(3)
            .joined(separator: " ")
        return collapsed.isEmpty ? "No diagnostic output." : collapsed
    }
}

final class AppState: ObservableObject {
    @Published var document = DocumentState()
    @Published var selectedSidebarItem: SidebarItem = .library
    @Published var showInspector = true
    @Published var recentDocuments: [URL] = RecentDocumentStore.load()
    @Published var inspectSummary = "Not run"
    @Published var metadataSummary = "Not run"
    @Published var searchSummary = "Not run"
    @Published var accessibilitySummary = "Not run"
    @Published var validationSummary = "Not run"
    @Published var mutationSummary = "Mutation pipeline required before redaction."

    private var didOpenVerificationFixture = false

    var cliAvailability: String {
        NativeCliAdapter.resolveCliURL() == nil ? "CLI adapter unavailable" : "CLI adapter ready"
    }

    func openVerificationFixtureIfNeeded() {
        guard !didOpenVerificationFixture,
              let fixture = verificationOptions.fixturePath else { return }
        didOpenVerificationFixture = true
        openDocument(at: URL(fileURLWithPath: fixture))
    }

    func openDocumentPicker() {
        let panel = NSOpenPanel()
        panel.allowedContentTypes = [.pdf]
        panel.allowsMultipleSelection = false
        panel.canChooseDirectories = false
        panel.title = "Open PDF Document"
        panel.message = "Select a PDF document to open in Fe Reader"
        panel.begin { [weak self] response in
            guard response == .OK, let url = panel.url else { return }
            self?.openDocument(at: url)
        }
    }

    func openDocument(at url: URL) {
        let scoped = url.startAccessingSecurityScopedResource()

        document.phase = .loading
        document.documentURL = url
        document.documentName = url.lastPathComponent
        clearDocumentEvidence()

        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self else { return }
            let result = Result { try NativeDocumentAdapter.open(url: url) }
            DispatchQueue.main.async {
                if scoped { url.stopAccessingSecurityScopedResource() }
                switch result {
                case .success(let opened):
                    self.document.fileSize = opened.fileSize
                    self.document.pageCount = opened.pageCount
                    self.document.author = opened.author
                    self.document.creationDate = opened.creationDate
                    self.document.documentHashPrefix = opened.hashPrefix
                    self.document.pdfDocument = opened.pdfDocument
                    self.document.pageIndex = 0
                    self.document.zoomScale = 1.0
                    self.document.fitMode = .fitWidth
                    self.document.phase = .open
                    self.addRecent(url)
                    self.inspectDocument()
                case .failure(let error):
                    self.document.pdfDocument = nil
                    self.document.phase = .error(error.localizedDescription)
                }
            }
        }
    }

    func closeDocument() {
        document = DocumentState()
        clearDocumentEvidence()
    }

    func previousPage() {
        guard document.phase == .open else { return }
        document.pageIndex = max(0, document.pageIndex - 1)
    }

    func nextPage() {
        guard document.phase == .open else { return }
        let maxIndex = max((document.pageCount ?? 1) - 1, 0)
        document.pageIndex = min(maxIndex, document.pageIndex + 1)
    }

    func zoomIn() {
        guard document.phase == .open else { return }
        document.fitMode = .custom
        document.zoomScale = min(document.zoomScale + 0.15, 4.0)
    }

    func zoomOut() {
        guard document.phase == .open else { return }
        document.fitMode = .custom
        document.zoomScale = max(document.zoomScale - 0.15, 0.25)
    }

    func fitWidth() {
        guard document.phase == .open else { return }
        document.fitMode = .fitWidth
    }

    func actualSize() {
        guard document.phase == .open else { return }
        document.fitMode = .actualSize
        document.zoomScale = 1.0
    }

    func inspectDocument() {
        guard let path = document.documentURL?.path, document.phase == .open else { return }
        runCliAction(title: "Inspect", arguments: ["inspect", path, "--json"]) { [weak self] outcome in
            self?.inspectSummary = outcome.detail
        }
        runCliAction(title: "Metadata", arguments: ["metadata", path, "--json"]) { [weak self] outcome in
            self?.metadataSummary = outcome.detail
        }
    }

    func validateDocument() {
        guard let path = document.documentURL?.path, document.phase == .open else { return }
        runCliAction(title: "Validate", arguments: ["lab", "inspect", path, "--json"]) { [weak self] outcome in
            self?.validationSummary = outcome.detail
        }
        runCliAction(title: "Accessibility", arguments: ["accessibility", path, "--json"]) { [weak self] outcome in
            self?.accessibilitySummary = outcome.detail
        }
    }

    func searchDocument() {
        guard let pdf = document.pdfDocument,
              let path = document.documentURL?.path,
              document.phase == .open else { return }
        let query = document.searchQuery.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !query.isEmpty else {
            document.highlightedSelections = []
            document.searchHitCount = nil
            searchSummary = "Enter a query to search locally."
            return
        }

        let selections = pdf.findString(query, withOptions: [.caseInsensitive])
        document.highlightedSelections = selections
        document.searchHitCount = selections.count
        searchSummary = "\(selections.count) local PDFKit hit(s)."
        runCliAction(title: "Search", arguments: ["search", path, query, "--json"]) { [weak self] outcome in
            self?.searchSummary = "\(selections.count) local hit(s); \(outcome.detail)"
        }
    }

    func exportDocument() {
        guard let source = document.documentURL, document.phase == .open else { return }
        let panel = NSSavePanel()
        panel.allowedContentTypes = [.pdf]
        panel.nameFieldStringValue = document.documentName
        panel.title = "Export PDF Copy"
        panel.message = "Create a local copy without mutating the source document."
        panel.begin { response in
            guard response == .OK, let destination = panel.url else { return }
            do {
                if FileManager.default.fileExists(atPath: destination.path) {
                    try FileManager.default.removeItem(at: destination)
                }
                try FileManager.default.copyItem(at: source, to: destination)
            } catch {
                DispatchQueue.main.async {
                    self.document.phase = .error("Export failed: \(error.localizedDescription)")
                }
            }
        }
    }

    func redactDocument() {
        mutationSummary = "Redaction remains disabled until OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt is wired end to end."
    }

    private func runCliAction(title: String,
                              arguments: [String],
                              update: @escaping (CliOutcome) -> Void) {
        update(CliOutcome(status: "\(title) running", detail: "Running read-only CLI evidence..."))
        DispatchQueue.global(qos: .userInitiated).async {
            let outcome: CliOutcome
            do {
                outcome = try NativeCliAdapter.run(arguments: arguments)
            } catch {
                outcome = CliOutcome(status: "\(title) failed", detail: error.localizedDescription)
            }
            DispatchQueue.main.async {
                update(outcome)
            }
        }
    }

    private func addRecent(_ url: URL) {
        recentDocuments.removeAll { $0 == url }
        recentDocuments.insert(url, at: 0)
        if recentDocuments.count > 20 {
            recentDocuments = Array(recentDocuments.prefix(20))
        }
        RecentDocumentStore.save(recentDocuments)
    }

    func openRecent(_ url: URL) {
        guard FileManager.default.fileExists(atPath: url.path) else {
            recentDocuments.removeAll { $0 == url }
            document.phase = .error("The file \"\(url.lastPathComponent)\" could not be found.")
            return
        }
        openDocument(at: url)
    }

    func clearRecents() {
        recentDocuments.removeAll()
        RecentDocumentStore.save(recentDocuments)
    }

    var statusText: String {
        switch document.phase {
        case .empty:
            return "No document open"
        case .loading:
            return "Loading \(document.documentName)"
        case .open:
            return "\(document.documentName) - \(document.pageLabel) - \(document.fileSizeFormatted)"
        case .error:
            return "Document error"
        }
    }

    private func clearDocumentEvidence() {
        inspectSummary = "Not run"
        metadataSummary = "Not run"
        searchSummary = "Not run"
        accessibilitySummary = "Not run"
        validationSummary = "Not run"
        mutationSummary = "Mutation pipeline required before redaction."
    }
}

// MARK: - Shell Layout

struct ShellView: View {
    @EnvironmentObject private var appState: AppState

    var body: some View {
        HStack(spacing: 0) {
            SidebarView()
                .frame(width: 220)
            Divider()
            DocumentCanvas()
                .frame(minWidth: 520)
            if appState.showInspector {
                Divider()
                InspectorPanel()
                    .frame(width: 300)
            }
        }
        .background(AppColors.window)
        .safeAreaInset(edge: .bottom, spacing: 0) {
            StatusBarView()
        }
        .toolbar { AppToolbarContent() }
    }
}

struct StatusBarView: View {
    @EnvironmentObject private var appState: AppState

    var body: some View {
        HStack(spacing: 12) {
            Label(appState.statusText, systemImage: statusIcon)
                .lineLimit(1)
                .truncationMode(.middle)
            Spacer(minLength: 0)
            Text("Local-first")
            Text("Read-only automation")
            Text(appState.cliAvailability)
            if let hash = appState.document.documentHashPrefix,
               appState.document.phase == .open {
                Text("hash \(hash.prefix(8))")
            }
        }
        .font(.caption)
        .foregroundColor(.secondary)
        .padding(.horizontal, 12)
        .frame(height: 24)
        .frame(maxWidth: .infinity)
        .background(AppColors.panel)
        .overlay(alignment: .top) {
            Rectangle()
                .fill(AppColors.separator)
                .frame(height: 1)
        }
        .accessibilityLabel("Status: \(appState.statusText)")
    }

    private var statusIcon: String {
        switch appState.document.phase {
        case .empty: return "doc"
        case .loading: return "hourglass"
        case .open: return "doc.richtext"
        case .error: return "exclamationmark.triangle"
        }
    }
}

// MARK: - Sidebar

struct SidebarView: View {
    @EnvironmentObject private var appState: AppState

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            VStack(alignment: .leading, spacing: 4) {
                Text("Fe Reader")
                    .font(.headline)
                    .foregroundColor(.primary)
                Text("Local PDF workflows")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            .padding(.horizontal, 14)
            .padding(.top, 16)

            VStack(spacing: 4) {
                ForEach(SidebarItem.allCases) { item in
                    SidebarRow(item: item,
                               isSelected: appState.selectedSidebarItem == item) {
                        appState.selectedSidebarItem = item
                    }
                }
            }
            .padding(.horizontal, 8)

            Divider()
                .padding(.horizontal, 12)

            VStack(alignment: .leading, spacing: 8) {
                Text("Recent")
                    .font(.caption)
                    .fontWeight(.semibold)
                    .foregroundColor(.secondary)
                    .textCase(.uppercase)

                if appState.recentDocuments.isEmpty {
                    Text("No recent documents")
                        .font(.caption)
                        .foregroundColor(.secondary)
                } else {
                    ForEach(appState.recentDocuments.prefix(5), id: \.self) { url in
                        Button {
                            appState.openRecent(url)
                        } label: {
                            Label(url.lastPathComponent, systemImage: "doc")
                                .lineLimit(1)
                                .truncationMode(.middle)
                        }
                        .buttonStyle(.plain)
                        .foregroundColor(.primary)
                        .accessibilityLabel("Open recent document \(url.lastPathComponent)")
                    }
                }
            }
            .padding(.horizontal, 14)

            Spacer(minLength: 0)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
        .background(AppColors.sidebar)
        .accessibilityLabel("Navigation sidebar")
        .frame(minWidth: 170)
    }
}

struct SidebarRow: View {
    let item: SidebarItem
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack(spacing: 8) {
                Image(systemName: item.iconName)
                    .frame(width: 18)
                    .accessibilityHidden(true)
                Text(item.rawValue)
                    .font(.subheadline)
                    .lineLimit(1)
                Spacer(minLength: 0)
            }
            .foregroundColor(isSelected ? .white : .primary)
            .padding(.horizontal, 8)
            .padding(.vertical, 6)
            .background(
                RoundedRectangle(cornerRadius: 6)
                    .fill(isSelected ? Color.accentColor : Color.clear)
            )
        }
        .buttonStyle(.plain)
        .accessibilityLabel("\(item.rawValue) sidebar section")
    }
}

// MARK: - Document Canvas

struct DocumentCanvas: View {
    @EnvironmentObject private var appState: AppState

    var body: some View {
        Group {
            switch appState.document.phase {
            case .empty:
                EmptyStateView()
            case .loading:
                LoadingStateView()
            case .open:
                OpenDocumentView()
            case .error(let message):
                ErrorStateView(message: message)
            }
        }
        .onDrop(of: [.fileURL],
                delegate: DocumentDropDelegate(appState: appState))
        .background(AppColors.canvas)
        .accessibilityLabel("Document canvas")
    }
}

struct EmptyStateView: View {
    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "doc.viewfinder")
                .font(.system(size: 56))
                .foregroundColor(.secondary)
                .accessibilityHidden(true)
            Text("Open a PDF Document")
                .font(.title2)
                .fontWeight(.semibold)
            Text("Drop a PDF file here or use File -> Open to begin.")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
            Text("Cmd+O")
                .font(.caption)
                .foregroundColor(.secondary)
                .opacity(0.55)
        }
        .padding(40)
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(AppColors.canvas)
        .overlay {
            RoundedRectangle(cornerRadius: 12)
                .stroke(style: StrokeStyle(lineWidth: 2, dash: [8, 6]))
                .foregroundColor(.secondary.opacity(0.3))
                .padding(20)
        }
        .accessibilityLabel("Drop zone. Open a PDF document to begin.")
    }
}

struct LoadingStateView: View {
    var body: some View {
        VStack(spacing: 16) {
            ProgressView()
                .scaleEffect(1.2)
                .accessibilityHidden(true)
            Text("Loading document...")
                .font(.subheadline)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(AppColors.canvas)
        .accessibilityLabel("Loading document")
    }
}

struct OpenDocumentView: View {
    @EnvironmentObject private var appState: AppState

    var body: some View {
        VStack(spacing: 0) {
            ReaderControlsView()
            ZStack(alignment: .bottomLeading) {
                PDFReaderRepresentable(document: appState.document)
                    .background(Color.white)
                    .overlay {
                        Rectangle()
                            .stroke(AppColors.separator, lineWidth: 1)
                    }
                Text(appState.document.pageLabel)
                    .font(.caption)
                    .foregroundColor(.secondary)
                    .padding(.horizontal, 8)
                    .padding(.vertical, 5)
                    .background(.regularMaterial)
                    .clipShape(RoundedRectangle(cornerRadius: 6))
                    .padding(12)
            }
        }
        .background(AppColors.canvas)
    }
}

struct ReaderControlsView: View {
    @EnvironmentObject private var appState: AppState

    var body: some View {
        HStack(spacing: 8) {
            Button { appState.previousPage() } label: {
                Label("Previous Page", systemImage: "chevron.up")
            }
            .disabled(appState.document.pageIndex == 0)
            Button { appState.nextPage() } label: {
                Label("Next Page", systemImage: "chevron.down")
            }
            .disabled(appState.document.pageIndex >= max((appState.document.pageCount ?? 1) - 1, 0))
            Divider().frame(height: 18)
            Button { appState.zoomOut() } label: {
                Label("Zoom Out", systemImage: "minus.magnifyingglass")
            }
            Button { appState.zoomIn() } label: {
                Label("Zoom In", systemImage: "plus.magnifyingglass")
            }
            Button { appState.fitWidth() } label: {
                Label("Fit Width", systemImage: "arrow.left.and.right")
            }
            Button { appState.actualSize() } label: {
                Label("Actual Size", systemImage: "1.magnifyingglass")
            }
            Divider().frame(height: 18)
            TextField("Search", text: $appState.document.searchQuery)
                .textFieldStyle(.roundedBorder)
                .frame(width: 180)
                .onSubmit { appState.searchDocument() }
            Button { appState.searchDocument() } label: {
                Label("Search", systemImage: "magnifyingglass")
            }
            Spacer(minLength: 0)
            Text(appState.document.fitMode.rawValue)
                .font(.caption)
                .foregroundColor(.secondary)
        }
        .labelStyle(.iconOnly)
        .padding(.horizontal, 10)
        .padding(.vertical, 8)
        .background(AppColors.panel)
        .overlay(alignment: .bottom) {
            Rectangle()
                .fill(AppColors.separator)
                .frame(height: 1)
        }
    }
}

struct PDFReaderRepresentable: NSViewRepresentable {
    @ObservedObject var document: DocumentState

    func makeCoordinator() -> Coordinator {
        Coordinator(document: document)
    }

    func makeNSView(context: Context) -> PDFView {
        let view = PDFView()
        view.displayMode = .singlePageContinuous
        view.displayDirection = .vertical
        view.displaysPageBreaks = true
        view.backgroundColor = NSColor.textBackgroundColor
        view.autoScales = true
        NotificationCenter.default.addObserver(context.coordinator,
                                               selector: #selector(Coordinator.pageChanged(_:)),
                                               name: Notification.Name.PDFViewPageChanged,
                                               object: view)
        context.coordinator.pdfView = view
        return view
    }

    func updateNSView(_ view: PDFView, context: Context) {
        context.coordinator.document = document
        if view.document !== document.pdfDocument {
            view.document = document.pdfDocument
        }
        guard let pdf = document.pdfDocument else { return }

        switch document.fitMode {
        case .fitWidth:
            view.autoScales = true
        case .actualSize, .custom:
            view.autoScales = false
            if abs(view.scaleFactor - document.zoomScale) > 0.01 {
                view.scaleFactor = document.zoomScale
            }
        }

        if let page = pdf.page(at: document.pageIndex),
           view.currentPage !== page {
            view.go(to: page)
        }

        view.highlightedSelections = document.highlightedSelections
        if let first = document.highlightedSelections.first {
            view.setCurrentSelection(first, animate: true)
            view.go(to: first)
        }
    }

    final class Coordinator: NSObject {
        var document: DocumentState
        weak var pdfView: PDFView?

        init(document: DocumentState) {
            self.document = document
        }

        @objc func pageChanged(_ notification: Notification) {
            guard let view = pdfView,
                  let pdf = view.document,
                  let page = view.currentPage else { return }
            let index = pdf.index(for: page)
            DispatchQueue.main.async {
                if index >= 0, self.document.pageIndex != index {
                    self.document.pageIndex = index
                }
            }
        }
    }
}

struct ErrorStateView: View {
    let message: String
    @EnvironmentObject private var appState: AppState

    var body: some View {
        VStack(spacing: 16) {
            Image(systemName: "exclamationmark.triangle.fill")
                .font(.system(size: 40))
                .foregroundColor(.orange)
                .accessibilityHidden(true)
            Text("Unable to Open Document")
                .font(.title3)
                .fontWeight(.semibold)
            Text(message)
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal, 40)
            HStack(spacing: 16) {
                Button("Dismiss") { appState.document.phase = .empty }
                    .keyboardShortcut(.escape)
                Button("Try Again") {
                    if let url = appState.document.documentURL {
                        appState.openDocument(at: url)
                    }
                }
                .keyboardShortcut(.return)
            }
            .padding(.top, 8)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(AppColors.canvas)
        .accessibilityLabel("Error: \(message)")
    }
}

// MARK: - Inspector Panel

struct InspectorPanel: View {
    @EnvironmentObject private var appState: AppState

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            Text("Inspector")
                .font(.headline)
                .padding(.horizontal, 16)
                .padding(.vertical, 12)
                .accessibilityLabel("Inspector panel header")
            Divider()
            ScrollView {
                VStack(alignment: .leading, spacing: 18) {
                    InspectorSection(title: "Document Info") {
                        InfoRow(label: "Name", value: valueOrDash(appState.document.documentName))
                        InfoRow(label: "Size", value: appState.document.phase == .open ? appState.document.fileSizeFormatted : "-")
                        InfoRow(label: "Pages", value: appState.document.pageCount.map(String.init) ?? "-")
                        InfoRow(label: "Position", value: appState.document.phase == .open ? appState.document.pageLabel : "-")
                        InfoRow(label: "Hash", value: appState.document.documentHashPrefix ?? "-")
                        InfoRow(label: "Author", value: appState.document.author ?? "-")
                        InfoRow(label: "Created", value: formattedDate(appState.document.creationDate))
                    }

                    InspectorSection(title: "Core Evidence") {
                        InfoRow(label: "Inspect", value: appState.inspectSummary)
                        InfoRow(label: "Metadata", value: appState.metadataSummary)
                        InfoRow(label: "Search", value: appState.searchSummary)
                        InfoRow(label: "A11y", value: appState.accessibilitySummary)
                        InfoRow(label: "Validate", value: appState.validationSummary)
                    }

                    InspectorSection(title: "Actions") {
                        VStack(spacing: 8) {
                            ActionButton(title: "Inspect", icon: "magnifyingglass", shortcut: "Shift Cmd I") {
                                appState.inspectDocument()
                            }
                            .disabled(appState.document.phase != .open)
                            ActionButton(title: "Export Copy...", icon: "square.and.arrow.up", shortcut: "Shift Cmd E") {
                                appState.exportDocument()
                            }
                            .disabled(appState.document.phase != .open)
                            ActionButton(title: "Validate", icon: "checkmark.shield", shortcut: "Shift Cmd V") {
                                appState.validateDocument()
                            }
                            .disabled(appState.document.phase != .open)
                            Divider()
                            ActionButton(title: "Redact plan only", icon: "eye.slash", shortcut: "") {
                                appState.redactDocument()
                            }
                            .disabled(true)
                            .accessibilityLabel("Redact document unavailable until mutation pipeline is implemented")
                            Text(appState.mutationSummary)
                                .font(.caption)
                                .foregroundColor(.secondary)
                                .fixedSize(horizontal: false, vertical: true)
                        }
                    }
                }
                .padding(16)
            }
        }
        .frame(minWidth: 240)
        .background(AppColors.panel)
        .accessibilityLabel("Inspector panel")
    }

    private func valueOrDash(_ value: String) -> String {
        value.isEmpty ? "-" : value
    }
}

private func formattedDate(_ date: Date?) -> String {
    guard let date else { return "-" }
    let formatter = DateFormatter()
    formatter.dateStyle = .medium
    formatter.timeStyle = .none
    return formatter.string(from: date)
}

struct InspectorSection<Content: View>: View {
    let title: String
    @ViewBuilder let content: Content

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(title)
                .font(.subheadline)
                .fontWeight(.semibold)
                .foregroundColor(.secondary)
                .accessibilityLabel("\(title) section")
            content
        }
    }
}

struct InfoRow: View {
    let label: String
    let value: String

    var body: some View {
        HStack(alignment: .top) {
            Text(label + ":")
                .font(.caption)
                .foregroundColor(.secondary)
                .frame(width: 58, alignment: .trailing)
            Text(value)
                .font(.caption)
                .foregroundColor(.primary)
                .lineLimit(3)
                .truncationMode(.tail)
            Spacer(minLength: 0)
        }
        .accessibilityLabel("\(label): \(value)")
    }
}

struct ActionButton: View {
    let title: String
    let icon: String
    let shortcut: String
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack(spacing: 8) {
                Image(systemName: icon)
                    .frame(width: 16)
                    .accessibilityHidden(true)
                Text(title)
                    .font(.subheadline)
                Spacer(minLength: 0)
                if !shortcut.isEmpty {
                    Text(shortcut)
                        .font(.caption2)
                        .foregroundColor(.secondary)
                        .opacity(0.65)
                }
            }
        }
        .buttonStyle(.borderless)
        .accessibilityLabel(title)
    }
}

// MARK: - Toolbar

struct AppToolbarContent: ToolbarContent {
    @EnvironmentObject private var appState: AppState

    @ToolbarContentBuilder
    var body: some ToolbarContent {
        ToolbarItem(placement: .navigation) {
            Button { appState.openDocumentPicker() } label: {
                Label("Open", systemImage: "folder")
            }
            .keyboardShortcut("O", modifiers: .command)
            .accessibilityLabel("Open document")
        }

        ToolbarItemGroup(placement: .primaryAction) {
            Button { appState.previousPage() } label: {
                Label("Previous Page", systemImage: "chevron.up")
            }
            .disabled(appState.document.phase != .open || appState.document.pageIndex == 0)
            Button { appState.nextPage() } label: {
                Label("Next Page", systemImage: "chevron.down")
            }
            .disabled(appState.document.phase != .open)
            Button { appState.showInspector.toggle() } label: {
                Label("Inspector", systemImage: "sidebar.trailing")
            }
            .accessibilityLabel(appState.showInspector ? "Hide inspector panel" : "Show inspector panel")
        }

        ToolbarTitleMenu {
            Button("Open...") { appState.openDocumentPicker() }
                .keyboardShortcut("O", modifiers: .command)
            Button("Close Document") { appState.closeDocument() }
                .keyboardShortcut("W", modifiers: .command)
                .disabled(appState.document.phase == .empty)
            Divider()
            Toggle("Show Inspector", isOn: $appState.showInspector)
                .keyboardShortcut("i", modifiers: [.command, .option])
        }
    }
}

struct OpenFileButton: View {
    @ObservedObject var appState: AppState

    var body: some View {
        Button("Open...") { appState.openDocumentPicker() }
            .accessibilityLabel("Open a PDF document")
    }
}

// MARK: - Drop Delegate

struct DocumentDropDelegate: DropDelegate {
    let appState: AppState

    func validateDrop(info: DropInfo) -> Bool {
        info.hasItemsConforming(to: [.fileURL])
    }

    func performDrop(info: DropInfo) -> Bool {
        guard let provider = info.itemProviders(for: [.fileURL]).first else {
            return false
        }
        provider.loadItem(forTypeIdentifier: UTType.fileURL.identifier,
                          options: nil) { item, error in
            guard error == nil else { return }
            let url: URL?
            if let data = item as? Data {
                url = URL(dataRepresentation: data, relativeTo: nil)
            } else if let urlItem = item as? URL {
                url = urlItem
            } else {
                url = nil
            }
            guard let resolvedURL = url,
                  resolvedURL.pathExtension.lowercased() == "pdf" else { return }
            DispatchQueue.main.async {
                self.appState.openDocument(at: resolvedURL)
            }
        }
        return true
    }

    func dropUpdated(info: DropInfo) -> DropProposal? {
        DropProposal(operation: .copy)
    }
}

// MARK: - Verification Capture

private final class AppDelegate: NSObject, NSApplicationDelegate {
    func applicationDidFinishLaunching(_ notification: Notification) {
        scheduleCaptureIfRequested()
    }
}

private func scheduleCaptureIfRequested() {
    guard verificationOptions.enabled else { return }
    let delay: DispatchTimeInterval = verificationOptions.fixturePath == nil ? .milliseconds(900) : .milliseconds(2200)
    DispatchQueue.main.asyncAfter(deadline: .now() + delay) {
        captureScreenshot()
    }
}

private func captureScreenshot() {
    guard let window = NSApplication.shared.windows.first,
          let contentView = window.contentView else { return }
    window.isOpaque = true
    window.backgroundColor = NSColor.windowBackgroundColor

    let bounds = contentView.bounds
    guard bounds.width > 0, bounds.height > 0 else { return }
    guard let rep = contentView.bitmapImageRepForCachingDisplay(in: bounds) else { return }
    contentView.cacheDisplay(in: bounds, to: rep)
    guard let data = rep.representation(using: .png, properties: [:]) else { return }

    do {
        let url = URL(fileURLWithPath: verificationOptions.outputPath)
        try FileManager.default.createDirectory(at: url.deletingLastPathComponent(),
                                                withIntermediateDirectories: true)
        try data.write(to: url, options: .atomic)
        try? FileManager.default.removeItem(atPath: "/tmp/fe-reader-capture-preview")
    } catch {
        print("[FeReader] verification capture failed: \(error.localizedDescription)")
    }
}
