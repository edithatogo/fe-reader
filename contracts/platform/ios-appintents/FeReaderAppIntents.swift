// Contract sketch for iOS/iPadOS App Intents. Implementation belongs in native shell.

import AppIntents

struct FeOpenDocumentIntent: AppIntent {
    static var title: LocalizedStringResource = "Open Document in Fe Reader"
    @Parameter(title: "Document") var document: IntentFile

    func perform() async throws -> some IntentResult {
        // Create FeOperationIntent with source IosAppIntent and operation OpenDocument.
        return .result()
    }
}

struct FeExtractPageTextIntent: AppIntent {
    static var title: LocalizedStringResource = "Extract PDF Page Text"
    @Parameter(title: "Document ID") var documentId: String
    @Parameter(title: "Page Index") var pageIndex: Int

    func perform() async throws -> some IntentResult & ReturnsValue<String> {
        // Read-only operation. Return JSON text spans.
        return .result(value: "{}")
    }
}

struct FePlanWorkflowIntent: AppIntent {
    static var title: LocalizedStringResource = "Plan Fe Reader Workflow"
    @Parameter(title: "Document ID") var documentId: String
    @Parameter(title: "Workflow ID") var workflowId: String
    @Parameter(title: "Parameters JSON") var parametersJson: String

    func perform() async throws -> some IntentResult & ReturnsValue<String> {
        // Return PatchPlan JSON. Do not mutate without patch_plan_id, document_hash_match,
        // policy_allow_rule and approval_token in a native approved flow.
        return .result(value: "{}")
    }
}
