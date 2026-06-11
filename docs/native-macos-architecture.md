# Native macOS Shell Architecture

> **Status**: Living document (updated per Track AQ phases)  
> **Framework ADR**: [0011-native-macos-shell-framework](./adr/0011-native-macos-shell-framework.json)  
> **Design Source**: Track AP Figma wireframe, UX brief, interaction model, design tokens  
> **Design Tokens**: [native-macos-design-tokens.md](native-macos-design-tokens.md)  
> **Interaction Model**: [native-macos-interaction-model.md](native-macos-interaction-model.md)  
> **Handoff Notes**: [native-macos-handoff-notes.md](native-macos-handoff-notes.md)  

---

## 1. View Hierarchy

SwiftUI @main with NSApplicationDelegateAdaptor for narrow AppKit interop. 
Three-pane SwiftUI shell with toolbar, sidebar, document canvas, inspector panel, and status bar.

```
App (SwiftUI @main - FeReaderApp)
|
+-- NSApplicationDelegateAdaptor (AppDelegate)
|   +-- applicationDidFinishLaunching (config)
|   +-- applicationShouldTerminateAfterLastWindowClosed -> true
|   +-- application:openFile: (Finder / file association)
|   +-- application:openURLs: (URL-based open)
|
+-- WindowGroup (SwiftUI scene)
    +-- ShellView
        |
        +-- ToolbarArea (.toolbar)
        |   +-- LeadingGroup
        |   |   +-- OpenButton (doc.badge.plus)
        |   |   +-- OpenRecentButton (clock.arrow.circlepath)
        |   |   +-- InspectButton (magnifyingglass)
        |   |   +-- ExportButton (square.and.arrow.up)
        |   |   +-- ValidateButton (checkmark.seal)
        |   +-- TrailingGroup
        |       +-- RedactButton (eye.slash, gated)
        |       +-- SidebarToggle (sidebar.left)
        |       +-- InspectorToggle (sidebar.right)
        |
        +-- Explicit HStack panes
        |   +-- Sidebar (~220pt, collapsible)
        |   |   +-- RecentDocumentsList
        |   |   +-- PinnedDocumentsList
        |   |   +-- ContractsList
        |   |
        |   +-- HSplitView (DetailArea)
        |       +-- DocumentCanvas (flexible)
        |       |   +-- EmptyState (drop zone)
        |       |   +-- LoadingState (progress)
        |       |   +-- OpenState (PDF surface)
        |       |   +-- ErrorState (error + recovery)
        |       |
        |       +-- InspectorPanel (~260pt, hideable)
        |           +-- DocumentInfoSection
        |           +-- ValidationSection
        |           +-- SelectionSection
        |           +-- ActionsSection
        |           +-- AuditReceiptSection
        |
        +-- MenuCommands (CommandsBuilder)
            +-- File | Edit | View | Document | Window | Help menus
            +-- Each with keyboardShortcut declarations
```
### 1.1 Component Responsibilities

| Region | Responsibility | Fills When |
|---|---|---|
| **ShellView** | Holds state objects, binds regions, dispatches commands | Always |
| **Sidebar** | Recent/pinned/contracts lists, drives canvas via selection | Always |
| **DocumentCanvas** | Empty/loading/open/error states, drag-drop surface | Always |
| **InspectorPanel** | Metadata, validation, selection, actions, audit receipts | Always (may be hidden) |
| **ToolbarArea** | Open, inspect, export, validate, redact commands | Always |

### 1.2 File Mapping

| File | Role |
|---|---|
| FeReaderApp.swift | @main, AppDelegate, WindowGroup |
| ShellView.swift | Root explicit three-pane layout |
| Models/DocumentState.swift | DocumentState enum, DocumentRef, LoadError |
| Models/SelectionState.swift | Sidebar, page, content selection |
| Models/ShellSettings.swift | Shell mode, sidebar/inspector visibility |
| Adapter/ShellAdapter.swift | Protocol definition |
| Adapter/CLIAdapter.swift | Production adapter (CLI subprocess) |
| Adapter/ShellAdapterStub.swift | Stub adapter for tests/previews |
| Views/SidebarView.swift | Sidebar list + context menus |
| Views/DocumentCanvas.swift | State-driven canvas |
| Views/InspectorPanel.swift | Metadata/validation/actions panel |
| Views/ToolbarContent.swift | Toolbar button definitions |
| Views/EmptyStateView.swift | Drop zone + instructions |
| Views/LoadingStateView.swift | Progress indicator |
| Views/ErrorStateView.swift | Error display + recovery actions |
| Commands/MenuCommands.swift | CommandsBuilder with all menus |

---

## 2. State Model Design

Observable classes as single source of truth. Each domain is @Observable (macOS 14+). Views observe only needed slices.

### 2.1 State Classes

```
AppModel (@Environment)
+-- document: DocumentState
+-- selection: SelectionState
+-- settings: ShellSettings
+-- recent: RecentDocuments
+-- auditLog: AuditLog
+-- adapter: ShellAdapter
```

### 2.2 DocumentState

```swift
enum DocumentState: Equatable {
    case empty
    case loading(url: URL, startedAt: Date)
    case open(DocumentRef)
    case error(LoadError)
}

struct DocumentRef: Equatable, Identifiable {
    let id: UUID
    let url: URL
    let filename: String
    let path: String
    let openedAt: Date
    let fileSize: Int64
    let metadata: DocumentMetadata?
    let validationResult: ValidationResult?
}

struct DocumentMetadata: Equatable {
    let pageCount: Int
    let pdfVersion: String?
    let title: String?
    let author: String?
    let creationDate: Date?
    let modificationDate: Date?
    let encrypted: Bool
    let pdfaCompliant: Bool?
    let hashSHA256: String
}

enum LoadError: Equatable {
    case fileNotFound(url: URL)
    case notAValidPDF(url: URL, detail: String)
    case encryptionNotSupported(url: URL)
    case corrupted(url: URL, detail: String)
    case adapterUnavailable(reason: String)
    case unknown(url: URL, detail: String)
}
```

### 2.3 SelectionState

```swift
@Observable
final class SelectionState {
    var sidebarSection: SidebarSection = .recent
    var sidebarDocumentId: UUID? = nil
    var pageSelection: PageSelection = .none
    var contentSelection: ContentSelection = .none
}

enum SidebarSection: String, CaseIterable, Identifiable {
    case recent; case pinned; case contracts
    var id: String { rawValue }
}

enum PageSelection: Equatable {
    case none; case single(page: Int)
    case range(start: Int, end: Int); case all
}

enum ContentSelection: Equatable {
    case none
    case text(String, page: Int, bounds: CGRect?)
    case region(page: Int, bounds: CGRect)
}
```

### 2.4 ShellSettings

```swift
@Observable
final class ShellSettings {
    var windowTitle: String = "Fe Reader"
    var sidebarVisible: Bool = true
    var sidebarWidth: CGFloat = 220
    var inspectorVisible: Bool = true
    var inspectorWidth: CGFloat = 260
    var shellMode: ShellMode = .normal
}

enum ShellMode: Equatable {
    case normal; case inspecting
    case exporting(ExportFormat)
    case redacting(RedactPhase)
}

enum RedactPhase: Equatable {
    case planning; case reviewing; case applying
    case completed(AuditReceiptStub)
}

enum ExportFormat: String, CaseIterable {
    case pdf; case text; case images
}
```
### 2.5 RecentDocuments

```swift
@Observable
final class RecentDocuments {
    private(set) var items: [RecentDocumentItem] = []

    var pinned: [RecentDocumentItem] { items.filter(\.isPinned) }
    var unpinned: [RecentDocumentItem] { items.filter { !$0.isPinned } }

    func add(_ url: URL) { ... }
    func remove(id: UUID) { ... }
    func togglePin(id: UUID) { ... }
    func clear() { ... }
}

struct RecentDocumentItem: Identifiable, Equatable, Codable {
    let id: UUID
    let url: URL
    let filename: String
    let lastOpenedAt: Date
    var openCount: Int
    var isPinned: Bool
}
```

### 2.6 AuditLog

```swift
@Observable
final class AuditLog {
    private(set) var receipts: [AuditReceiptStub] = []
    func append(_ receipt: AuditReceiptStub) { ... }
}

struct AuditReceiptStub: Identifiable, Equatable {
    let id: String
    let timestamp: Date
    let operation: String
    let documentHash: String
    let patchPlanId: String
    let outcome: String
}
```

### 2.7 AppModel (Root)

```swift
@Observable
final class AppModel {
    var document: DocumentState = .empty
    var selection = SelectionState()
    var settings = ShellSettings()
    var recent = RecentDocuments()
    var auditLog = AuditLog()
    var adapter: ShellAdapter

    init(adapter: ShellAdapter) {
        self.adapter = adapter
        self.recent = RecentDocuments.load()
    }

    @MainActor
    func openDocument(url: URL) async {
        document = .loading(url: url, startedAt: Date())
        do {
            let ref = try await adapter.openDocument(url: url)
            document = .open(ref)
            recent.add(url)
            settings.windowTitle = "Fe Reader - \(ref.filename)"
        } catch {
            document = .error(mapError(error, url: url))
        }
    }

    @MainActor
    func closeDocument() {
        document = .empty
        selection = SelectionState()
        settings.windowTitle = "Fe Reader"
    }

    @MainActor
    func validateDocument() async {
        guard case .open(var ref) = document else { return }
        do {
            let result = try await adapter.validateDocument(url: ref.url)
            ref.validationResult = result
            document = .open(ref)
        } catch {
            document = .error(.unknown(url: ref.url, detail: error.localizedDescription))
        }
    }

    @MainActor
    func initiateRedact() {
        guard case .open = document,
              case .region = selection.contentSelection else { return }
        settings.shellMode = .redacting(.planning)
    }

    @MainActor
    func confirmRedact() async {
        // adapter.applyPatch(document, patchPlanID, approvalToken)
        // auditLog.append(receipt)
        // shellMode = .redacting(.completed(receipt))
    }

    @MainActor
    func beginExport(format: ExportFormat) {
        settings.shellMode = .exporting(format)
    }

    @MainActor
    func executeExport(url: URL, format: ExportFormat) async {
        guard case .open(let ref) = document else { return }
        do {
            try await adapter.exportDocument(ref.url, to: url, format: format)
            settings.shellMode = .normal
        } catch {
            document = .error(.unknown(url: ref.url, detail: error.localizedDescription))
        }
    }
}
```

### 2.8 State Flow Diagram

```
                       .empty (launch, Cmd+W, dismiss error)
                          | Cmd+O / drag-drop / recent
                          v
                       .loading (adapter.openDocument() in flight)
                          |
                     +----+----+
                     |         |
                     v         v
                  .open     .error (adapter threw / invalid file)
                     |         |
                     |         +-- Retry -> .loading
                     |         +-- Dismiss -> .empty
                     |
                     +-- Validate -> .open (updated validationResult)
                     +-- Select -> selection updates (.open)
                     +-- Redact -> mode: .redacting
                     |   +-- Confirm -> Apply -> .open (post-mutation)
                     |   +-- Cancel -> mode: .normal
                     +-- Export -> mode: .exporting -> .open / .error
```

---

## 3. Adapter Boundary Design

ShellAdapter protocol is the narrow boundary between shell and core. No SwiftUI view or state class may call core mutation logic directly.

### 3.1 ShellAdapter Protocol

```swift
protocol ShellAdapter: AnyObject, Sendable {
    // Read Operations
    func openDocument(url: URL) async throws -> DocumentRef
    func validateDocument(url: URL) async throws -> ValidationResult

    func inspectSelection(
        url: URL, pageRange: Range<Int>, selection: ContentSelection
    ) async throws -> InspectionResult

    // Export (read-only from core perspective)
    func exportDocument(_: URL, to: URL, format: ExportFormat) async throws

    // Mutation Pipeline (all writes gated)
    func planRedaction(document: DocumentRef, selection: ContentSelection)
        async throws -> PatchPlanStub

    func applyPatch(document: DocumentRef, patchPlanID: String, approvalToken: String)
        async throws -> AuditReceiptStub

    // Utility
    func canOpen(_ url: URL) -> Bool
}

struct PatchPlanStub: Equatable {
    let id: String
    let documentHash: String
    let operations: [PatchOperationStub]
    let estimatedImpact: String
}

struct PatchOperationStub: Equatable {
    let type: String
    let page: Int
    let bounds: CGRect
    let description: String
}

struct ValidationResult: Equatable {
    let isValid: Bool
    let checks: [ValidationCheck]
}

struct ValidationCheck: Equatable, Identifiable {
    let id: String
    let name: String
    let status: ValidationStatus
    let detail: String?
}

enum ValidationStatus: Equatable {
    case passed; case warning(String)
    case failed(String); case skipped(reason: String)
}

struct InspectionResult: Equatable {
    let text: String?
    let metadata: [String: String]
    let warnings: [String]
}
```

### 3.2 Adapter Architecture

```
ShellView (SwiftUI)
    | calls protocol
    v
ShellAdapter (native/macos/Adapter)
    |
    +-- CLIAdapter (fe-reader CLI subprocess)
    +-- FFIBridgeAdapter (future: UniFFI)
    +-- ShellAdapterStub (previews and tests)
            |
            v
    fe_reader_core (pure Rust, no UI deps)
```

### 3.3 CLIAdapter (Production)

```swift
final class CLIAdapter: ShellAdapter {
    private let cliPath: String

    init(cliPath: String = "/usr/local/bin/fe-reader") {
        self.cliPath = cliPath
    }

    func openDocument(url: URL) async throws -> DocumentRef {
        // Spawn: fe-reader document inspect <url> --format json
        // Parse JSON into DocumentRef
    }

    func validateDocument(url: URL) async throws -> ValidationResult {
        // Spawn: fe-reader document validate <url> --format json
    }

    func exportDocument(_ src: URL, to dest: URL, format: ExportFormat) async throws {
        // Spawn: fe-reader document export <src> --output <dest> --format <fmt>
    }

    func planRedaction(document: DocumentRef, selection: ContentSelection)
        async throws -> PatchPlanStub {
        // Read-only: fe-reader document plan-redact <url> ...
    }

    func applyPatch(document: DocumentRef, patchPlanID: String, approvalToken: String)
        async throws -> AuditReceiptStub {
        // Mutation gated: fe-reader document apply-patch ...
    }

    func canOpen(_ url: URL) -> Bool {
        url.pathExtension.lowercased() == "pdf"
    }

    func inspectSelection(url: URL, pageRange: Range<Int>, selection: ContentSelection)
        async throws -> InspectionResult {
        // fe-reader document inspect <url> --page <n> ...
    }
}
```

### 3.4 ShellAdapterStub (Previews & Tests)

```swift
final class ShellAdapterStub: ShellAdapter {
    // Returns synthetic DocumentRef with fake metadata
    // Validation always returns isValid: true
    // planRedaction returns a minimal PatchPlanStub
    // No real file I/O occurs
}
```

### 3.5 Core Isolation Contract

| Rule | Enforcement |
|---|---|
| No SwiftUI view imports core Rust types | Import scan |
| All core access through ShellAdapter | Architecture review |
| Adapter runs on background; results to @MainActor | async throws + @MainActor |
| Mutation always through full pipeline | planRedaction -> review -> applyPatch |
| Automation read-only by default | applyPatch requires approval token |

---

## 4. Menu/Toolbar Command Structure

### 4.1 Menu Bar (SwiftUI CommandsBuilder)

```swift
struct MenuCommands: Commands {
    @Environment(AppModel.self) private var model

    var body: some Commands {
        CommandMenu("File") {
            OpenDocumentCommand(model: model)
                .keyboardShortcut("o", modifiers: .command)
            OpenRecentMenu(model: model)
            Divider()
            CloseDocumentCommand(model: model)
                .keyboardShortcut("w", modifiers: .command)
            Divider()
            ExportCommand(model: model)
                .keyboardShortcut("e", modifiers: .command)
        }

        CommandMenu("View") {
            ToggleSidebarCommand()
                .keyboardShortcut("l", modifiers: [.command, .shift])
            ToggleInspectorCommand(model: model)
                .keyboardShortcut("i", modifiers: [.command, .shift])
            Divider()
            Menu("Zoom") {
                ZoomInCommand().keyboardShortcut("+", modifiers: .command)
                ZoomOutCommand().keyboardShortcut("-", modifiers: .command)
                Divider()
                FitToWidthCommand().keyboardShortcut("9", modifiers: .command)
                ActualSizeCommand().keyboardShortcut("8", modifiers: .command)
            }
        }

        CommandMenu("Document") {
            InspectCommand(model: model)
                .keyboardShortcut("i", modifiers: .command)
            ValidateCommand(model: model)
                .keyboardShortcut("v", modifiers: [.command, .shift])
            Divider()
            RedactCommand(model: model)
                .keyboardShortcut("r", modifiers: [.command, .shift])
                // Disabled by default
        }

        CommandMenu("Help") {
            Link("Fe Reader Documentation",
                 destination: URL(string: "https://edithatogo.github.io/fe-reader/")!)
                .keyboardShortcut("?", modifiers: .command)
        }
    }
}
```

### 4.2 Command Definitions

| Menu | Label | Shortcut | Action | Enabled When |
|---|---|---|---|---|
| File | Open... | Cmd+O | NSOpenPanel -> model.openDocument(url:) | Always |
| File | Open Recent | Cmd+Shift+O | Submenu of recent items | Recent list non-empty |
| File | Close | Cmd+W | model.closeDocument() | .open state |
| File | Export... | Cmd+E | model.beginExport() -> NSSavePanel | .open state |
| View | Toggle Sidebar | Cmd+Shift+L | settings.sidebarVisible.toggle() | Always |
| View | Toggle Inspector | Cmd+Shift+I | settings.inspectorVisible.toggle() | Always |
| View | Zoom In | Cmd+= | Canvas zoom +0.25 | .open state |
| View | Zoom Out | Cmd+- | Canvas zoom -0.25 | .open state |
| View | Fit to Width | Cmd+9 | Canvas fit to width | .open state |
| View | Actual Size | Cmd+8 | Canvas 100% zoom | .open state |
| Document | Inspect | Cmd+I | Focus inspector, scroll to info | .open state |
| Document | Validate | Cmd+Shift+V | model.validateDocument() | .open state |
| Document | Redact | Cmd+Shift+R | model.initiateRedact() | .open + selection + policy |
| Help | Documentation | Cmd+? | Open docs URL | Always |

### 4.3 Toolbar Layout

```swift
struct ShellToolbarContent: ToolbarContent {
    @Environment(AppModel.self) private var model
    @State private var showOpenPanel = false

    var body: some ToolbarContent {
        // Leading group
        ToolbarItemGroup(placement: .primaryAction) {
            Button("Open", systemImage: "doc.badge.plus") {
                showOpenPanel = true
            }
            .help("Open PDF (Cmd+O)")

            Button("Open Recent", systemImage: "clock.arrow.circlepath") {
                // Show recent popover
            }
            .help("Open recent (Cmd+Shift+O)")
            .disabled(model.recent.items.isEmpty)

            Divider()

            Button("Inspect", systemImage: "magnifyingglass") {
                model.settings.shellMode = .inspecting
            }
            .help("Inspect metadata (Cmd+I)")
            .disabled(!model.document.isOpen)

            Button("Export", systemImage: "square.and.arrow.up") {
                model.beginExport(format: .pdf)
            }
            .help("Export (Cmd+E)")
            .disabled(!model.document.isOpen)

            Button("Validate", systemImage: "checkmark.seal") {
                Task { await model.validateDocument() }
            }
            .help("Validate (Cmd+Shift+V)")
            .disabled(!model.document.isOpen)
        }

        // Trailing group
        ToolbarItemGroup(placement: .automatic) {
            Spacer()

            Button("Redact", systemImage: "eye.slash") {
                model.initiateRedact()
            }
            .help("Redact (Cmd+Shift+R, gated)")
            .disabled(!model.canRedact)

            Button("Sidebar", systemImage: "sidebar.left") {
                model.settings.sidebarVisible.toggle()
            }
            .help("Toggle sidebar (Cmd+Shift+L)")

            Button("Inspector", systemImage: "sidebar.right") {
                model.settings.inspectorVisible.toggle()
            }
            .help("Toggle inspector (Cmd+Shift+I)")
        }
    }
}
```

### 4.4 Mutation Gating (Redact)

Redact follows the strict pipeline:

```
User selects content -> Cmd+Shift+R
    |
    v
1. planRedaction -> adapter.planRedaction() (read-only)
   | PatchPlanStub returned
   v
2. Review Plan -> UI shows plan in inspector + banner
   | User confirms or cancels
   v
3. applyPatch -> adapter.applyPatch() (write gated)
   | AuditReceiptStub returned
   v
4. Receipt -> shown in inspector, added to audit log
```

The Redact button is **disabled by default**. Enabled only when ALL of:
1. document == .open
2. selection.contentSelection != .none (selection made)
3. Policy evaluation returns approved

---

## 5. Window and Navigation Design

### 5.1 Window Configuration

Managed via NSApplicationDelegateAdaptor in FeReaderApp.swift.

```swift
@main
struct FeReaderApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    @State private var model = AppModel(adapter: CLIAdapter())

    var body: some Scene {
        WindowGroup {
            ShellView()
                .environment(model)
        }
        .windowResizability(.contentMinSize)
        .defaultSize(width: 1200, height: 800)
        .windowResizabilityContentMinSize(width: 800, height: 600)
        .commands { MenuCommands() }
    }
}

final class AppDelegate: NSObject, NSApplicationDelegate {
    func applicationDidFinishLaunching(_ notification: Notification) {
        NSDocumentController.shared.autosavingDelay = 60
    }

    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        true
    }

    func application(_ application: NSApplication, openFile filename: String) -> Bool {
        Task { @MainActor in
            // await model.openDocument(url: URL(fileURLWithPath: filename))
        }
        return true
    }

    func application(_ sender: NSApplication, openURLs urls: [URL]) {
        guard let url = urls.first else { return }
        Task { @MainActor in
            // await model.openDocument(url: url)
        }
    }
}
```

### 5.2 Window Metrics

| Property | Value | Source |
|---|---|---|
| Default size | 1200 x 800 | Track AP (breathing room at 1120x760) |
| Minimum size | 800 x 600 | Interaction model |
| Sidebar default | 220pt | Balance for 3-column at 1200px |
| Sidebar range | 180-360pt | Interaction model |
| Inspector default | 260pt | Balance for 3-column at 1200px |
| Inspector range | 220-400pt | Interaction model |
| Sidebar collapse | <800px window width | Icon-only |
| Inspector hide | <600px window width | Overlay on demand |
| Title (empty) | Fe Reader | Interaction model |
| Title (open) | Fe Reader - filename.pdf | Interaction model |

### 5.3 Three-Pane Layout

```swift
struct ShellView: View {
    @Environment(AppModel.self) private var model

    var body: some View {
        HStack(
            columnVisibility: $sidebarVisibility,
            sidebar: {
                SidebarView()
                    .navigationSplitViewColumnWidth(min: 180, ideal: 220, max: 360)
                    .frame(minWidth: 180)
            },
            detail: {
                HSplitView {
                    DocumentCanvas()
                        .layoutPriority(1)

                    if model.settings.inspectorVisible {
                        InspectorPanel()
                            .navigationSplitViewColumnWidth(min: 220, ideal: 260, max: 400)
                            .frame(minWidth: 220)
                    }
                }
            }
        )
        .navigationSplitViewStyle(.prominentDetail)
        .toolbar { ShellToolbarContent() }
        .frame(minWidth: 800, minHeight: 600)
    }

    private var sidebarVisibility: Binding<Bool> {
        Binding(
            get: { model.settings.sidebarVisible ? .all : .detailOnly },
            set: { model.settings.sidebarVisible = $0 != .detailOnly }
        )
    }
}
```

### 5.4 Window Management Rules

| Behavior | Implementation |
|---|---|
| Single window per scene | WindowGroup manages one window by default |
| New window (Cmd+N) | WindowGroup creates independent AppModel instance |
| Close last window quits | applicationShouldTerminateAfterLastWindowClosed -> true |
| File association | application(_:openFile:) in AppDelegate |
| Drag to Dock icon | application(_:openURLs:) in AppDelegate |
| Activation policy | .regular (standard app, menu bar + Dock) |

### 5.5 Responsive Breakpoints

```
switch windowWidth {
case 600..<800:
    // Inspector hidden, sidebar icon-only
    // Cmd+Shift+I shows inspector as overlay
case 800..<1120:
    // Sidebar icon-only, inspector visible
case 1120...:
    // Full layout: sidebar (labels) + canvas + inspector
}
```

---

## 6. File Handling Flow

### 6.1 Open via Menu / Cmd+O

```
User: Cmd+O or File -> Open
    |
    v
NSOpenPanel (via .fileImporter or AppKit interop)
    | User selects file
    v
URL validation (file exists, .pdf extension, adapter.canOpen)
    | Valid
    v
model.openDocument(url: selectedURL)
    |
+---+---+
|       |
v       v
.loading .error
    |
    v
 .open
```

Implementation (preferred):
```swift
.fileImporter(
    isPresented: $showOpenPanel,
    allowedContentTypes: [.pdf],
    allowsMultipleSelection: false
) { result in
    guard case .success(let urls) = result, let url = urls.first else { return }
    Task { await model.openDocument(url: url) }
}
```

### 6.2 Open Recent

Recent items stored in UserDefaults as JSON. Maximum 20 items (pinned excluded from cap).

```swift
extension RecentDocuments {
    private static let storageKey = "FeReaderRecentDocuments"

    static func load() -> RecentDocuments {
        guard let data = UserDefaults.standard.data(forKey: storageKey),
              let items = try? JSONDecoder().decode([RecentDocumentItem].self, from: data)
        else { return RecentDocuments() }
        return RecentDocuments(items: items)
    }

    func save() {
        guard let data = try? JSONEncoder().encode(items) else { return }
        UserDefaults.standard.set(data, forKey: Self.storageKey)
    }

    func add(_ url: URL) {
        // Deduplicate by URL, move to front, cap at 20
        // Save to UserDefaults
    }
}
```

### 6.3 Drag-Drop onto Document Canvas

```swift
struct DocumentCanvas: View {
    @Environment(AppModel.self) private var model
    @State private var isDropTargeted = false

    var body: some View {
        Group {
            switch model.document {
            case .empty:
                EmptyDropZone(isTargeted: $isDropTargeted)
            case .loading(let url, _):
                LoadingStateView(filename: url.lastPathComponent)
            case .open(let ref):
                DocumentSurfaceView(ref: ref)
            case .error(let error):
                ErrorStateView(error: error)
            }
        }
        .onDrop(of: [.fileURL], isTargeted: $isDropTargeted) { providers in
            handleDrop(providers: providers)
        }
    }

    private func handleDrop(providers: [NSItemProvider]) -> Bool {
        guard let provider = providers.first else { return false }
        provider.loadItem(forTypeIdentifier: UTType.fileURL.identifier, options: nil) { item, error in
            guard let data = item as? Data,
                  let url = URL(dataRepresentation: data, relativeTo: nil) else { return }
            Task { @MainActor in
                if case .open = model.document {
                    // Show confirmation dialog before replacing
                }
                await model.openDocument(url: url)
            }
        }
        return true
    }
}
```

Drop flow:
```
Drag PDF from Finder -> DocumentCanvas
    |
    v
.onDrop(of: [.fileURL]) -> NSItemProvider -> URL
    |
    v
Validate: file exists + adapter.canOpen(url)
    |
+---+---+
|       |
v       v
Open   Show rejection toast (non-PDF or invalid)
(if doc open, ask confirmation)
```

### 6.4 Export Flow

```
User: File -> Export / toolbar Export button
    |
    v
model.beginExport(format: .pdf) -> mode: .exporting
    |
    v
NSSavePanel (via .fileExporter or AppKit interop)
    | User selects destination
    v
model.executeExport(url: dest, format: format)
    -> adapter.exportDocument()
    |
+---+---+
|       |
v       v
.normal .error
```

### 6.5 File Association (Finder)

Info.plist in app bundle:

```xml
<key>CFBundleDocumentTypes</key>
<array>
    <dict>
        <key>CFBundleTypeName</key>
        <string>PDF Document</string>
        <key>CFBundleTypeRole</key>
        <string>Viewer</string>
        <key>LSHandlerRank</key>
        <string>Alternate</string>
        <key>LSItemContentTypes</key>
        <array>
            <string>com.adobe.pdf</string>
        </array>
    </dict>
</array>
```

### 6.6 NSDocumentController (Alternative Recent Path)

```swift
// AppDelegate
func applicationDidFinishLaunching(_ notification: Notification) {
    for item in model.recent.items {
        NSDocumentController.shared.noteNewRecentDocumentURL(item.url)
    }
}
```

Note: Primary recent UI is SwiftUI-driven. NSDocumentController populates the system-level dock menu.

---

## 7. Project File Structure

```
native/macos/
+-- FeReaderApp.swift              # @main, WindowGroup, AppDelegate
+-- ShellView.swift                # Root explicit three-pane layout
|
+-- Models/
|   +-- AppModel.swift             # Root observable, all state + actions
|   +-- DocumentState.swift        # DocumentState enum, DocumentRef, LoadError
|   +-- SelectionState.swift       # Sidebar, page, content selection
|   +-- ShellSettings.swift        # Shell mode, sidebar/inspector prefs
|
+-- Adapter/
|   +-- ShellAdapter.swift         # Protocol definition
|   +-- CLIAdapter.swift           # Production: CLI subprocess
|   +-- ShellAdapterStub.swift     # Stub for previews/tests
|
+-- Views/
|   +-- SidebarView.swift          # Sidebar list + context menus
|   +-- DocumentCanvas.swift       # State-driven canvas with state switching
|   +-- InspectorPanel.swift       # Metadata/validation/actions panel
|   +-- ToolbarContent.swift       # Toolbar button groups
|   +-- EmptyStateView.swift       # Drop zone + instructions
|   +-- LoadingStateView.swift     # Progress indicator
|   +-- ErrorStateView.swift       # Error display + recovery
|   +-- StatusBarView.swift        # Bottom status bar (future)
|
+-- Commands/
    +-- MenuCommands.swift         # CommandsBuilder for all menus
    +-- OpenDocumentCommand.swift  # File -> Open
    +-- OpenRecentMenu.swift       # File -> Open Recent submenu
    +-- CloseDocumentCommand.swift # File -> Close
    +-- ExportCommand.swift        # File -> Export
    +-- ToggleSidebarCommand.swift # View -> Toggle Sidebar
    +-- ToggleInspectorCommand.swift # View -> Toggle Inspector
    +-- InspectCommand.swift       # Document -> Inspect
    +-- ValidateCommand.swift      # Document -> Validate
    +-- RedactCommand.swift        # Document -> Redact (gated)
```

---

## 8. Design Decisions & Deviations

### 8.1 Deviations from Figma / Design Tokens

| Deviation | Rationale | Status |
|---|---|---|
| Sidebar default 220pt vs 240pt | Three-column balance at 1200px window | Intentional |
| Inspector default 260pt vs 280pt | Three-column balance at 1200px window | Intentional |
| Minimum window 800x600 vs 800x500 | Better usability for 3-column layout | Intentional |
| Sidebar: recent/pinned/contracts vs placeholder sections | Replaces placeholder Library/Recent/Contracts/Release | Planned AQ2 |

### 8.2 Open Questions (from Track AP Handoff)

| Question | Resolution |
|---|---|
| Sidebar content model | Flat recent + pinned (replacing placeholder sections) |
| Document canvas impl | SwiftUI NSViewRepresentable for future PDF rendering adapter |
| Inspector tabs | Expandable accordion sections |
| Recent files persistence | UserDefaults with JSON encoding |
| Safe-open stub | Build error UI with stub recovery; adapter catches |
| Validation adapter API | Defined in ShellAdapter.validateDocument() |
| Mutation gating UX | Inline inspector banner + confirmation button |

---

## 9. Track AQ Phase Mapping

| AQ Phase | Architecture Document Sections |
|---|---|
| AQ1: Shell architecture and view model | SS1 View Hierarchy, SS2 State Model, SS3 Adapter Boundary, SS7 File Structure |
| AQ2: Document and command wiring | SS4 Command Structure, SS6 File Handling Flow |
| AQ3: Native finish and parity pass | SS5 Window and Navigation, SS8 Deviations |

---

## 10. Testing Strategy

| Layer | Test Approach |
|---|---|
| State model (AppModel) | Unit tests with ShellAdapterStub - verify state transitions, errors |
| Adapter boundary | Unit tests on CLIAdapter (mock process) + ShellAdapterStub |
| SwiftUI views | Preview provider snapshots with ShellAdapterStub + AppModel |
| Menu commands | Unit tests on command view models |
| File handling | Smoke test: launch, Cmd+O, drag-drop, Open Recent |
| Window metrics | Visual regression: window size constraints, breakpoints |
| Mutation pipeline | Integration test: plan -> review -> apply -> receipt on stub adapter |
