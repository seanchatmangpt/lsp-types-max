/// Jobs-To-Be-Done tests for LSP 3.18 features.
///
/// Each test represents a real protocol scenario a language client or server needs to handle.
use lsp_types_max::*;
use serde_json::json;

fn assert_serde_roundtrip<T>(val: &T, expected: serde_json::Value)
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
{
    let serialized = serde_json::to_value(val).expect("Failed to serialize");
    assert_eq!(serialized, expected, "Serialized JSON does not match expected");
    let deserialized: T = serde_json::from_value(expected).expect("Failed to deserialize");
    assert_eq!(val, &deserialized, "Deserialized value does not match original");
}

// ─── Job: Debug-level window log message (MessageType::DEBUG, @since 3.18.0) ─────────────────

#[test]
fn job_server_sends_debug_log_message() {
    assert_serde_roundtrip(
        &MessageType::DEBUG,
        json!(5),
    );
}

#[test]
fn job_show_message_params_with_debug_type() {
    assert_serde_roundtrip(
        &ShowMessageParams {
            typ: MessageType::DEBUG,
            message: "LSP server debug output".into(),
        },
        json!({"type": 5, "message": "LSP server debug output"}),
    );
}

// ─── Job: Language kind annotation for a text document (@since 3.18.0) ──────────────────────

#[test]
fn job_open_rust_document_with_language_kind() {
    assert_serde_roundtrip(
        &LanguageKind::RUST,
        json!("rust"),
    );
}

#[test]
fn job_open_typescript_document_with_language_kind() {
    assert_serde_roundtrip(
        &LanguageKind::TYPESCRIPT,
        json!("typescript"),
    );
}

#[test]
fn job_open_document_with_custom_language_kind() {
    use std::borrow::Cow;
    assert_serde_roundtrip(
        &LanguageKind::Custom(Cow::Borrowed("my-dsl")),
        json!("my-dsl"),
    );
}

#[test]
fn job_open_python_document_with_language_kind() {
    assert_serde_roundtrip(
        &LanguageKind::PYTHON,
        json!("python"),
    );
}

// ─── Job: Snippet text edit for code actions (@since 3.18.0) ────────────────────────────────

#[test]
fn job_code_action_applies_snippet_insert() {
    assert_serde_roundtrip(
        &SnippetTextEdit {
            range: Range::new(Position::new(5, 0), Position::new(5, 0)),
            snippet: "fn ${1:name}($2) -> $3 {\n\t$0\n}".into(),
            annotation_id: None,
        },
        json!({
            "range": {"start": {"line": 5, "character": 0}, "end": {"line": 5, "character": 0}},
            "snippet": "fn ${1:name}($2) -> $3 {\n\t$0\n}"
        }),
    );
}

#[test]
fn job_code_action_applies_snippet_with_annotation() {
    assert_serde_roundtrip(
        &SnippetTextEdit {
            range: Range::new(Position::new(0, 0), Position::new(0, 10)),
            snippet: "${1:replacement}".into(),
            annotation_id: Some("rename-op".into()),
        },
        json!({
            "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 10}},
            "snippet": "${1:replacement}",
            "annotationId": "rename-op"
        }),
    );
}

// ─── Job: Workspace edit with metadata (@since 3.18.0) ──────────────────────────────────────

#[test]
fn job_rename_symbol_with_workspace_edit_metadata() {
    assert_serde_roundtrip(
        &WorkspaceEditMetadata {
            label: Some("Rename symbol".into()),
            description: Some("Rename 'foo' to 'bar' across workspace".into()),
            is_auto_apply: Some(false),
        },
        json!({
            "label": "Rename symbol",
            "description": "Rename 'foo' to 'bar' across workspace",
            "isAutoApply": false
        }),
    );
}

#[test]
fn job_auto_fix_with_workspace_edit_metadata() {
    assert_serde_roundtrip(
        &WorkspaceEditMetadata {
            label: Some("Auto-fix unused import".into()),
            description: None,
            is_auto_apply: Some(true),
        },
        json!({
            "label": "Auto-fix unused import",
            "isAutoApply": true
        }),
    );
}

#[test]
fn job_workspace_edit_metadata_minimal() {
    assert_serde_roundtrip(
        &WorkspaceEditMetadata::default(),
        json!({}),
    );
}

// ─── Job: Completion list with item defaults including data field (@since 3.18.0) ────────────

#[test]
fn job_completion_list_carries_shared_data_for_resolve() {
    assert_serde_roundtrip(
        &CompletionList {
            is_incomplete: false,
            item_defaults: Some(CompletionListItemDefaults {
                commit_characters: Some(vec![".".into(), "(".into()]),
                edit_range: None,
                insert_text_format: None,
                insert_text_mode: None,
                data: Some(json!({"source": "my-lsp", "version": 2})),
            }),
            items: vec![
                CompletionItem {
                    label: "println".into(),
                    kind: Some(CompletionItemKind::FUNCTION),
                    ..Default::default()
                },
            ],
        },
        json!({
            "isIncomplete": false,
            "itemDefaults": {
                "commitCharacters": [".", "("],
                "data": {"source": "my-lsp", "version": 2}
            },
            "items": [{"label": "println", "kind": 3}]
        }),
    );
}

#[test]
fn job_completion_list_item_defaults_with_edit_range_and_data() {
    let insert_range = Range::new(Position::new(3, 5), Position::new(3, 8));
    let replace_range = Range::new(Position::new(3, 5), Position::new(3, 12));
    assert_serde_roundtrip(
        &CompletionListItemDefaults {
            commit_characters: None,
            edit_range: Some(OneOf::Right(CompletionListItemDefaultsEditRange {
                insert: insert_range,
                replace: replace_range,
            })),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            data: Some(json!(42)),
        },
        json!({
            "editRange": {
                "insert": {"start": {"line": 3, "character": 5}, "end": {"line": 3, "character": 8}},
                "replace": {"start": {"line": 3, "character": 5}, "end": {"line": 3, "character": 12}}
            },
            "insertTextFormat": 2,
            "data": 42
        }),
    );
}

// ─── Job: Semantic token LABEL type (@since 3.18.0) ─────────────────────────────────────────

#[test]
fn job_server_registers_label_semantic_token_type() {
    assert_serde_roundtrip(
        &SemanticTokenType::LABEL,
        json!("label"),
    );
}

#[test]
fn job_semantic_tokens_legend_includes_label() {
    let legend = SemanticTokensLegend {
        token_types: vec![
            SemanticTokenType::KEYWORD,
            SemanticTokenType::VARIABLE,
            SemanticTokenType::LABEL,
        ],
        token_modifiers: vec![],
    };
    assert_serde_roundtrip(
        &legend,
        json!({
            "tokenTypes": ["keyword", "variable", "label"],
            "tokenModifiers": []
        }),
    );
}

// ─── Job: Document range formatting with ranges_support capability (@since 3.18.0) ───────────

#[test]
fn job_client_announces_multi_range_formatting_support() {
    assert_serde_roundtrip(
        &DocumentRangeFormattingClientCapabilities {
            dynamic_registration: Some(true),
            ranges_support: Some(true),
        },
        json!({"dynamicRegistration": true, "rangesSupport": true}),
    );
}

#[test]
fn job_client_announces_range_formatting_without_multi_range() {
    assert_serde_roundtrip(
        &DocumentRangeFormattingClientCapabilities {
            dynamic_registration: Some(true),
            ranges_support: None,
        },
        json!({"dynamicRegistration": true}),
    );
}

// ─── Job: Format multiple ranges in one request (textDocument/rangesFormatting, @since 3.18.0)

#[test]
fn job_format_selected_regions_in_document() {
    let uri: Uri = "file:///src/main.rs".parse().unwrap();
    assert_serde_roundtrip(
        &DocumentRangesFormattingParams {
            text_document: TextDocumentIdentifier::new(uri),
            ranges: vec![
                Range::new(Position::new(0, 0), Position::new(5, 0)),
                Range::new(Position::new(10, 0), Position::new(15, 0)),
            ],
            options: FormattingOptions {
                tab_size: 4,
                insert_spaces: true,
                ..Default::default()
            },
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "ranges": [
                {"start": {"line": 0, "character": 0}, "end": {"line": 5, "character": 0}},
                {"start": {"line": 10, "character": 0}, "end": {"line": 15, "character": 0}}
            ],
            "options": {"tabSize": 4, "insertSpaces": true}
        }),
    );
}

#[test]
fn job_client_ranges_formatting_capabilities() {
    assert_serde_roundtrip(
        &DocumentRangesFormattingClientCapabilities {
            dynamic_registration: Some(false),
        },
        json!({"dynamicRegistration": false}),
    );
}

#[test]
fn job_server_announces_ranges_formatting_options() {
    assert_serde_roundtrip(
        &DocumentRangesFormattingOptions {
            work_done_progress_options: WorkDoneProgressOptions { work_done_progress: None },
        },
        json!({}),
    );
}

// ─── Job: Workspace text document content provider (@since 3.18.0) ───────────────────────────

#[test]
fn job_client_requests_virtual_document_content() {
    let uri: Uri = "lsp-virtual:///generated/src/lib.rs".parse().unwrap();
    assert_serde_roundtrip(
        &TextDocumentContentParams {
            text_document: TextDocumentIdentifier::new(uri),
        },
        json!({"textDocument": {"uri": "lsp-virtual:///generated/src/lib.rs"}}),
    );
}

#[test]
fn job_server_returns_virtual_document_content() {
    assert_serde_roundtrip(
        &TextDocumentContentResult {
            text: "// Auto-generated by build\npub fn generated() {}".into(),
        },
        json!({"text": "// Auto-generated by build\npub fn generated() {}"}),
    );
}

#[test]
fn job_client_announces_text_document_content_capability() {
    assert_serde_roundtrip(
        &TextDocumentContentClientCapabilities {
            dynamic_registration: Some(true),
        },
        json!({"dynamicRegistration": true}),
    );
}

#[test]
fn job_server_registers_text_document_content_schemes() {
    assert_serde_roundtrip(
        &TextDocumentContentOptions {
            schemes: vec!["lsp-virtual".into(), "mcp".into()],
        },
        json!({"schemes": ["lsp-virtual", "mcp"]}),
    );
}

#[test]
fn job_server_refreshes_virtual_document_content() {
    let uri1: Uri = "lsp-virtual:///gen/a.rs".parse().unwrap();
    let uri2: Uri = "lsp-virtual:///gen/b.rs".parse().unwrap();
    assert_serde_roundtrip(
        &TextDocumentContentRefreshParams {
            text_documents: vec![
                TextDocumentIdentifier::new(uri1),
                TextDocumentIdentifier::new(uri2),
            ],
        },
        json!({
            "textDocuments": [
                {"uri": "lsp-virtual:///gen/a.rs"},
                {"uri": "lsp-virtual:///gen/b.rs"}
            ]
        }),
    );
}

// ─── Job: Folding range workspace refresh (@since 3.18.0) ────────────────────────────────────

#[test]
fn job_client_supports_folding_range_refresh() {
    assert_serde_roundtrip(
        &FoldingRangeWorkspaceClientCapabilities {
            refresh_support: Some(true),
        },
        json!({"refreshSupport": true}),
    );
}

#[test]
fn job_client_does_not_support_folding_range_refresh() {
    assert_serde_roundtrip(
        &FoldingRangeWorkspaceClientCapabilities {
            refresh_support: Some(false),
        },
        json!({"refreshSupport": false}),
    );
}

#[test]
fn job_folding_range_workspace_capability_defaults() {
    assert_serde_roundtrip(
        &FoldingRangeWorkspaceClientCapabilities::default(),
        json!({}),
    );
}

// ─── Job: Notebook document diagnostic pull (@since 3.18.0) ─────────────────────────────────

#[test]
fn job_client_announces_notebook_diagnostic_capability() {
    assert_serde_roundtrip(
        &NotebookDiagnosticClientCapabilities {
            dynamic_registration: Some(true),
        },
        json!({"dynamicRegistration": true}),
    );
}

#[test]
fn job_client_pulls_notebook_diagnostics() {
    let uri: Uri = "file:///notebook.ipynb".parse().unwrap();
    assert_serde_roundtrip(
        &NotebookDiagnosticParams {
            notebook_document: NotebookDocumentIdentifier { uri },
            previous_result_id: None,
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
            partial_result_params: PartialResultParams { partial_result_token: None, is_partial_result_token_null: false },
        },
        json!({
            "notebookDocument": {"uri": "file:///notebook.ipynb"}
        }),
    );
}

#[test]
fn job_client_pulls_notebook_diagnostics_with_delta_id() {
    let uri: Uri = "file:///notebook.ipynb".parse().unwrap();
    assert_serde_roundtrip(
        &NotebookDiagnosticParams {
            notebook_document: NotebookDocumentIdentifier { uri },
            previous_result_id: Some("result-id-42".into()),
            work_done_progress_params: WorkDoneProgressParams { work_done_token: None },
            partial_result_params: PartialResultParams { partial_result_token: None, is_partial_result_token_null: false },
        },
        json!({
            "notebookDocument": {"uri": "file:///notebook.ipynb"},
            "previousResultId": "result-id-42"
        }),
    );
}

#[test]
fn job_server_returns_notebook_diagnostic_report() {
    let cell_uri: Uri = "file:///notebook.ipynb#cell0".parse().unwrap();
    let report = NotebookDiagnosticReport {
        items: vec![
            OneOf::Left(NotebookCellDiagnosticReport {
                uri: cell_uri,
                report: DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                    related_documents: None,
                    full_document_diagnostic_report: FullDocumentDiagnosticReport {
                        result_id: None,
                        items: vec![
                            Diagnostic {
                                range: Range::new(Position::new(1, 0), Position::new(1, 5)),
                                severity: Some(DiagnosticSeverity::ERROR),
                                code: None,
                                code_description: None,
                                source: Some("my-lsp".into()),
                                message: "Undefined variable".into(),
                                related_information: None,
                                tags: None,
                                data: None,
                            },
                        ],
                    },
                }),
            }),
        ],
    };
    assert_serde_roundtrip(
        &report,
        json!({
            "items": [{
                "uri": "file:///notebook.ipynb#cell0",
                "kind": "full",
                "items": [{
                    "range": {"start": {"line": 1, "character": 0}, "end": {"line": 1, "character": 5}},
                    "severity": 1,
                    "source": "my-lsp",
                    "message": "Undefined variable"
                }]
            }]
        }),
    );
}

#[test]
fn job_server_returns_empty_notebook_diagnostic_report() {
    assert_serde_roundtrip(
        &NotebookDiagnosticReport { items: vec![] },
        json!({"items": []}),
    );
}

// ─── Job: Notebook diagnostic server options (@since 3.18.0) ─────────────────────────────────

#[test]
fn job_server_registers_notebook_diagnostic_provider() {
    let opts = NotebookDiagnosticOptions {
        identifier: Some("my-notebook-diag".into()),
        notebook_selector: vec![
            NotebookSelector::ByNotebook {
                notebook: Notebook::String("jupyter-notebook".into()),
                cells: None,
            },
        ],
        work_done_progress_options: WorkDoneProgressOptions { work_done_progress: None },
    };
    assert_serde_roundtrip(
        &opts,
        json!({
            "identifier": "my-notebook-diag",
            "notebookSelector": [{"notebook": "jupyter-notebook"}]
        }),
    );
}

// ─── Job: DocumentFilterExt with relative glob pattern (@since 3.18.0) ───────────────────────

#[test]
fn job_register_capability_with_relative_glob_pattern() {
    assert_serde_roundtrip(
        &DocumentFilterExt {
            language: Some("rust".into()),
            scheme: None,
            pattern: Some(GlobPattern::String("**/*.rs".into())),
        },
        json!({"language": "rust", "pattern": "**/*.rs"}),
    );
}

#[test]
fn job_filter_by_scheme_for_virtual_documents() {
    assert_serde_roundtrip(
        &DocumentFilterExt {
            language: None,
            scheme: Some("lsp-virtual".into()),
            pattern: None,
        },
        json!({"scheme": "lsp-virtual"}),
    );
}

// ─── Job: Full initialize handshake with all LSP 3.18 capabilities ───────────────────────────

#[test]
fn job_client_initialize_with_318_text_document_capabilities() {
    let caps = TextDocumentClientCapabilities {
        range_formatting: Some(DocumentRangeFormattingClientCapabilities {
            dynamic_registration: Some(true),
            ranges_support: Some(true),
        }),
        ranges_formatting: Some(DocumentRangesFormattingClientCapabilities {
            dynamic_registration: Some(true),
        }),
        ..Default::default()
    };
    let val = serde_json::to_value(&caps).unwrap();
    assert_eq!(
        val["rangeFormatting"],
        json!({"dynamicRegistration": true, "rangesSupport": true})
    );
    assert_eq!(
        val["rangesFormatting"],
        json!({"dynamicRegistration": true})
    );
}

#[test]
fn job_client_initialize_with_318_workspace_capabilities() {
    let caps = WorkspaceClientCapabilities {
        folding_range: Some(FoldingRangeWorkspaceClientCapabilities {
            refresh_support: Some(true),
        }),
        text_document_content: Some(TextDocumentContentClientCapabilities {
            dynamic_registration: Some(true),
        }),
        ..Default::default()
    };
    let val = serde_json::to_value(&caps).unwrap();
    assert_eq!(val["foldingRange"], json!({"refreshSupport": true}));
    assert_eq!(val["textDocumentContent"], json!({"dynamicRegistration": true}));
}

#[test]
fn job_client_initialize_with_318_notebook_capabilities() {
    let caps = NotebookDocumentClientCapabilities {
        synchronization: NotebookDocumentSyncClientCapabilities {
            dynamic_registration: Some(true),
            execution_summary_report: Some(false),
        },
        diagnostic: Some(NotebookDiagnosticClientCapabilities {
            dynamic_registration: Some(true),
        }),
    };
    assert_serde_roundtrip(
        &caps,
        json!({
            "synchronization": {
                "dynamicRegistration": true,
                "executionSummaryReport": false
            },
            "diagnostic": {"dynamicRegistration": true}
        }),
    );
}

#[test]
fn job_server_advertises_318_capabilities() {
    let caps = ServerCapabilities {
        document_ranges_formatting_provider: Some(OneOf::Left(true)),
        text_document_content_provider: Some(OneOf::Left(TextDocumentContentOptions {
            schemes: vec!["lsp-virtual".into()],
        })),
        ..Default::default()
    };
    let val = serde_json::to_value(&caps).unwrap();
    assert_eq!(val["documentRangesFormattingProvider"], json!(true));
    assert_eq!(
        val["textDocumentContentProvider"],
        json!({"schemes": ["lsp-virtual"]})
    );
}

// ─── Job: Request type constants expose correct method strings ────────────────────────────────

#[test]
fn job_ranges_formatting_request_has_correct_method() {
    use lsp_types_max::request::Request;
    assert_eq!(
        <DocumentRangesFormatting as Request>::METHOD,
        "textDocument/rangesFormatting"
    );
}

#[test]
fn job_text_document_content_request_has_correct_method() {
    use lsp_types_max::request::Request;
    assert_eq!(
        <TextDocumentContentRequest as Request>::METHOD,
        "workspace/textDocumentContent"
    );
}

#[test]
fn job_text_document_content_refresh_has_correct_method() {
    use lsp_types_max::request::Request;
    assert_eq!(
        <TextDocumentContentRefreshRequest as Request>::METHOD,
        "workspace/textDocumentContent/refresh"
    );
}

#[test]
fn job_folding_range_refresh_has_correct_method() {
    use lsp_types_max::request::Request;
    assert_eq!(
        <FoldingRangeRefresh as Request>::METHOD,
        "workspace/foldingRange/refresh"
    );
}

#[test]
fn job_notebook_diagnostic_request_has_correct_method() {
    use lsp_types_max::request::Request;
    assert_eq!(
        <NotebookDiagnosticRequest as Request>::METHOD,
        "notebookDocument/diagnostic"
    );
}

// ─── Job: lsp_request! macro dispatches 3.18 method strings ──────────────────────────────────

#[test]
fn job_macro_dispatches_ranges_formatting() {
    // lsp_request! expands to a type; verify that type has the correct METHOD.
    type T = lsp_request!("textDocument/rangesFormatting");
    let _method = <T as lsp_types_max::request::Request>::METHOD;
    assert_eq!(_method, "textDocument/rangesFormatting");
}

#[test]
fn job_macro_dispatches_text_document_content() {
    let _method = <lsp_types_max::request::TextDocumentContentRequest as lsp_types_max::request::Request>::METHOD;
    assert_eq!(_method, "workspace/textDocumentContent");
}

#[test]
fn job_macro_dispatches_folding_range_refresh() {
    let _method = <lsp_types_max::request::FoldingRangeRefresh as lsp_types_max::request::Request>::METHOD;
    assert_eq!(_method, "workspace/foldingRange/refresh");
}

// ─── Job: Language-kind-tagged TextDocumentItem roundtrip ────────────────────────────────────

#[test]
fn job_text_document_item_carries_language_kind() {
    // Ensure the core types that use LanguageKind serialize consistently.
    assert_serde_roundtrip(
        &LanguageKind::GO,
        json!("go"),
    );
    assert_serde_roundtrip(
        &LanguageKind::C_SHARP,
        json!("csharp"),
    );
    assert_serde_roundtrip(
        &LanguageKind::CPP,
        json!("cpp"),
    );
}

// ─── Job: DocumentOnTypeFormatting registration with trigger chars ────────────────────────────

#[test]
fn job_register_on_type_formatting_with_triggers() {
    assert_serde_roundtrip(
        &DocumentOnTypeFormattingOptions {
            first_trigger_character: "}".into(),
            more_trigger_character: Some(vec![";".into(), "\n".into()]),
        },
        json!({"firstTriggerCharacter": "}", "moreTriggerCharacter": [";", "\n"]}),
    );
}

// ─── Job: WorkDoneProgress with token for long operations ────────────────────────────────────

#[test]
fn job_ranges_format_with_work_done_token() {
    let uri: Uri = "file:///large.rs".parse().unwrap();
    assert_serde_roundtrip(
        &DocumentRangesFormattingParams {
            text_document: TextDocumentIdentifier::new(uri),
            ranges: vec![Range::new(Position::new(0, 0), Position::new(100, 0))],
            options: FormattingOptions {
                tab_size: 4,
                insert_spaces: true,
                ..Default::default()
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: Some(NumberOrString::String("format-token".into())),
            },
        },
        json!({
            "textDocument": {"uri": "file:///large.rs"},
            "ranges": [
                {"start": {"line": 0, "character": 0}, "end": {"line": 100, "character": 0}}
            ],
            "options": {"tabSize": 4, "insertSpaces": true},
            "workDoneToken": "format-token"
        }),
    );
}

// ─── Job: Inline completion (proposed) ───────────────────────────────────────────────────────

#[cfg(feature = "proposed")]
#[test]
fn job_client_requests_inline_completion() {
    use lsp_types_max::request::Request;
    assert_eq!(
        <lsp_types_max::request::InlineCompletionRequest as Request>::METHOD,
        "textDocument/inlineCompletion"
    );
}

#[cfg(feature = "proposed")]
#[test]
fn job_inline_completion_item_roundtrip() {
    assert_serde_roundtrip(
        &InlineCompletionItem {
            insert_text: StringOrStringValue::String("let x = 42;".into()),
            filter_text: None,
            range: None,
            command: None,
            insert_text_format: None,
        },
        json!({"insertText": "let x = 42;"}),
    );
}
