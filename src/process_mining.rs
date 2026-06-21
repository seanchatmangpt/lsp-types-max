//! XES (eXtensible Event Stream) compatible process mining types for LSP sessions.
//! In process mining terms: LspEventLog = event log, LspTrace = case, LspEvent = event.
//!
//! Inspired by Dr. Wil van der Aalst's process mining work (XES standard, event logs, traces, cases).

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Direction of an LSP message on the wire.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageDirection {
    ClientToServer,
    ServerToClient,
}

/// The JSON-RPC message kind.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageKind {
    Request,
    Response,
    Notification,
}

/// The LSP activity (method) that an event represents.
///
/// Variants map 1-to-1 with the LSP method strings defined in the specification.
/// `Custom` is a catch-all for non-standard or extension methods.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LspActivity {
    // ── Lifecycle ────────────────────────────────────────────────────────────
    Initialize,
    Initialized,
    Shutdown,
    Exit,
    CancelRequest,
    Progress,

    // ── Window ───────────────────────────────────────────────────────────────
    ShowMessage,
    ShowMessageRequest,
    LogMessage,
    ShowDocument,
    WorkDoneProgressCreate,
    WorkDoneProgressCancel,

    // ── Telemetry ────────────────────────────────────────────────────────────
    TelemetryEvent,

    // ── Client ───────────────────────────────────────────────────────────────
    RegisterCapability,
    UnregisterCapability,

    // ── Text Document Sync ───────────────────────────────────────────────────
    TextDocumentDidOpen,
    TextDocumentDidChange,
    TextDocumentWillSave,
    TextDocumentWillSaveWaitUntil,
    TextDocumentDidSave,
    TextDocumentDidClose,

    // ── Language Features ────────────────────────────────────────────────────
    TextDocumentCompletion,
    CompletionItemResolve,
    TextDocumentHover,
    TextDocumentSignatureHelp,
    TextDocumentDeclaration,
    TextDocumentDefinition,
    TextDocumentTypeDefinition,
    TextDocumentImplementation,
    TextDocumentReferences,
    TextDocumentDocumentHighlight,
    TextDocumentDocumentSymbol,
    TextDocumentCodeAction,
    CodeActionResolve,
    TextDocumentCodeLens,
    CodeLensResolve,
    TextDocumentDocumentLink,
    DocumentLinkResolve,
    TextDocumentFormatting,
    TextDocumentRangeFormatting,
    TextDocumentRangesFormatting,
    TextDocumentOnTypeFormatting,
    TextDocumentRename,
    TextDocumentPrepareRename,
    TextDocumentFoldingRange,
    TextDocumentSelectionRange,
    TextDocumentLinkedEditingRange,
    TextDocumentMoniker,

    // ── Semantic Tokens ──────────────────────────────────────────────────────
    TextDocumentSemanticTokensFull,
    TextDocumentSemanticTokensDelta,
    TextDocumentSemanticTokensRange,
    WorkspaceSemanticTokensRefresh,

    // ── Inlay Hints ──────────────────────────────────────────────────────────
    TextDocumentInlayHint,
    InlayHintResolve,
    WorkspaceInlayHintRefresh,

    // ── Inline Values ────────────────────────────────────────────────────────
    TextDocumentInlineValue,
    WorkspaceInlineValueRefresh,

    // ── Diagnostics ──────────────────────────────────────────────────────────
    TextDocumentDiagnostic,
    WorkspaceDiagnostic,
    WorkspaceDiagnosticRefresh,
    NotebookDocumentDiagnostic,

    // ── Call / Type Hierarchy ────────────────────────────────────────────────
    TextDocumentPrepareCallHierarchy,
    CallHierarchyIncomingCalls,
    CallHierarchyOutgoingCalls,
    TextDocumentPrepareTypeHierarchy,
    TypeHierarchySupertypes,
    TypeHierarchySubtypes,

    // ── Code Lens (workspace) ────────────────────────────────────────────────
    WorkspaceCodeLensRefresh,

    // ── Workspace ────────────────────────────────────────────────────────────
    WorkspaceSymbol,
    WorkspaceSymbolResolve,
    WorkspaceExecuteCommand,
    WorkspaceApplyEdit,
    WorkspaceTextDocumentContent,
    WorkspaceFoldingRangeRefresh,

    // ── Notebook Documents ───────────────────────────────────────────────────
    NotebookDocumentDidOpen,
    NotebookDocumentDidChange,
    NotebookDocumentDidSave,
    NotebookDocumentDidClose,

    // ── Extension point ──────────────────────────────────────────────────────
    /// Non-standard or extension method; carries the raw method string.
    Custom(String),
}

impl LspActivity {
    /// Returns the LSP method string for this activity.
    pub fn method_name(&self) -> &str {
        match self {
            // Lifecycle
            Self::Initialize => "initialize",
            Self::Initialized => "initialized",
            Self::Shutdown => "shutdown",
            Self::Exit => "exit",
            Self::CancelRequest => "$/cancelRequest",
            Self::Progress => "$/progress",

            // Window
            Self::ShowMessage => "window/showMessage",
            Self::ShowMessageRequest => "window/showMessageRequest",
            Self::LogMessage => "window/logMessage",
            Self::ShowDocument => "window/showDocument",
            Self::WorkDoneProgressCreate => "window/workDoneProgress/create",
            Self::WorkDoneProgressCancel => "window/workDoneProgress/cancel",

            // Telemetry
            Self::TelemetryEvent => "telemetry/event",

            // Client
            Self::RegisterCapability => "client/registerCapability",
            Self::UnregisterCapability => "client/unregisterCapability",

            // Text Document Sync
            Self::TextDocumentDidOpen => "textDocument/didOpen",
            Self::TextDocumentDidChange => "textDocument/didChange",
            Self::TextDocumentWillSave => "textDocument/willSave",
            Self::TextDocumentWillSaveWaitUntil => "textDocument/willSaveWaitUntil",
            Self::TextDocumentDidSave => "textDocument/didSave",
            Self::TextDocumentDidClose => "textDocument/didClose",

            // Language Features
            Self::TextDocumentCompletion => "textDocument/completion",
            Self::CompletionItemResolve => "completionItem/resolve",
            Self::TextDocumentHover => "textDocument/hover",
            Self::TextDocumentSignatureHelp => "textDocument/signatureHelp",
            Self::TextDocumentDeclaration => "textDocument/declaration",
            Self::TextDocumentDefinition => "textDocument/definition",
            Self::TextDocumentTypeDefinition => "textDocument/typeDefinition",
            Self::TextDocumentImplementation => "textDocument/implementation",
            Self::TextDocumentReferences => "textDocument/references",
            Self::TextDocumentDocumentHighlight => "textDocument/documentHighlight",
            Self::TextDocumentDocumentSymbol => "textDocument/documentSymbol",
            Self::TextDocumentCodeAction => "textDocument/codeAction",
            Self::CodeActionResolve => "codeAction/resolve",
            Self::TextDocumentCodeLens => "textDocument/codeLens",
            Self::CodeLensResolve => "codeLens/resolve",
            Self::TextDocumentDocumentLink => "textDocument/documentLink",
            Self::DocumentLinkResolve => "documentLink/resolve",
            Self::TextDocumentFormatting => "textDocument/formatting",
            Self::TextDocumentRangeFormatting => "textDocument/rangeFormatting",
            Self::TextDocumentRangesFormatting => "textDocument/rangesFormatting",
            Self::TextDocumentOnTypeFormatting => "textDocument/onTypeFormatting",
            Self::TextDocumentRename => "textDocument/rename",
            Self::TextDocumentPrepareRename => "textDocument/prepareRename",
            Self::TextDocumentFoldingRange => "textDocument/foldingRange",
            Self::TextDocumentSelectionRange => "textDocument/selectionRange",
            Self::TextDocumentLinkedEditingRange => "textDocument/linkedEditingRange",
            Self::TextDocumentMoniker => "textDocument/moniker",

            // Semantic Tokens
            Self::TextDocumentSemanticTokensFull => "textDocument/semanticTokens/full",
            Self::TextDocumentSemanticTokensDelta => "textDocument/semanticTokens/full/delta",
            Self::TextDocumentSemanticTokensRange => "textDocument/semanticTokens/range",
            Self::WorkspaceSemanticTokensRefresh => "workspace/semanticTokens/refresh",

            // Inlay Hints
            Self::TextDocumentInlayHint => "textDocument/inlayHint",
            Self::InlayHintResolve => "inlayHint/resolve",
            Self::WorkspaceInlayHintRefresh => "workspace/inlayHint/refresh",

            // Inline Values
            Self::TextDocumentInlineValue => "textDocument/inlineValue",
            Self::WorkspaceInlineValueRefresh => "workspace/inlineValue/refresh",

            // Diagnostics
            Self::TextDocumentDiagnostic => "textDocument/diagnostic",
            Self::WorkspaceDiagnostic => "workspace/diagnostic",
            Self::WorkspaceDiagnosticRefresh => "workspace/diagnostic/refresh",
            Self::NotebookDocumentDiagnostic => "notebookDocument/diagnostic",

            // Call / Type Hierarchy
            Self::TextDocumentPrepareCallHierarchy => "textDocument/prepareCallHierarchy",
            Self::CallHierarchyIncomingCalls => "callHierarchy/incomingCalls",
            Self::CallHierarchyOutgoingCalls => "callHierarchy/outgoingCalls",
            Self::TextDocumentPrepareTypeHierarchy => "textDocument/prepareTypeHierarchy",
            Self::TypeHierarchySupertypes => "typeHierarchy/supertypes",
            Self::TypeHierarchySubtypes => "typeHierarchy/subtypes",

            // Code Lens (workspace)
            Self::WorkspaceCodeLensRefresh => "workspace/codeLens/refresh",

            // Workspace
            Self::WorkspaceSymbol => "workspace/symbol",
            Self::WorkspaceSymbolResolve => "workspaceSymbol/resolve",
            Self::WorkspaceExecuteCommand => "workspace/executeCommand",
            Self::WorkspaceApplyEdit => "workspace/applyEdit",
            Self::WorkspaceTextDocumentContent => "workspace/textDocumentContent",
            Self::WorkspaceFoldingRangeRefresh => "workspace/foldingRange/refresh",

            // Notebook Documents
            Self::NotebookDocumentDidOpen => "notebookDocument/didOpen",
            Self::NotebookDocumentDidChange => "notebookDocument/didChange",
            Self::NotebookDocumentDidSave => "notebookDocument/didSave",
            Self::NotebookDocumentDidClose => "notebookDocument/didClose",

            // Custom / extension
            Self::Custom(s) => s.as_str(),
        }
    }

    /// Constructs an `LspActivity` from a raw LSP method string.
    /// Falls back to `Custom` for unrecognised methods.
    pub fn from_method(method: &str) -> Self {
        match method {
            // Lifecycle
            "initialize" => Self::Initialize,
            "initialized" => Self::Initialized,
            "shutdown" => Self::Shutdown,
            "exit" => Self::Exit,
            "$/cancelRequest" => Self::CancelRequest,
            "$/progress" => Self::Progress,

            // Window
            "window/showMessage" => Self::ShowMessage,
            "window/showMessageRequest" => Self::ShowMessageRequest,
            "window/logMessage" => Self::LogMessage,
            "window/showDocument" => Self::ShowDocument,
            "window/workDoneProgress/create" => Self::WorkDoneProgressCreate,
            "window/workDoneProgress/cancel" => Self::WorkDoneProgressCancel,

            // Telemetry
            "telemetry/event" => Self::TelemetryEvent,

            // Client
            "client/registerCapability" => Self::RegisterCapability,
            "client/unregisterCapability" => Self::UnregisterCapability,

            // Text Document Sync
            "textDocument/didOpen" => Self::TextDocumentDidOpen,
            "textDocument/didChange" => Self::TextDocumentDidChange,
            "textDocument/willSave" => Self::TextDocumentWillSave,
            "textDocument/willSaveWaitUntil" => Self::TextDocumentWillSaveWaitUntil,
            "textDocument/didSave" => Self::TextDocumentDidSave,
            "textDocument/didClose" => Self::TextDocumentDidClose,

            // Language Features
            "textDocument/completion" => Self::TextDocumentCompletion,
            "completionItem/resolve" => Self::CompletionItemResolve,
            "textDocument/hover" => Self::TextDocumentHover,
            "textDocument/signatureHelp" => Self::TextDocumentSignatureHelp,
            "textDocument/declaration" => Self::TextDocumentDeclaration,
            "textDocument/definition" => Self::TextDocumentDefinition,
            "textDocument/typeDefinition" => Self::TextDocumentTypeDefinition,
            "textDocument/implementation" => Self::TextDocumentImplementation,
            "textDocument/references" => Self::TextDocumentReferences,
            "textDocument/documentHighlight" => Self::TextDocumentDocumentHighlight,
            "textDocument/documentSymbol" => Self::TextDocumentDocumentSymbol,
            "textDocument/codeAction" => Self::TextDocumentCodeAction,
            "codeAction/resolve" => Self::CodeActionResolve,
            "textDocument/codeLens" => Self::TextDocumentCodeLens,
            "codeLens/resolve" => Self::CodeLensResolve,
            "textDocument/documentLink" => Self::TextDocumentDocumentLink,
            "documentLink/resolve" => Self::DocumentLinkResolve,
            "textDocument/formatting" => Self::TextDocumentFormatting,
            "textDocument/rangeFormatting" => Self::TextDocumentRangeFormatting,
            "textDocument/rangesFormatting" => Self::TextDocumentRangesFormatting,
            "textDocument/onTypeFormatting" => Self::TextDocumentOnTypeFormatting,
            "textDocument/rename" => Self::TextDocumentRename,
            "textDocument/prepareRename" => Self::TextDocumentPrepareRename,
            "textDocument/foldingRange" => Self::TextDocumentFoldingRange,
            "textDocument/selectionRange" => Self::TextDocumentSelectionRange,
            "textDocument/linkedEditingRange" => Self::TextDocumentLinkedEditingRange,
            "textDocument/moniker" => Self::TextDocumentMoniker,

            // Semantic Tokens
            "textDocument/semanticTokens/full" => Self::TextDocumentSemanticTokensFull,
            "textDocument/semanticTokens/full/delta" => Self::TextDocumentSemanticTokensDelta,
            "textDocument/semanticTokens/range" => Self::TextDocumentSemanticTokensRange,
            "workspace/semanticTokens/refresh" => Self::WorkspaceSemanticTokensRefresh,

            // Inlay Hints
            "textDocument/inlayHint" => Self::TextDocumentInlayHint,
            "inlayHint/resolve" => Self::InlayHintResolve,
            "workspace/inlayHint/refresh" => Self::WorkspaceInlayHintRefresh,

            // Inline Values
            "textDocument/inlineValue" => Self::TextDocumentInlineValue,
            "workspace/inlineValue/refresh" => Self::WorkspaceInlineValueRefresh,

            // Diagnostics
            "textDocument/diagnostic" => Self::TextDocumentDiagnostic,
            "workspace/diagnostic" => Self::WorkspaceDiagnostic,
            "workspace/diagnostic/refresh" => Self::WorkspaceDiagnosticRefresh,
            "notebookDocument/diagnostic" => Self::NotebookDocumentDiagnostic,

            // Call / Type Hierarchy
            "textDocument/prepareCallHierarchy" => Self::TextDocumentPrepareCallHierarchy,
            "callHierarchy/incomingCalls" => Self::CallHierarchyIncomingCalls,
            "callHierarchy/outgoingCalls" => Self::CallHierarchyOutgoingCalls,
            "textDocument/prepareTypeHierarchy" => Self::TextDocumentPrepareTypeHierarchy,
            "typeHierarchy/supertypes" => Self::TypeHierarchySupertypes,
            "typeHierarchy/subtypes" => Self::TypeHierarchySubtypes,

            // Code Lens (workspace)
            "workspace/codeLens/refresh" => Self::WorkspaceCodeLensRefresh,

            // Workspace
            "workspace/symbol" => Self::WorkspaceSymbol,
            "workspaceSymbol/resolve" => Self::WorkspaceSymbolResolve,
            "workspace/executeCommand" => Self::WorkspaceExecuteCommand,
            "workspace/applyEdit" => Self::WorkspaceApplyEdit,
            "workspace/textDocumentContent" => Self::WorkspaceTextDocumentContent,
            "workspace/foldingRange/refresh" => Self::WorkspaceFoldingRangeRefresh,

            // Notebook Documents
            "notebookDocument/didOpen" => Self::NotebookDocumentDidOpen,
            "notebookDocument/didChange" => Self::NotebookDocumentDidChange,
            "notebookDocument/didSave" => Self::NotebookDocumentDidSave,
            "notebookDocument/didClose" => Self::NotebookDocumentDidClose,

            // Fallback
            other => Self::Custom(other.to_owned()),
        }
    }

    /// Returns `true` if this activity corresponds to a JSON-RPC **request**
    /// (i.e. expects a response).
    pub fn is_request(&self) -> bool {
        matches!(
            self,
            Self::Initialize
                | Self::Shutdown
                | Self::ShowMessageRequest
                | Self::ShowDocument
                | Self::WorkDoneProgressCreate
                | Self::RegisterCapability
                | Self::UnregisterCapability
                | Self::TextDocumentWillSaveWaitUntil
                | Self::TextDocumentCompletion
                | Self::CompletionItemResolve
                | Self::TextDocumentHover
                | Self::TextDocumentSignatureHelp
                | Self::TextDocumentDeclaration
                | Self::TextDocumentDefinition
                | Self::TextDocumentTypeDefinition
                | Self::TextDocumentImplementation
                | Self::TextDocumentReferences
                | Self::TextDocumentDocumentHighlight
                | Self::TextDocumentDocumentSymbol
                | Self::TextDocumentCodeAction
                | Self::CodeActionResolve
                | Self::TextDocumentCodeLens
                | Self::CodeLensResolve
                | Self::TextDocumentDocumentLink
                | Self::DocumentLinkResolve
                | Self::TextDocumentFormatting
                | Self::TextDocumentRangeFormatting
                | Self::TextDocumentRangesFormatting
                | Self::TextDocumentOnTypeFormatting
                | Self::TextDocumentRename
                | Self::TextDocumentPrepareRename
                | Self::TextDocumentFoldingRange
                | Self::TextDocumentSelectionRange
                | Self::TextDocumentLinkedEditingRange
                | Self::TextDocumentMoniker
                | Self::TextDocumentSemanticTokensFull
                | Self::TextDocumentSemanticTokensDelta
                | Self::TextDocumentSemanticTokensRange
                | Self::WorkspaceSemanticTokensRefresh
                | Self::TextDocumentInlayHint
                | Self::InlayHintResolve
                | Self::WorkspaceInlayHintRefresh
                | Self::TextDocumentInlineValue
                | Self::WorkspaceInlineValueRefresh
                | Self::TextDocumentDiagnostic
                | Self::WorkspaceDiagnostic
                | Self::WorkspaceDiagnosticRefresh
                | Self::NotebookDocumentDiagnostic
                | Self::TextDocumentPrepareCallHierarchy
                | Self::CallHierarchyIncomingCalls
                | Self::CallHierarchyOutgoingCalls
                | Self::TextDocumentPrepareTypeHierarchy
                | Self::TypeHierarchySupertypes
                | Self::TypeHierarchySubtypes
                | Self::WorkspaceCodeLensRefresh
                | Self::WorkspaceSymbol
                | Self::WorkspaceSymbolResolve
                | Self::WorkspaceExecuteCommand
                | Self::WorkspaceApplyEdit
                | Self::WorkspaceTextDocumentContent
                | Self::WorkspaceFoldingRangeRefresh
        )
    }

    /// Returns `true` if this activity corresponds to a JSON-RPC **notification**
    /// (i.e. no response expected).
    pub fn is_notification(&self) -> bool {
        matches!(
            self,
            Self::Initialized
                | Self::Exit
                | Self::CancelRequest
                | Self::Progress
                | Self::ShowMessage
                | Self::LogMessage
                | Self::WorkDoneProgressCancel
                | Self::TelemetryEvent
                | Self::TextDocumentDidOpen
                | Self::TextDocumentDidChange
                | Self::TextDocumentWillSave
                | Self::TextDocumentDidSave
                | Self::TextDocumentDidClose
                | Self::NotebookDocumentDidOpen
                | Self::NotebookDocumentDidChange
                | Self::NotebookDocumentDidSave
                | Self::NotebookDocumentDidClose
        )
    }
}

/// A single LSP event — the atomic unit of process mining observation.
///
/// Analogous to an *event* in the XES standard, belonging to a *case* (`LspTrace`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LspEvent {
    /// Unix timestamp in milliseconds when the message was observed.
    pub timestamp_ms: i64,
    /// The LSP method / activity that occurred.
    pub activity: LspActivity,
    /// Which side sent this message.
    pub direction: MessageDirection,
    /// JSON-RPC message kind.
    pub kind: MessageKind,
    /// The JSON-RPC `id` field, present for requests and responses.
    pub message_id: Option<u64>,
    /// Round-trip duration in milliseconds; set on response events only.
    pub duration_ms: Option<i64>,
}

impl LspEvent {
    /// Constructs a request event.
    pub fn request(
        timestamp_ms: i64,
        activity: LspActivity,
        direction: MessageDirection,
        id: u64,
    ) -> Self {
        Self {
            timestamp_ms,
            activity,
            direction,
            kind: MessageKind::Request,
            message_id: Some(id),
            duration_ms: None,
        }
    }

    /// Constructs a notification event (no id, no duration).
    pub fn notification(
        timestamp_ms: i64,
        activity: LspActivity,
        direction: MessageDirection,
    ) -> Self {
        Self {
            timestamp_ms,
            activity,
            direction,
            kind: MessageKind::Notification,
            message_id: None,
            duration_ms: None,
        }
    }

    /// Constructs a response event.
    ///
    /// The `activity` is set to `LspActivity::Custom("response".to_owned())` because
    /// responses in LSP carry an `id` but no method name; callers should correlate
    /// with the matching request to determine the activity.
    pub fn response(
        timestamp_ms: i64,
        direction: MessageDirection,
        id: u64,
        duration_ms: Option<i64>,
    ) -> Self {
        Self {
            timestamp_ms,
            activity: LspActivity::Custom("response".to_owned()),
            direction,
            kind: MessageKind::Response,
            message_id: Some(id),
            duration_ms,
        }
    }
}

/// A sequence of LSP events belonging to a single *case* (e.g. one editor session).
///
/// Analogous to a *trace* in the XES standard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LspTrace {
    /// Unique case identifier (e.g. a session UUID or workspace path).
    pub case_id: String,
    /// Optional name of the language server involved in this trace.
    pub server_name: Option<String>,
    /// Ordered sequence of events in this case.
    pub events: Vec<LspEvent>,
}

impl LspTrace {
    /// Creates an empty trace with the given case identifier.
    pub fn new(case_id: impl Into<String>) -> Self {
        Self {
            case_id: case_id.into(),
            server_name: None,
            events: Vec::new(),
        }
    }

    /// Appends an event to this trace.
    pub fn push(&mut self, event: LspEvent) {
        self.events.push(event);
    }

    /// Returns the total elapsed time of this trace in milliseconds
    /// (max timestamp minus min timestamp), or `None` if fewer than two events
    /// are present.
    pub fn duration_ms(&self) -> Option<i64> {
        if self.events.len() < 2 {
            return None;
        }
        let min = self.events.iter().map(|e| e.timestamp_ms).min()?;
        let max = self.events.iter().map(|e| e.timestamp_ms).max()?;
        Some(max - min)
    }

    /// Returns the number of request events in this trace.
    pub fn request_count(&self) -> usize {
        self.events
            .iter()
            .filter(|e| e.kind == MessageKind::Request)
            .count()
    }

    /// Returns the number of notification events in this trace.
    pub fn notification_count(&self) -> usize {
        self.events
            .iter()
            .filter(|e| e.kind == MessageKind::Notification)
            .count()
    }

    /// Iterates over the activity of every event in this trace, in order.
    pub fn activities(&self) -> impl Iterator<Item = &LspActivity> {
        self.events.iter().map(|e| &e.activity)
    }
}

/// A collection of LSP traces — the top-level process mining artefact.
///
/// Analogous to an *event log* in the XES standard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LspEventLog {
    /// Human-readable name for this event log.
    pub name: String,
    /// All cases (traces) recorded in this log.
    pub traces: Vec<LspTrace>,
}

impl LspEventLog {
    /// Creates an empty event log with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            traces: Vec::new(),
        }
    }

    /// Appends a trace (case) to this log.
    pub fn push(&mut self, trace: LspTrace) {
        self.traces.push(trace);
    }

    /// Returns the number of cases (traces) in this log.
    pub fn case_count(&self) -> usize {
        self.traces.len()
    }

    /// Returns the total number of events across all traces.
    pub fn event_count(&self) -> usize {
        self.traces.iter().map(|t| t.events.len()).sum()
    }

    /// Returns all activities seen in this log, sorted by descending frequency.
    ///
    /// The returned vector contains `(activity, count)` pairs.
    pub fn activities_by_frequency(&self) -> Vec<(LspActivity, usize)> {
        let mut counts: HashMap<LspActivity, usize> = HashMap::new();
        for trace in &self.traces {
            for event in &trace.events {
                *counts.entry(event.activity.clone()).or_insert(0) += 1;
            }
        }
        let mut result: Vec<(LspActivity, usize)> = counts.into_iter().collect();
        // Sort descending by count; break ties by method name for determinism.
        result.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.method_name().cmp(b.0.method_name())));
        result
    }
}
