import SwiftUI
import AppKit
import CryptoKit
import Foundation
import PDFKit
import UniformTypeIdentifiers

// MARK: - Design Tokens

private enum AppColors {
    static let window = Color(NSColor.windowBackgroundColor)
    static let sidebar = Color(NSColor.controlBackgroundColor)
    static let canvas = Color(NSColor.textBackgroundColor)
    static let panel = Color(NSColor.controlBackgroundColor)
    static let separator = Color(NSColor.separatorColor)
}

// MARK: - App Entry Point

/// Verification capture marker path. Written by the build script before launch.
private let captureMarkerPath = "/tmp/fe-reader-capture-preview"
/// Output path for verification screenshot.
private let captureOutputPath = "/tmp/fe-reader-native-preview.png"

@main
struct FeReaderApp: App {
    @StateObject private var appState = AppState()

    /// AppKit delegate adaptor for verification capture and window-level behavior.
    @NSApplicationDelegateAdaptor(AppDelegate.self) private var appDelegate

    var body: some Scene {
        WindowGroup {
            ShellView()
                .environmentObject(appState)
                .frame(minWidth: 900, minHeight: 520)
                .onAppear {
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
                Toggle("Toggle Inspector", isOn: $appState.showInspector)
                    .keyboardShortcut("i", modifiers: [.command, .option])
            }

            CommandMenu("Document") {
                Button("Inspect") {
                    appState.inspectDocument()
                }
                .keyboardShortcut("I", modifiers: [.command, .shift])
                .disabled(appState.document.phase != .open)

                Button("Export\u{2026}") {
                    appState.exportDocument()
                }
                .keyboardShortcut("E", modifiers: [.command, .shift])
                .disabled(appState.document.phase != .open)

                Divider()

                Button("Validate") {
                    appState.validateDocument()
                }
                .keyboardShortcut("V", modifiers: [.command, .shift])
                .disabled(appState.document.phase != .open)

                Divider()

                Button("Redact\u{2026}") {
                    appState.redactDocument()
                }
                .disabled(appState.document.phase != .open)
            }
        }
    }
}

// MARK: - Sidebar Items

enum SidebarItem: String, CaseIterable, Hashable, Identifiable {
    case library = "Library"
    case recent = "Recent"
    case contracts = "Contracts"

    var id: String { rawValue }

    var iconName: String {
        switch self {
        case .library:  return "books.vertical.fill"
        case .recent:   return "clock.fill"
        case .contracts: return "doc.text.fill"
        }
    }
}

// MARK: - State Models

/// Tracks the current document lifecycle phase.
final class DocumentState: ObservableObject {
    enum Phase: Equatable {
        case empty
        case loading
        case open
        case error(String)
    }

    @Published var phase: Phase = .empty
    @Published var documentURL: URL?
    @Published var documentName: String = ""
    @Published var fileSize: Int64 = 0
    @Published var pageCount: Int?
    @Published var author: String?
    @Published var creationDate: Date?
    @Published var documentHashPrefix: String?

    var fileSizeFormatted: String {
        let formatter = ByteCountFormatter()
        formatter.countStyle = .file
        return formatter.string(fromByteCount: fileSize)
    }
}

struct DocumentMetadata {
    let fileSize: Int64
    let pageCount: Int?
    let author: String?
    let creationDate: Date?
    let hashPrefix: String
}

enum NativeDocumentAdapter {
    static func inspect(url: URL) throws -> DocumentMetadata {
        let attrs = try FileManager.default.attributesOfItem(atPath: url.path)
        let size = (attrs[.size] as? Int64) ?? 0
        let pdf = PDFDocument(url: url)
        let attrsDict = pdf?.documentAttributes
        let author = attrsDict?[PDFDocumentAttribute.authorAttribute] as? String
        let creationDate = attrsDict?[PDFDocumentAttribute.creationDateAttribute] as? Date
        let hashPrefix = try sha256Prefix(url: url)
        return DocumentMetadata(fileSize: size,
                                pageCount: pdf?.pageCount,
                                author: author,
                                creationDate: creationDate,
                                hashPrefix: hashPrefix)
    }

    private static func sha256Prefix(url: URL) throws -> String {
        let handle = try FileHandle(forReadingFrom: url)
        defer { try? handle.close() }

        var hasher = SHA256()
        while true {
            let chunk = try handle.read(upToCount: 1024 * 1024) ?? Data()
            if chunk.isEmpty {
                break
            }
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

/// Top-level application state shared across the shell.
final class AppState: ObservableObject {
    @Published var document = DocumentState()
    @Published var selectedSidebarItem: SidebarItem = .library
    @Published var showInspector: Bool = true
    @Published var recentDocuments: [URL] = RecentDocumentStore.load()

    // MARK: - Document Actions

    func openDocumentPicker() {
        let panel = NSOpenPanel()
        panel.allowedContentTypes = [.pdf]
        panel.allowsMultipleSelection = false
        panel.canChooseDirectories = false
        panel.title = "Open PDF Document"
        panel.message = "Select a PDF document to open in Fe Reader"
        // NSOpenPanel accessibility is set via .title and .message

        panel.begin { [weak self] response in
            guard response == .OK, let url = panel.url else { return }
            self?.openDocument(at: url)
        }
    }

    func openDocument(at url: URL) {
        guard url.startAccessingSecurityScopedResource() else {
            document.phase = .error("Could not access security-scoped resource.")
            return
        }
        defer { url.stopAccessingSecurityScopedResource() }

        document.phase = .loading
        document.documentURL = url
        document.documentName = url.lastPathComponent

        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            guard let self else { return }
            let result = Result { try NativeDocumentAdapter.inspect(url: url) }

            DispatchQueue.main.async {
                switch result {
                case .success(let metadata):
                    self.document.fileSize = metadata.fileSize
                    self.document.pageCount = metadata.pageCount
                    self.document.author = metadata.author
                    self.document.creationDate = metadata.creationDate
                    self.document.documentHashPrefix = metadata.hashPrefix
                    self.document.phase = .open
                    self.addRecent(url)
                case .failure(let error):
                    self.document.phase = .error(error.localizedDescription)
                }
            }
        }
    }

    func closeDocument() {
        document = DocumentState()
    }

    // MARK: - Recent Documents

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
            let pages = document.pageCount.map { "\($0) pages" } ?? "Pages unknown"
            return "\(document.documentName) • \(pages) • \(document.fileSizeFormatted)"
        case .error:
            return "Document error"
        }
    }

    // MARK: - Placeholder Actions

    func inspectDocument() {
        guard document.phase == .open else { return }
        // Placeholder: will dispatch to fe_reader_core inspection pipeline.
    }

    func exportDocument() {
        guard document.phase == .open else { return }
        let panel = NSSavePanel()
        panel.allowedContentTypes = [.pdf]
        panel.nameFieldStringValue = document.documentName
        panel.begin { _ in }
    }

    func validateDocument() {
        guard document.phase == .open else { return }
        // Placeholder: will dispatch to fe_reader_core validation pipeline.
    }

    func redactDocument() {
        guard document.phase == .open else { return }
        // Gate: mutation pipeline required \u{2014} not yet available.
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
                .frame(minWidth: 420)

            if appState.showInspector {
                Divider()
                InspectorPanel()
                    .frame(width: 260)
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
        case .empty:
            return "doc"
        case .loading:
            return "hourglass"
        case .open:
            return "doc.richtext"
        case .error:
            return "exclamationmark.triangle"
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

// MARK: - Empty State

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

            Text("Drop a PDF file here or use File \u{2192} Open to begin.")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)

                Text("Cmd+O")
                .font(.caption)
                .foregroundColor(.secondary)
                .opacity(0.5)
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

// MARK: - Loading State

struct LoadingStateView: View {
    var body: some View {
        VStack(spacing: 16) {
            ProgressView()
                .scaleEffect(1.2)
                .accessibilityHidden(true)

            Text("Loading document\u{2026}")
                .font(.subheadline)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(AppColors.canvas)
        .accessibilityLabel("Loading document")
    }
}

// MARK: - Open Document State

struct OpenDocumentView: View {
    @EnvironmentObject private var appState: AppState

    var body: some View {
        VStack(spacing: 0) {
            VStack(spacing: 6) {
                Image(systemName: "doc.richtext")
                    .font(.title)
                    .foregroundColor(.accentColor)
                    .accessibilityHidden(true)

                Text(appState.document.documentName)
                    .font(.title2)
                    .fontWeight(.medium)
                    .lineLimit(1)
                    .truncationMode(.middle)

                Text(appState.document.fileSizeFormatted)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            .padding(.top, 32)
            .padding(.bottom, 24)

            Divider()
                .padding(.horizontal, 40)

            VStack(alignment: .leading, spacing: 12) {
                Label("Pages: \(appState.document.pageCount.map(String.init) ?? "\u{2014}")", systemImage: "number")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                Label("Author: \(appState.document.author ?? "\u{2014}")", systemImage: "person")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                Label("Created: \(formattedDate(appState.document.creationDate))", systemImage: "calendar")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }
            .padding(.top, 24)
            .accessibilityElement(children: .combine)

            Spacer()
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(AppColors.canvas)
    }
}

// MARK: - Error State

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
                Button("Dismiss") {
                    appState.document.phase = .empty
                }
                .controlSize(.regular)
                .keyboardShortcut(.escape)

                Button("Try Again") {
                    if let url = appState.document.documentURL {
                        appState.openDocument(at: url)
                    }
                }
                .controlSize(.regular)
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
                VStack(alignment: .leading, spacing: 20) {
                    InspectorSection(title: "Document Info") {
                        InfoRow(label: "Name",
                                value: appState.document.documentName.isEmpty
                                    ? "\u{2014}"
                                    : appState.document.documentName)
                        InfoRow(label: "Size",
                                value: appState.document.phase == .open
                                    ? appState.document.fileSizeFormatted
                                    : "\u{2014}")
                        InfoRow(label: "Pages",
                                value: appState.document.pageCount.map(String.init) ?? "\u{2014}")
                        InfoRow(label: "Hash",
                                value: appState.document.documentHashPrefix ?? "\u{2014}")
                        InfoRow(label: "Format",
                                value: appState.document.phase == .open
                                    ? "PDF" : "\u{2014}")
                    }

                    InspectorSection(title: "Actions") {
                        VStack(spacing: 8) {
                            ActionButton(title: "Inspect",
                                         icon: "magnifyingglass",
                                         shortcut: "\u{21e7}\u{2318}I") {
                                appState.inspectDocument()
                            }
                            .disabled(appState.document.phase != .open)

                            ActionButton(title: "Export\u{2026}",
                                         icon: "square.and.arrow.up",
                                         shortcut: "\u{21e7}\u{2318}E") {
                                appState.exportDocument()
                            }
                            .disabled(appState.document.phase != .open)

                            ActionButton(title: "Validate",
                                         icon: "checkmark.shield",
                                         shortcut: "\u{21e7}\u{2318}V") {
                                appState.validateDocument()
                            }
                            .disabled(appState.document.phase != .open)

                            Divider()

                            ActionButton(title: "Redact\u{2026}",
                                         icon: "eye.slash",
                                         shortcut: "") {
                                appState.redactDocument()
                            }
                            .disabled(true)
                            .accessibilityLabel("Redact document \u{2014} mutation pipeline not yet available")
                        }
                    }
                }
                .padding(16)
            }
        }
        .frame(minWidth: 200)
        .background(AppColors.panel)
        .accessibilityLabel("Inspector panel")
    }
}

private func formattedDate(_ date: Date?) -> String {
    guard let date else { return "\u{2014}" }
    let formatter = DateFormatter()
    formatter.dateStyle = .medium
    formatter.timeStyle = .none
    return formatter.string(from: date)
}

// MARK: - Inspector Subviews

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
        HStack {
            Text(label + ":")
                .font(.caption)
                .foregroundColor(.secondary)
                .frame(width: 56, alignment: .trailing)
            Text(value)
                .font(.caption)
                .foregroundColor(.primary)
                .lineLimit(1)
                .truncationMode(.middle)
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
                        .opacity(0.5)
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
            Button {
                appState.openDocumentPicker()
            } label: {
                Label("Open", systemImage: "folder")
            }
            .keyboardShortcut("O", modifiers: .command)
            .accessibilityLabel("Open document")
        }

        ToolbarItem(placement: .primaryAction) {
            Button {
                withAnimation {
                    appState.showInspector.toggle()
                }
            } label: {
                Label("Inspector", systemImage: "sidebar.trailing")
            }
            .accessibilityLabel(appState.showInspector
                               ? "Hide inspector panel"
                               : "Show inspector panel")
        }

        ToolbarTitleMenu {
            Button("Open\u{2026}") {
                appState.openDocumentPicker()
            }
            .keyboardShortcut("O", modifiers: .command)

            Button("Close Document") {
                appState.closeDocument()
            }
            .keyboardShortcut("W", modifiers: .command)
            .disabled(appState.document.phase == .empty)

            Divider()

            Toggle("Show Inspector", isOn: $appState.showInspector)
                .keyboardShortcut("i", modifiers: [.command, .option])
        }
    }
}

// MARK: - Open File Button (for CommandGroup reuse)

struct OpenFileButton: View {
    @ObservedObject var appState: AppState

    var body: some View {
        Button("Open\u{2026}") {
            appState.openDocumentPicker()
        }
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
                  resolvedURL.pathExtension.lowercased() == "pdf" else {
                return
            }

            DispatchQueue.main.async {
                self.appState.openDocument(at: resolvedURL)
            }
        }

        return true
    }

    func dropEntered(info: DropInfo) {}
    func dropExited(info: DropInfo) {}
    func dropUpdated(info: DropInfo) -> DropProposal? {
        DropProposal(operation: .copy)
    }
}

// MARK: - Verification Capture (used by build_and_run.sh --verify)

/// AppKit delegate used for verification screenshot capture and window-level behavior.
private final class AppDelegate: NSObject, NSApplicationDelegate {
    func applicationDidFinishLaunching(_ notification: Notification) {
        scheduleCaptureIfRequested()
    }
}

/// Schedules a verification screenshot capture if the marker file exists.
private func scheduleCaptureIfRequested() {
    guard FileManager.default.fileExists(atPath: captureMarkerPath) else { return }

    DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
        captureScreenshot()
    }
}

/// Captures the app window content to a PNG file for verification.
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

    let url = URL(fileURLWithPath: captureOutputPath)
    do {
        try data.write(to: url, options: .atomic)
        try? FileManager.default.removeItem(atPath: captureMarkerPath)
    } catch {
        // Non-fatal: verification capture is best-effort for CI/evidence.
        print("[FeReader] verification capture failed: \(error.localizedDescription)")
    }
}
