/// Jobs-To-Be-Done tests for ALL LSP request and notification parameter/result types.
///
/// Each test represents a real protocol scenario with complete roundtrip serde coverage.
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

fn make_uri(s: &str) -> Uri {
    s.parse().unwrap()
}

fn make_range(sl: u32, sc: u32, el: u32, ec: u32) -> Range {
    Range::new(Position::new(sl, sc), Position::new(el, ec))
}

// ─── Initialize / Lifecycle ──────────────────────────────────────────────────

#[test]
fn job_initialize_params_minimal() {
    assert_serde_roundtrip(
        &InitializeParams {
            process_id: Some(12345),
            capabilities: ClientCapabilities::default(),
            ..Default::default()
        },
        json!({
            "processId": 12345,
            "capabilities": {},
            "rootUri": null
        }),
    );
}

#[test]
fn job_initialized_params_empty_struct() {
    assert_serde_roundtrip(&InitializedParams {}, json!({}));
}

#[test]
fn job_cancel_params_number_id() {
    assert_serde_roundtrip(
        &CancelParams {
            id: NumberOrString::Number(42),
        },
        json!({"id": 42}),
    );
}

#[test]
fn job_cancel_params_string_id() {
    assert_serde_roundtrip(
        &CancelParams {
            id: NumberOrString::String("req-abc".into()),
        },
        json!({"id": "req-abc"}),
    );
}

#[test]
fn job_initialize_result_server_capabilities_basic() {
    assert_serde_roundtrip(
        &InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".into(), "::".into()]),
                    resolve_provider: Some(true),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                document_highlight_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                code_lens_provider: Some(CodeLensOptions {
                    resolve_provider: Some(true),
                }),
                document_formatting_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "my-lsp".into(),
                version: Some("1.0.0".into()),
            }),
        },
        json!({
            "capabilities": {
                "textDocumentSync": 2,
                "hoverProvider": true,
                "completionProvider": {
                    "triggerCharacters": [".", "::"],
                    "resolveProvider": true
                },
                "definitionProvider": true,
                "referencesProvider": true,
                "documentHighlightProvider": true,
                "documentSymbolProvider": true,
                "workspaceSymbolProvider": true,
                "codeActionProvider": true,
                "codeLensProvider": {"resolveProvider": true},
                "documentFormattingProvider": true,
                "renameProvider": true
            },
            "serverInfo": {"name": "my-lsp", "version": "1.0.0"}
        }),
    );
}

// ─── Text Document Sync ───────────────────────────────────────────────────────

#[test]
fn job_did_open_text_document_rust_file() {
    assert_serde_roundtrip(
        &DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: make_uri("file:///project/src/main.rs"),
                language_id: "rust".into(),
                version: 1,
                text: "fn main() {}\n".into(),
            },
        },
        json!({
            "textDocument": {
                "uri": "file:///project/src/main.rs",
                "languageId": "rust",
                "version": 1,
                "text": "fn main() {}\n"
            }
        }),
    );
}

#[test]
fn job_did_change_text_document_full_sync() {
    assert_serde_roundtrip(
        &DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: make_uri("file:///project/src/lib.rs"),
                version: 3,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: "pub fn foo() {}\n".into(),
            }],
        },
        json!({
            "textDocument": {"uri": "file:///project/src/lib.rs", "version": 3},
            "contentChanges": [{"text": "pub fn foo() {}\n"}]
        }),
    );
}

#[test]
fn job_did_change_text_document_incremental() {
    assert_serde_roundtrip(
        &DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: make_uri("file:///project/src/lib.rs"),
                version: 4,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: Some(make_range(5, 4, 5, 7)),
                range_length: Some(3),
                text: "bar".into(),
            }],
        },
        json!({
            "textDocument": {"uri": "file:///project/src/lib.rs", "version": 4},
            "contentChanges": [{
                "range": {"start": {"line": 5, "character": 4}, "end": {"line": 5, "character": 7}},
                "rangeLength": 3,
                "text": "bar"
            }]
        }),
    );
}

#[test]
fn job_text_document_content_change_event_full() {
    assert_serde_roundtrip(
        &TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: "entire document content".into(),
        },
        json!({"text": "entire document content"}),
    );
}

#[test]
fn job_text_document_content_change_event_ranged() {
    assert_serde_roundtrip(
        &TextDocumentContentChangeEvent {
            range: Some(make_range(0, 0, 0, 5)),
            range_length: None,
            text: "hello".into(),
        },
        json!({
            "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 5}},
            "text": "hello"
        }),
    );
}

#[test]
fn job_will_save_text_document_manual() {
    assert_serde_roundtrip(
        &WillSaveTextDocumentParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            reason: TextDocumentSaveReason::MANUAL,
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "reason": 1
        }),
    );
}

#[test]
fn job_will_save_text_document_after_delay() {
    assert_serde_roundtrip(
        &WillSaveTextDocumentParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/lib.rs"),
            },
            reason: TextDocumentSaveReason::AFTER_DELAY,
        },
        json!({
            "textDocument": {"uri": "file:///src/lib.rs"},
            "reason": 2
        }),
    );
}

#[test]
fn job_will_save_text_document_focus_out() {
    assert_serde_roundtrip(
        &WillSaveTextDocumentParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/mod.rs"),
            },
            reason: TextDocumentSaveReason::FOCUS_OUT,
        },
        json!({
            "textDocument": {"uri": "file:///src/mod.rs"},
            "reason": 3
        }),
    );
}

#[test]
fn job_did_save_text_document_with_text() {
    assert_serde_roundtrip(
        &DidSaveTextDocumentParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            text: Some("fn main() {}\n".into()),
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "text": "fn main() {}\n"
        }),
    );
}

#[test]
fn job_did_save_text_document_without_text() {
    assert_serde_roundtrip(
        &DidSaveTextDocumentParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/lib.rs"),
            },
            text: None,
        },
        json!({"textDocument": {"uri": "file:///src/lib.rs"}}),
    );
}

#[test]
fn job_did_close_text_document() {
    assert_serde_roundtrip(
        &DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
        },
        json!({"textDocument": {"uri": "file:///src/main.rs"}}),
    );
}

// ─── Completion ───────────────────────────────────────────────────────────────

#[test]
fn job_completion_params_with_trigger_character() {
    assert_serde_roundtrip(
        &CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: make_uri("file:///src/main.rs"),
                },
                position: Position::new(10, 5),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
            context: Some(CompletionContext {
                trigger_kind: CompletionTriggerKind::TRIGGER_CHARACTER,
                trigger_character: Some(".".into()),
            }),
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "position": {"line": 10, "character": 5},
            "context": {"triggerKind": 2, "triggerCharacter": "."}
        }),
    );
}

#[test]
fn job_completion_trigger_kind_invoked() {
    assert_serde_roundtrip(&CompletionTriggerKind::INVOKED, json!(1));
}

#[test]
fn job_completion_trigger_kind_trigger_character() {
    assert_serde_roundtrip(&CompletionTriggerKind::TRIGGER_CHARACTER, json!(2));
}

#[test]
fn job_completion_trigger_kind_for_incomplete() {
    assert_serde_roundtrip(
        &CompletionTriggerKind::TRIGGER_FOR_INCOMPLETE_COMPLETIONS,
        json!(3),
    );
}

#[test]
fn job_insert_text_format_plain_text() {
    assert_serde_roundtrip(&InsertTextFormat::PLAIN_TEXT, json!(1));
}

#[test]
fn job_insert_text_format_snippet() {
    assert_serde_roundtrip(&InsertTextFormat::SNIPPET, json!(2));
}

#[test]
fn job_insert_text_mode_as_is() {
    assert_serde_roundtrip(&InsertTextMode::AS_IS, json!(1));
}

#[test]
fn job_insert_text_mode_adjust_indentation() {
    assert_serde_roundtrip(&InsertTextMode::ADJUST_INDENTATION, json!(2));
}

#[test]
fn job_completion_item_tag_deprecated() {
    assert_serde_roundtrip(&CompletionItemTag::DEPRECATED, json!(1));
}

#[test]
fn job_completion_item_all_fields() {
    assert_serde_roundtrip(
        &CompletionItem {
            label: "println".into(),
            kind: Some(CompletionItemKind::FUNCTION),
            tags: Some(vec![CompletionItemTag::DEPRECATED]),
            detail: Some("fn println(...)".into()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Prints to stdout".into(),
            })),
            deprecated: Some(true),
            preselect: Some(false),
            sort_text: Some("0_println".into()),
            filter_text: Some("println".into()),
            insert_text: Some("println!(\"$1\")".into()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: Some(InsertTextMode::ADJUST_INDENTATION),
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                range: make_range(3, 0, 3, 7),
                new_text: "println!(\"${1:msg}\")".into(),
            })),
            additional_text_edits: Some(vec![TextEdit {
                range: make_range(0, 0, 0, 0),
                new_text: "use std::io;\n".into(),
            }]),
            commit_characters: Some(vec!["\n".into()]),
            command: Some(Command {
                title: "Trigger Parameter Hints".into(),
                command: "editor.action.triggerParameterHints".into(),
                arguments: None,
            }),
            data: Some(json!({"resolveKey": "println_macro"})),
            label_details: None,
        },
        json!({
            "label": "println",
            "kind": 3,
            "tags": [1],
            "detail": "fn println(...)",
            "documentation": {"kind": "markdown", "value": "Prints to stdout"},
            "deprecated": true,
            "preselect": false,
            "sortText": "0_println",
            "filterText": "println",
            "insertText": "println!(\"$1\")",
            "insertTextFormat": 2,
            "insertTextMode": 2,
            "textEdit": {
                "range": {"start": {"line": 3, "character": 0}, "end": {"line": 3, "character": 7}},
                "newText": "println!(\"${1:msg}\")"
            },
            "additionalTextEdits": [{
                "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 0}},
                "newText": "use std::io;\n"
            }],
            "commitCharacters": ["\n"],
            "command": {
                "title": "Trigger Parameter Hints",
                "command": "editor.action.triggerParameterHints"
            },
            "data": {"resolveKey": "println_macro"}
        }),
    );
}

#[test]
fn job_completion_item_kinds_all_25() {
    let kinds = [
        (CompletionItemKind::TEXT, 1),
        (CompletionItemKind::METHOD, 2),
        (CompletionItemKind::FUNCTION, 3),
        (CompletionItemKind::CONSTRUCTOR, 4),
        (CompletionItemKind::FIELD, 5),
        (CompletionItemKind::VARIABLE, 6),
        (CompletionItemKind::CLASS, 7),
        (CompletionItemKind::INTERFACE, 8),
        (CompletionItemKind::MODULE, 9),
        (CompletionItemKind::PROPERTY, 10),
        (CompletionItemKind::UNIT, 11),
        (CompletionItemKind::VALUE, 12),
        (CompletionItemKind::ENUM, 13),
        (CompletionItemKind::KEYWORD, 14),
        (CompletionItemKind::SNIPPET, 15),
        (CompletionItemKind::COLOR, 16),
        (CompletionItemKind::FILE, 17),
        (CompletionItemKind::REFERENCE, 18),
        (CompletionItemKind::FOLDER, 19),
        (CompletionItemKind::ENUM_MEMBER, 20),
        (CompletionItemKind::CONSTANT, 21),
        (CompletionItemKind::STRUCT, 22),
        (CompletionItemKind::EVENT, 23),
        (CompletionItemKind::OPERATOR, 24),
        (CompletionItemKind::TYPE_PARAMETER, 25),
    ];
    for (kind, expected_val) in &kinds {
        assert_serde_roundtrip(kind, json!(expected_val));
    }
}

#[test]
fn job_completion_list_with_item_defaults() {
    assert_serde_roundtrip(
        &CompletionList {
            is_incomplete: true,
            item_defaults: Some(CompletionListItemDefaults {
                commit_characters: Some(vec![".".into()]),
                edit_range: None,
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                insert_text_mode: None,
                data: Some(json!({"source": "rust-analyzer"})),
            }),
            items: vec![CompletionItem {
                label: "vec".into(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                ..Default::default()
            }],
        },
        json!({
            "isIncomplete": true,
            "itemDefaults": {
                "commitCharacters": ["."],
                "insertTextFormat": 2,
                "data": {"source": "rust-analyzer"}
            },
            "items": [{"label": "vec", "kind": 25}]
        }),
    );
}

// ─── Hover ────────────────────────────────────────────────────────────────────

#[test]
fn job_hover_params() {
    assert_serde_roundtrip(
        &HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: make_uri("file:///src/main.rs"),
                },
                position: Position::new(3, 10),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "position": {"line": 3, "character": 10}
        }),
    );
}

#[test]
fn job_hover_result_with_markup_content_markdown() {
    assert_serde_roundtrip(
        &Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "## fn main()\n\nEntry point of the program.".into(),
            }),
            range: Some(make_range(0, 3, 0, 7)),
        },
        json!({
            "contents": {"kind": "markdown", "value": "## fn main()\n\nEntry point of the program."},
            "range": {"start": {"line": 0, "character": 3}, "end": {"line": 0, "character": 7}}
        }),
    );
}

#[test]
fn job_hover_result_with_marked_string_array() {
    assert_serde_roundtrip(
        &Hover {
            contents: HoverContents::Array(vec![
                MarkedString::String("A plain string".into()),
                MarkedString::LanguageString(LanguageString {
                    language: "rust".into(),
                    value: "fn foo() -> i32 { 42 }".into(),
                }),
            ]),
            range: None,
        },
        json!({
            "contents": [
                "A plain string",
                {"language": "rust", "value": "fn foo() -> i32 { 42 }"}
            ]
        }),
    );
}

#[test]
fn job_markup_content_plain_text() {
    assert_serde_roundtrip(
        &MarkupContent {
            kind: MarkupKind::PlainText,
            value: "simple text".into(),
        },
        json!({"kind": "plaintext", "value": "simple text"}),
    );
}

#[test]
fn job_markup_content_markdown() {
    assert_serde_roundtrip(
        &MarkupContent {
            kind: MarkupKind::Markdown,
            value: "**bold** and *italic*".into(),
        },
        json!({"kind": "markdown", "value": "**bold** and *italic*"}),
    );
}

// ─── Signature Help ───────────────────────────────────────────────────────────

#[test]
fn job_signature_help_params() {
    assert_serde_roundtrip(
        &SignatureHelpParams {
            context: Some(SignatureHelpContext {
                trigger_kind: SignatureHelpTriggerKind::TRIGGER_CHARACTER,
                trigger_character: Some("(".into()),
                is_retrigger: false,
                active_signature_help: None,
            }),
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: make_uri("file:///src/main.rs"),
                },
                position: Position::new(5, 12),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
        },
        json!({
            "context": {
                "triggerKind": 2,
                "triggerCharacter": "(",
                "isRetrigger": false
            },
            "textDocument": {"uri": "file:///src/main.rs"},
            "position": {"line": 5, "character": 12}
        }),
    );
}

#[test]
fn job_signature_help_result_with_multiple_signatures() {
    assert_serde_roundtrip(
        &SignatureHelp {
            signatures: vec![
                SignatureInformation {
                    label: "fn write(buf: &[u8]) -> Result<usize>".into(),
                    documentation: Some(Documentation::String(
                        "Writes a buffer into the writer".into(),
                    )),
                    parameters: Some(vec![ParameterInformation {
                        label: ParameterLabel::Simple("buf: &[u8]".into()),
                        documentation: None,
                    }]),
                    active_parameter: None,
                },
                SignatureInformation {
                    label: "fn write_all(buf: &[u8]) -> Result<()>".into(),
                    documentation: None,
                    parameters: Some(vec![ParameterInformation {
                        label: ParameterLabel::LabelOffsets([12, 22]),
                        documentation: Some(Documentation::String("The buffer".into())),
                    }]),
                    active_parameter: Some(0),
                },
            ],
            active_signature: Some(0),
            active_parameter: Some(0),
        },
        json!({
            "signatures": [
                {
                    "label": "fn write(buf: &[u8]) -> Result<usize>",
                    "documentation": "Writes a buffer into the writer",
                    "parameters": [{"label": "buf: &[u8]"}]
                },
                {
                    "label": "fn write_all(buf: &[u8]) -> Result<()>",
                    "parameters": [{"label": [12, 22], "documentation": "The buffer"}],
                    "activeParameter": 0
                }
            ],
            "activeSignature": 0,
            "activeParameter": 0
        }),
    );
}

// ─── Go-to locations ─────────────────────────────────────────────────────────

#[test]
fn job_goto_definition_params() {
    assert_serde_roundtrip(
        &GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: make_uri("file:///src/main.rs"),
                },
                position: Position::new(7, 4),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
            context: None,
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "position": {"line": 7, "character": 4}
        }),
    );
}

#[test]
fn job_goto_definition_response_scalar() {
    assert_serde_roundtrip(
        &GotoDefinitionResponse::Scalar(Location {
            uri: make_uri("file:///src/lib.rs"),
            range: make_range(10, 0, 10, 6),
        }),
        json!({
            "uri": "file:///src/lib.rs",
            "range": {"start": {"line": 10, "character": 0}, "end": {"line": 10, "character": 6}}
        }),
    );
}

#[test]
fn job_goto_definition_response_array() {
    assert_serde_roundtrip(
        &GotoDefinitionResponse::Array(vec![
            Location {
                uri: make_uri("file:///src/lib.rs"),
                range: make_range(10, 0, 10, 6),
            },
            Location {
                uri: make_uri("file:///src/other.rs"),
                range: make_range(20, 4, 20, 10),
            },
        ]),
        json!([
            {
                "uri": "file:///src/lib.rs",
                "range": {"start": {"line": 10, "character": 0}, "end": {"line": 10, "character": 6}}
            },
            {
                "uri": "file:///src/other.rs",
                "range": {"start": {"line": 20, "character": 4}, "end": {"line": 20, "character": 10}}
            }
        ]),
    );
}

#[test]
fn job_goto_definition_response_link() {
    assert_serde_roundtrip(
        &GotoDefinitionResponse::Link(vec![LocationLink {
            origin_selection_range: Some(make_range(3, 4, 3, 10)),
            target_uri: make_uri("file:///src/lib.rs"),
            target_range: make_range(15, 0, 18, 1),
            target_selection_range: make_range(15, 7, 15, 13),
        }]),
        json!([{
            "originSelectionRange": {
                "start": {"line": 3, "character": 4},
                "end": {"line": 3, "character": 10}
            },
            "targetUri": "file:///src/lib.rs",
            "targetRange": {
                "start": {"line": 15, "character": 0},
                "end": {"line": 18, "character": 1}
            },
            "targetSelectionRange": {
                "start": {"line": 15, "character": 7},
                "end": {"line": 15, "character": 13}
            }
        }]),
    );
}

#[test]
fn job_location_link_no_origin_range() {
    assert_serde_roundtrip(
        &LocationLink {
            origin_selection_range: None,
            target_uri: make_uri("file:///src/mod.rs"),
            target_range: make_range(0, 0, 5, 0),
            target_selection_range: make_range(0, 3, 0, 9),
        },
        json!({
            "targetUri": "file:///src/mod.rs",
            "targetRange": {
                "start": {"line": 0, "character": 0},
                "end": {"line": 5, "character": 0}
            },
            "targetSelectionRange": {
                "start": {"line": 0, "character": 3},
                "end": {"line": 0, "character": 9}
            }
        }),
    );
}

#[test]
fn job_references_params_include_declaration() {
    assert_serde_roundtrip(
        &ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: make_uri("file:///src/main.rs"),
                },
                position: Position::new(8, 6),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
            context: ReferenceContext {
                include_declaration: true,
            },
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "position": {"line": 8, "character": 6},
            "context": {"includeDeclaration": true}
        }),
    );
}

// ─── Document Highlights ─────────────────────────────────────────────────────

#[test]
fn job_document_highlight_params() {
    assert_serde_roundtrip(
        &DocumentHighlightParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: make_uri("file:///src/main.rs"),
                },
                position: Position::new(4, 8),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "position": {"line": 4, "character": 8}
        }),
    );
}

#[test]
fn job_document_highlight_text_kind() {
    assert_serde_roundtrip(
        &DocumentHighlight {
            range: make_range(2, 4, 2, 9),
            kind: Some(DocumentHighlightKind::TEXT),
        },
        json!({
            "range": {"start": {"line": 2, "character": 4}, "end": {"line": 2, "character": 9}},
            "kind": 1
        }),
    );
}

#[test]
fn job_document_highlight_read_kind() {
    assert_serde_roundtrip(
        &DocumentHighlight {
            range: make_range(5, 0, 5, 3),
            kind: Some(DocumentHighlightKind::READ),
        },
        json!({
            "range": {"start": {"line": 5, "character": 0}, "end": {"line": 5, "character": 3}},
            "kind": 2
        }),
    );
}

#[test]
fn job_document_highlight_write_kind() {
    assert_serde_roundtrip(
        &DocumentHighlight {
            range: make_range(1, 4, 1, 7),
            kind: Some(DocumentHighlightKind::WRITE),
        },
        json!({
            "range": {"start": {"line": 1, "character": 4}, "end": {"line": 1, "character": 7}},
            "kind": 3
        }),
    );
}

// ─── Document Symbols ────────────────────────────────────────────────────────

#[test]
fn job_document_symbol_params() {
    assert_serde_roundtrip(
        &DocumentSymbolParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({"textDocument": {"uri": "file:///src/main.rs"}}),
    );
}

#[test]
#[allow(deprecated)]
fn job_document_symbol_response_nested_with_children() {
    assert_serde_roundtrip(
        &DocumentSymbolResponse::Nested(vec![DocumentSymbol {
            name: "MyStruct".into(),
            detail: Some("struct MyStruct".into()),
            kind: SymbolKind::STRUCT,
            tags: None,
            deprecated: None,
            range: make_range(0, 0, 10, 1),
            selection_range: make_range(0, 7, 0, 15),
            children: Some(vec![DocumentSymbol {
                name: "field".into(),
                detail: Some("pub field: u32".into()),
                kind: SymbolKind::FIELD,
                tags: None,
                deprecated: None,
                range: make_range(1, 4, 1, 18),
                selection_range: make_range(1, 8, 1, 13),
                children: None,
            }]),
        }]),
        json!([{
            "name": "MyStruct",
            "detail": "struct MyStruct",
            "kind": 23,
            "range": {"start": {"line": 0, "character": 0}, "end": {"line": 10, "character": 1}},
            "selectionRange": {"start": {"line": 0, "character": 7}, "end": {"line": 0, "character": 15}},
            "children": [{
                "name": "field",
                "detail": "pub field: u32",
                "kind": 8,
                "range": {"start": {"line": 1, "character": 4}, "end": {"line": 1, "character": 18}},
                "selectionRange": {"start": {"line": 1, "character": 8}, "end": {"line": 1, "character": 13}}
            }]
        }]),
    );
}

#[test]
#[allow(deprecated)]
fn job_document_symbol_response_flat_with_deprecated() {
    assert_serde_roundtrip(
        &DocumentSymbolResponse::Flat(vec![SymbolInformation {
            name: "old_function".into(),
            kind: SymbolKind::FUNCTION,
            tags: Some(vec![SymbolTag::DEPRECATED]),
            deprecated: Some(true),
            location: Location {
                uri: make_uri("file:///src/main.rs"),
                range: make_range(5, 0, 8, 1),
            },
            container_name: Some("main".into()),
        }]),
        json!([{
            "name": "old_function",
            "kind": 12,
            "tags": [1],
            "deprecated": true,
            "location": {
                "uri": "file:///src/main.rs",
                "range": {"start": {"line": 5, "character": 0}, "end": {"line": 8, "character": 1}}
            },
            "containerName": "main"
        }]),
    );
}

#[test]
fn job_symbol_kinds_common_values() {
    let kinds = [
        (SymbolKind::FILE, 1u64),
        (SymbolKind::MODULE, 2),
        (SymbolKind::CLASS, 5),
        (SymbolKind::METHOD, 6),
        (SymbolKind::PROPERTY, 7),
        (SymbolKind::FUNCTION, 12),
        (SymbolKind::VARIABLE, 13),
        (SymbolKind::CONSTANT, 14),
        (SymbolKind::STRUCT, 23),
        (SymbolKind::ENUM_MEMBER, 22),
    ];
    for (kind, expected) in &kinds {
        assert_serde_roundtrip(kind, json!(expected));
    }
}

// ─── Code Actions ────────────────────────────────────────────────────────────

#[test]
fn job_code_action_params_with_context() {
    assert_serde_roundtrip(
        &CodeActionParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            range: make_range(3, 0, 3, 20),
            context: CodeActionContext {
                diagnostics: vec![Diagnostic {
                    range: make_range(3, 0, 3, 10),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("E0308".into())),
                    message: "mismatched types".into(),
                    ..Default::default()
                }],
                only: Some(vec![CodeActionKind::QUICKFIX]),
                trigger_kind: Some(CodeActionTriggerKind::INVOKED),
                version: None,
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "range": {"start": {"line": 3, "character": 0}, "end": {"line": 3, "character": 20}},
            "context": {
                "diagnostics": [{
                    "range": {"start": {"line": 3, "character": 0}, "end": {"line": 3, "character": 10}},
                    "severity": 1,
                    "code": "E0308",
                    "message": "mismatched types"
                }],
                "only": ["quickfix"],
                "triggerKind": 1
            }
        }),
    );
}

#[test]
fn job_code_action_or_command_code_action() {
    assert_serde_roundtrip(
        &CodeActionOrCommand::CodeAction(CodeAction {
            title: "Fix mismatched types".into(),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: None,
            edit: Some(WorkspaceEdit {
                changes: None,
                document_changes: None,
                ..Default::default()
            }),
            command: None,
            is_preferred: Some(true),
            ..Default::default()
        }),
        json!({
            "title": "Fix mismatched types",
            "kind": "quickfix",
            "edit": {},
            "isPreferred": true
        }),
    );
}

#[test]
fn job_code_action_or_command_command() {
    assert_serde_roundtrip(
        &CodeActionOrCommand::Command(Command {
            title: "Run tests".into(),
            command: "rust-analyzer.runTests".into(),
            arguments: Some(vec![json!({"target": "all"})]),
        }),
        json!({
            "title": "Run tests",
            "command": "rust-analyzer.runTests",
            "arguments": [{"target": "all"}]
        }),
    );
}

#[test]
fn job_code_action_kinds_common_values() {
    assert_serde_roundtrip(&CodeActionKind::QUICKFIX, json!("quickfix"));
    assert_serde_roundtrip(&CodeActionKind::REFACTOR, json!("refactor"));
    assert_serde_roundtrip(&CodeActionKind::REFACTOR_EXTRACT, json!("refactor.extract"));
    assert_serde_roundtrip(&CodeActionKind::SOURCE, json!("source"));
    assert_serde_roundtrip(
        &CodeActionKind::SOURCE_ORGANIZE_IMPORTS,
        json!("source.organizeImports"),
    );
}

// ─── Code Lens ───────────────────────────────────────────────────────────────

#[test]
fn job_code_lens_params() {
    assert_serde_roundtrip(
        &CodeLensParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({"textDocument": {"uri": "file:///src/main.rs"}}),
    );
}

#[test]
fn job_code_lens_with_command_and_data() {
    assert_serde_roundtrip(
        &CodeLens {
            range: make_range(10, 0, 10, 20),
            command: Some(Command {
                title: "2 references".into(),
                command: "editor.action.showReferences".into(),
                arguments: Some(vec![json!("file:///src/main.rs")]),
            }),
            data: Some(json!({"functionName": "main"})),
        },
        json!({
            "range": {"start": {"line": 10, "character": 0}, "end": {"line": 10, "character": 20}},
            "command": {
                "title": "2 references",
                "command": "editor.action.showReferences",
                "arguments": ["file:///src/main.rs"]
            },
            "data": {"functionName": "main"}
        }),
    );
}

// ─── Document Links ───────────────────────────────────────────────────────────

#[test]
fn job_document_link_params() {
    assert_serde_roundtrip(
        &DocumentLinkParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({"textDocument": {"uri": "file:///src/main.rs"}}),
    );
}

#[test]
fn job_document_link_with_target_and_tooltip() {
    assert_serde_roundtrip(
        &DocumentLink {
            range: make_range(5, 10, 5, 45),
            target: Some(make_uri("https://doc.rust-lang.org/std/vec/struct.Vec.html")),
            tooltip: Some("Open documentation".into()),
            data: None,
        },
        json!({
            "range": {"start": {"line": 5, "character": 10}, "end": {"line": 5, "character": 45}},
            "target": "https://doc.rust-lang.org/std/vec/struct.Vec.html",
            "tooltip": "Open documentation"
        }),
    );
}

// ─── Formatting ──────────────────────────────────────────────────────────────

#[test]
fn job_document_formatting_params() {
    assert_serde_roundtrip(
        &DocumentFormattingParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            options: FormattingOptions {
                tab_size: 4,
                insert_spaces: true,
                trim_trailing_whitespace: Some(true),
                insert_final_newline: Some(true),
                trim_final_newlines: Some(false),
                properties: std::collections::HashMap::new(),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            context: None,
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "options": {
                "tabSize": 4,
                "insertSpaces": true,
                "trimTrailingWhitespace": true,
                "insertFinalNewline": true,
                "trimFinalNewlines": false
            }
        }),
    );
}

#[test]
fn job_document_range_formatting_params() {
    assert_serde_roundtrip(
        &DocumentRangeFormattingParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            range: make_range(10, 0, 20, 0),
            options: FormattingOptions {
                tab_size: 2,
                insert_spaces: true,
                properties: std::collections::HashMap::new(),
                trim_trailing_whitespace: None,
                insert_final_newline: None,
                trim_final_newlines: None,
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "range": {"start": {"line": 10, "character": 0}, "end": {"line": 20, "character": 0}},
            "options": {"tabSize": 2, "insertSpaces": true}
        }),
    );
}

#[test]
fn job_document_on_type_formatting_params() {
    assert_serde_roundtrip(
        &DocumentOnTypeFormattingParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: make_uri("file:///src/main.rs"),
                },
                position: Position::new(12, 1),
            },
            ch: "}".into(),
            options: FormattingOptions {
                tab_size: 4,
                insert_spaces: true,
                properties: std::collections::HashMap::new(),
                trim_trailing_whitespace: None,
                insert_final_newline: None,
                trim_final_newlines: None,
            },
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "position": {"line": 12, "character": 1},
            "ch": "}",
            "options": {"tabSize": 4, "insertSpaces": true}
        }),
    );
}

// ─── Rename ───────────────────────────────────────────────────────────────────

#[test]
fn job_rename_params() {
    assert_serde_roundtrip(
        &RenameParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: make_uri("file:///src/main.rs"),
                },
                position: Position::new(3, 8),
            },
            new_name: "new_function_name".into(),
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "position": {"line": 3, "character": 8},
            "newName": "new_function_name"
        }),
    );
}

#[test]
fn job_prepare_rename_response_range() {
    assert_serde_roundtrip(
        &PrepareRenameResponse::Range(make_range(3, 3, 3, 11)),
        json!({
            "start": {"line": 3, "character": 3},
            "end": {"line": 3, "character": 11}
        }),
    );
}

#[test]
fn job_prepare_rename_response_with_placeholder() {
    assert_serde_roundtrip(
        &PrepareRenameResponse::RangeWithPlaceholder {
            range: make_range(5, 4, 5, 12),
            placeholder: "oldName".into(),
        },
        json!({
            "range": {"start": {"line": 5, "character": 4}, "end": {"line": 5, "character": 12}},
            "placeholder": "oldName"
        }),
    );
}

#[test]
fn job_prepare_rename_response_default_behavior() {
    assert_serde_roundtrip(
        &PrepareRenameResponse::DefaultBehavior {
            default_behavior: true,
        },
        json!({"defaultBehavior": true}),
    );
}

// ─── Folding Ranges ──────────────────────────────────────────────────────────

#[test]
fn job_folding_range_params() {
    assert_serde_roundtrip(
        &FoldingRangeParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({"textDocument": {"uri": "file:///src/main.rs"}}),
    );
}

#[test]
fn job_folding_range_comment_kind() {
    assert_serde_roundtrip(
        &FoldingRange {
            start_line: 0,
            start_character: None,
            end_line: 5,
            end_character: None,
            kind: Some(FoldingRangeKind::Comment),
            collapsed_text: Some("// ...".into()),
        },
        json!({
            "startLine": 0,
            "endLine": 5,
            "kind": "comment",
            "collapsedText": "// ..."
        }),
    );
}

#[test]
fn job_folding_range_imports_kind() {
    assert_serde_roundtrip(
        &FoldingRange {
            start_line: 1,
            start_character: Some(0),
            end_line: 10,
            end_character: Some(0),
            kind: Some(FoldingRangeKind::Imports),
            collapsed_text: None,
        },
        json!({
            "startLine": 1,
            "startCharacter": 0,
            "endLine": 10,
            "endCharacter": 0,
            "kind": "imports"
        }),
    );
}

#[test]
fn job_folding_range_region_kind() {
    assert_serde_roundtrip(
        &FoldingRange {
            start_line: 20,
            start_character: None,
            end_line: 50,
            end_character: None,
            kind: Some(FoldingRangeKind::Region),
            collapsed_text: None,
        },
        json!({
            "startLine": 20,
            "endLine": 50,
            "kind": "region"
        }),
    );
}

// ─── Selection Range ─────────────────────────────────────────────────────────

#[test]
fn job_selection_range_params() {
    assert_serde_roundtrip(
        &SelectionRangeParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            positions: vec![Position::new(5, 10)],
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "positions": [{"line": 5, "character": 10}]
        }),
    );
}

#[test]
fn job_selection_range_nested_parent() {
    assert_serde_roundtrip(
        &SelectionRange {
            range: make_range(5, 8, 5, 12),
            parent: Some(Box::new(SelectionRange {
                range: make_range(5, 4, 5, 20),
                parent: Some(Box::new(SelectionRange {
                    range: make_range(5, 0, 6, 0),
                    parent: None,
                })),
            })),
        },
        json!({
            "range": {"start": {"line": 5, "character": 8}, "end": {"line": 5, "character": 12}},
            "parent": {
                "range": {"start": {"line": 5, "character": 4}, "end": {"line": 5, "character": 20}},
                "parent": {
                    "range": {"start": {"line": 5, "character": 0}, "end": {"line": 6, "character": 0}}
                }
            }
        }),
    );
}

// ─── Call Hierarchy ──────────────────────────────────────────────────────────

#[test]
fn job_call_hierarchy_prepare_params() {
    assert_serde_roundtrip(
        &CallHierarchyPrepareParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: make_uri("file:///src/main.rs"),
                },
                position: Position::new(5, 4),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "position": {"line": 5, "character": 4}
        }),
    );
}

#[test]
fn job_call_hierarchy_item_all_fields() {
    assert_serde_roundtrip(
        &CallHierarchyItem {
            name: "process".into(),
            kind: SymbolKind::FUNCTION,
            tags: Some(vec![SymbolTag::DEPRECATED]),
            detail: Some("fn process(data: &Data) -> Result<()>".into()),
            uri: make_uri("file:///src/handler.rs"),
            range: make_range(10, 0, 15, 1),
            selection_range: make_range(10, 7, 10, 14),
            data: Some(json!({"calleeId": "handler::process"})),
        },
        json!({
            "name": "process",
            "kind": 12,
            "tags": [1],
            "detail": "fn process(data: &Data) -> Result<()>",
            "uri": "file:///src/handler.rs",
            "range": {"start": {"line": 10, "character": 0}, "end": {"line": 15, "character": 1}},
            "selectionRange": {"start": {"line": 10, "character": 7}, "end": {"line": 10, "character": 14}},
            "data": {"calleeId": "handler::process"}
        }),
    );
}

#[test]
fn job_call_hierarchy_incoming_call_with_from_ranges() {
    let item = CallHierarchyItem {
        name: "main".into(),
        kind: SymbolKind::FUNCTION,
        tags: None,
        detail: None,
        uri: make_uri("file:///src/main.rs"),
        range: make_range(0, 0, 3, 1),
        selection_range: make_range(0, 3, 0, 7),
        data: None,
    };
    assert_serde_roundtrip(
        &CallHierarchyIncomingCall {
            from: item,
            from_ranges: vec![make_range(2, 4, 2, 11)],
        },
        json!({
            "from": {
                "name": "main",
                "kind": 12,
                "uri": "file:///src/main.rs",
                "range": {"start": {"line": 0, "character": 0}, "end": {"line": 3, "character": 1}},
                "selectionRange": {"start": {"line": 0, "character": 3}, "end": {"line": 0, "character": 7}}
            },
            "fromRanges": [
                {"start": {"line": 2, "character": 4}, "end": {"line": 2, "character": 11}}
            ]
        }),
    );
}

#[test]
fn job_call_hierarchy_outgoing_call() {
    let item = CallHierarchyItem {
        name: "helper".into(),
        kind: SymbolKind::FUNCTION,
        tags: None,
        detail: None,
        uri: make_uri("file:///src/util.rs"),
        range: make_range(5, 0, 8, 1),
        selection_range: make_range(5, 3, 5, 9),
        data: None,
    };
    assert_serde_roundtrip(
        &CallHierarchyOutgoingCall {
            to: item,
            from_ranges: vec![make_range(12, 8, 12, 14)],
        },
        json!({
            "to": {
                "name": "helper",
                "kind": 12,
                "uri": "file:///src/util.rs",
                "range": {"start": {"line": 5, "character": 0}, "end": {"line": 8, "character": 1}},
                "selectionRange": {"start": {"line": 5, "character": 3}, "end": {"line": 5, "character": 9}}
            },
            "fromRanges": [
                {"start": {"line": 12, "character": 8}, "end": {"line": 12, "character": 14}}
            ]
        }),
    );
}

// ─── Type Hierarchy ──────────────────────────────────────────────────────────

#[test]
fn job_type_hierarchy_prepare_params() {
    assert_serde_roundtrip(
        &TypeHierarchyPrepareParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: make_uri("file:///src/types.rs"),
                },
                position: Position::new(3, 12),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
        },
        json!({
            "textDocument": {"uri": "file:///src/types.rs"},
            "position": {"line": 3, "character": 12}
        }),
    );
}

#[test]
fn job_type_hierarchy_item() {
    assert_serde_roundtrip(
        &TypeHierarchyItem {
            name: "Animal".into(),
            kind: SymbolKind::CLASS,
            tags: None,
            detail: Some("trait Animal".into()),
            uri: make_uri("file:///src/types.rs"),
            range: make_range(0, 0, 5, 1),
            selection_range: make_range(0, 6, 0, 12),
            data: Some(json!({"typeId": "types::Animal"})),
        },
        json!({
            "name": "Animal",
            "kind": 5,
            "detail": "trait Animal",
            "uri": "file:///src/types.rs",
            "range": {"start": {"line": 0, "character": 0}, "end": {"line": 5, "character": 1}},
            "selectionRange": {"start": {"line": 0, "character": 6}, "end": {"line": 0, "character": 12}},
            "data": {"typeId": "types::Animal"}
        }),
    );
}

#[test]
fn job_type_hierarchy_supertypes_params() {
    let item = TypeHierarchyItem {
        name: "Dog".into(),
        kind: SymbolKind::CLASS,
        tags: None,
        detail: None,
        uri: make_uri("file:///src/types.rs"),
        range: make_range(10, 0, 15, 1),
        selection_range: make_range(10, 6, 10, 9),
        data: None,
    };
    assert_serde_roundtrip(
        &TypeHierarchySupertypesParams {
            item,
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({
            "item": {
                "name": "Dog",
                "kind": 5,
                "uri": "file:///src/types.rs",
                "range": {"start": {"line": 10, "character": 0}, "end": {"line": 15, "character": 1}},
                "selectionRange": {"start": {"line": 10, "character": 6}, "end": {"line": 10, "character": 9}}
            }
        }),
    );
}

#[test]
fn job_type_hierarchy_subtypes_params() {
    let item = TypeHierarchyItem {
        name: "Animal".into(),
        kind: SymbolKind::CLASS,
        tags: None,
        detail: None,
        uri: make_uri("file:///src/types.rs"),
        range: make_range(0, 0, 5, 1),
        selection_range: make_range(0, 6, 0, 12),
        data: None,
    };
    assert_serde_roundtrip(
        &TypeHierarchySubtypesParams {
            item,
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({
            "item": {
                "name": "Animal",
                "kind": 5,
                "uri": "file:///src/types.rs",
                "range": {"start": {"line": 0, "character": 0}, "end": {"line": 5, "character": 1}},
                "selectionRange": {"start": {"line": 0, "character": 6}, "end": {"line": 0, "character": 12}}
            }
        }),
    );
}

// ─── Semantic Tokens ─────────────────────────────────────────────────────────

#[test]
fn job_semantic_tokens_params() {
    assert_serde_roundtrip(
        &SemanticTokensParams {
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
        },
        json!({"textDocument": {"uri": "file:///src/main.rs"}}),
    );
}

#[test]
fn job_semantic_tokens_result_with_data() {
    assert_serde_roundtrip(
        &SemanticTokens {
            result_id: Some("tok-v1".into()),
            data: vec![
                SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: 2,
                    token_type: 1,
                    token_modifiers_bitset: 0,
                },
                SemanticToken {
                    delta_line: 1,
                    delta_start: 4,
                    length: 5,
                    token_type: 2,
                    token_modifiers_bitset: 3,
                },
            ],
        },
        json!({
            "resultId": "tok-v1",
            "data": [0, 0, 2, 1, 0,  1, 4, 5, 2, 3]
        }),
    );
}

#[test]
fn job_semantic_tokens_delta_params() {
    assert_serde_roundtrip(
        &SemanticTokensDeltaParams {
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            previous_result_id: "tok-v1".into(),
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "previousResultId": "tok-v1"
        }),
    );
}

#[test]
fn job_semantic_tokens_delta_with_edits() {
    assert_serde_roundtrip(
        &SemanticTokensDelta {
            result_id: Some("tok-v2".into()),
            edits: vec![SemanticTokensEdit {
                start: 5,
                delete_count: 5,
                data: Some(vec![SemanticToken {
                    delta_line: 1,
                    delta_start: 0,
                    length: 3,
                    token_type: 0,
                    token_modifiers_bitset: 0,
                }]),
            }],
        },
        json!({
            "resultId": "tok-v2",
            "edits": [{"start": 5, "deleteCount": 5, "data": [1, 0, 3, 0, 0]}]
        }),
    );
}

#[test]
fn job_semantic_tokens_edit_delete_only() {
    assert_serde_roundtrip(
        &SemanticTokensEdit {
            start: 10,
            delete_count: 5,
            data: None,
        },
        json!({"start": 10, "deleteCount": 5}),
    );
}

#[test]
fn job_semantic_tokens_range_params() {
    assert_serde_roundtrip(
        &SemanticTokensRangeParams {
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            range: make_range(0, 0, 50, 0),
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "range": {"start": {"line": 0, "character": 0}, "end": {"line": 50, "character": 0}}
        }),
    );
}

// ─── Inline Values ───────────────────────────────────────────────────────────

#[test]
fn job_inline_value_params() {
    assert_serde_roundtrip(
        &InlineValueParams {
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            range: make_range(0, 0, 20, 0),
            context: InlineValueContext {
                frame_id: 42,
                stopped_location: make_range(10, 0, 10, 30),
            },
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "range": {"start": {"line": 0, "character": 0}, "end": {"line": 20, "character": 0}},
            "context": {
                "frameId": 42,
                "stoppedLocation": {"start": {"line": 10, "character": 0}, "end": {"line": 10, "character": 30}}
            }
        }),
    );
}

#[test]
fn job_inline_value_text() {
    assert_serde_roundtrip(
        &InlineValue::Text(InlineValueText {
            range: make_range(5, 0, 5, 10),
            text: "x = 42".into(),
        }),
        json!({
            "range": {"start": {"line": 5, "character": 0}, "end": {"line": 5, "character": 10}},
            "text": "x = 42"
        }),
    );
}

#[test]
fn job_inline_value_variable_lookup() {
    assert_serde_roundtrip(
        &InlineValue::VariableLookup(InlineValueVariableLookup {
            range: make_range(3, 4, 3, 9),
            variable_name: Some("count".into()),
            case_sensitive_lookup: true,
        }),
        json!({
            "range": {"start": {"line": 3, "character": 4}, "end": {"line": 3, "character": 9}},
            "variableName": "count",
            "caseSensitiveLookup": true
        }),
    );
}

#[test]
fn job_inline_value_evaluatable_expression() {
    assert_serde_roundtrip(
        &InlineValue::EvaluatableExpression(InlineValueEvaluatableExpression {
            range: make_range(7, 0, 7, 15),
            expression: Some("items.length()".into()),
        }),
        json!({
            "range": {"start": {"line": 7, "character": 0}, "end": {"line": 7, "character": 15}},
            "expression": "items.length()"
        }),
    );
}

// ─── Inlay Hints ─────────────────────────────────────────────────────────────

#[test]
fn job_inlay_hint_params() {
    assert_serde_roundtrip(
        &InlayHintParams {
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            range: make_range(0, 0, 100, 0),
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "range": {"start": {"line": 0, "character": 0}, "end": {"line": 100, "character": 0}}
        }),
    );
}

#[test]
fn job_inlay_hint_with_label_parts_and_tooltip() {
    assert_serde_roundtrip(
        &InlayHint {
            position: Position::new(5, 20),
            label: InlayHintLabel::LabelParts(vec![
                InlayHintLabelPart {
                    value: ": i32".into(),
                    tooltip: Some(InlayHintLabelPartTooltip::String("i32 type annotation".into())),
                    location: Some(Location {
                        uri: make_uri("file:///std/i32.rs"),
                        range: make_range(0, 0, 0, 3),
                    }),
                    command: None,
                },
                InlayHintLabelPart {
                    value: " (derived)".into(),
                    tooltip: None,
                    location: None,
                    command: Some(Command {
                        title: "Go to derivation".into(),
                        command: "rust.go_to_derive".into(),
                        arguments: None,
                    }),
                },
            ]),
            kind: Some(InlayHintKind::TYPE),
            text_edits: None,
            tooltip: Some(InlayHintTooltip::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Type: **i32**".into(),
            })),
            padding_left: Some(false),
            padding_right: Some(true),
            data: None,
        },
        json!({
            "position": {"line": 5, "character": 20},
            "label": [
                {
                    "value": ": i32",
                    "tooltip": "i32 type annotation",
                    "location": {
                        "uri": "file:///std/i32.rs",
                        "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 3}}
                    }
                },
                {
                    "value": " (derived)",
                    "command": {
                        "title": "Go to derivation",
                        "command": "rust.go_to_derive"
                    }
                }
            ],
            "kind": 1,
            "tooltip": {"kind": "markdown", "value": "Type: **i32**"},
            "paddingLeft": false,
            "paddingRight": true
        }),
    );
}

#[test]
fn job_inlay_hint_kind_type() {
    assert_serde_roundtrip(&InlayHintKind::TYPE, json!(1));
}

#[test]
fn job_inlay_hint_kind_parameter() {
    assert_serde_roundtrip(&InlayHintKind::PARAMETER, json!(2));
}

// ─── Moniker ─────────────────────────────────────────────────────────────────

#[test]
fn job_moniker_params() {
    assert_serde_roundtrip(
        &MonikerParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: make_uri("file:///src/main.rs"),
                },
                position: Position::new(4, 6),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "position": {"line": 4, "character": 6}
        }),
    );
}

#[test]
fn job_moniker_import_kind() {
    assert_serde_roundtrip(
        &Moniker {
            scheme: "tsc".into(),
            identifier: "lib/std/fs:readFile".into(),
            unique: UniquenessLevel::Global,
            kind: Some(MonikerKind::Import),
        },
        json!({
            "scheme": "tsc",
            "identifier": "lib/std/fs:readFile",
            "unique": "global",
            "kind": "import"
        }),
    );
}

#[test]
fn job_moniker_export_kind() {
    assert_serde_roundtrip(
        &Moniker {
            scheme: "cargo".into(),
            identifier: "crate::util::helper".into(),
            unique: UniquenessLevel::Scheme,
            kind: Some(MonikerKind::Export),
        },
        json!({
            "scheme": "cargo",
            "identifier": "crate::util::helper",
            "unique": "scheme",
            "kind": "export"
        }),
    );
}

#[test]
fn job_moniker_local_kind() {
    assert_serde_roundtrip(
        &Moniker {
            scheme: "rust-analyzer".into(),
            identifier: "main::x".into(),
            unique: UniquenessLevel::Document,
            kind: Some(MonikerKind::Local),
        },
        json!({
            "scheme": "rust-analyzer",
            "identifier": "main::x",
            "unique": "document",
            "kind": "local"
        }),
    );
}

#[test]
fn job_moniker_uniqueness_level_all() {
    assert_serde_roundtrip(&UniquenessLevel::Document, json!("document"));
    assert_serde_roundtrip(&UniquenessLevel::Project, json!("project"));
    assert_serde_roundtrip(&UniquenessLevel::Group, json!("group"));
    assert_serde_roundtrip(&UniquenessLevel::Scheme, json!("scheme"));
    assert_serde_roundtrip(&UniquenessLevel::Global, json!("global"));
}

#[test]
fn job_moniker_no_kind() {
    assert_serde_roundtrip(
        &Moniker {
            scheme: "tsc".into(),
            identifier: "internal:symbol".into(),
            unique: UniquenessLevel::Project,
            kind: None,
        },
        json!({
            "scheme": "tsc",
            "identifier": "internal:symbol",
            "unique": "project"
        }),
    );
}

// ─── Diagnostics ─────────────────────────────────────────────────────────────

#[test]
fn job_document_diagnostic_params() {
    assert_serde_roundtrip(
        &DocumentDiagnosticParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/main.rs"),
            },
            identifier: Some("rust-analyzer".into()),
            previous_result_id: Some("diag-v1".into()),
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({
            "textDocument": {"uri": "file:///src/main.rs"},
            "identifier": "rust-analyzer",
            "previousResultId": "diag-v1"
        }),
    );
}

#[test]
fn job_document_diagnostic_report_full() {
    assert_serde_roundtrip(
        &DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
            related_documents: None,
            full_document_diagnostic_report: FullDocumentDiagnosticReport {
                result_id: Some("diag-v2".into()),
                items: vec![Diagnostic {
                    range: make_range(3, 0, 3, 10),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("E0308".into())),
                    message: "mismatched types".into(),
                    ..Default::default()
                }],
            },
        }),
        json!({
            "kind": "full",
            "resultId": "diag-v2",
            "items": [{
                "range": {"start": {"line": 3, "character": 0}, "end": {"line": 3, "character": 10}},
                "severity": 1,
                "code": "E0308",
                "message": "mismatched types"
            }]
        }),
    );
}

#[test]
fn job_document_diagnostic_report_unchanged() {
    assert_serde_roundtrip(
        &DocumentDiagnosticReport::Unchanged(RelatedUnchangedDocumentDiagnosticReport {
            related_documents: None,
            unchanged_document_diagnostic_report: UnchangedDocumentDiagnosticReport {
                result_id: "diag-v2".into(),
            },
        }),
        json!({
            "kind": "unchanged",
            "resultId": "diag-v2"
        }),
    );
}

#[test]
fn job_workspace_diagnostic_params() {
    assert_serde_roundtrip(
        &WorkspaceDiagnosticParams {
            identifier: Some("rust-analyzer".into()),
            previous_result_ids: vec![PreviousResultId {
                uri: make_uri("file:///src/main.rs"),
                value: "diag-v1".into(),
            }],
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({
            "identifier": "rust-analyzer",
            "previousResultIds": [{"uri": "file:///src/main.rs", "value": "diag-v1"}]
        }),
    );
}

#[test]
fn job_workspace_diagnostic_report() {
    assert_serde_roundtrip(
        &WorkspaceDiagnosticReport {
            items: vec![WorkspaceDocumentDiagnosticReport::Full(
                WorkspaceFullDocumentDiagnosticReport {
                    uri: make_uri("file:///src/main.rs"),
                    version: Some(3),
                    full_document_diagnostic_report: FullDocumentDiagnosticReport {
                        result_id: None,
                        items: vec![],
                    },
                },
            )],
        },
        json!({
            "items": [{
                "kind": "full",
                "uri": "file:///src/main.rs",
                "version": 3,
                "items": []
            }]
        }),
    );
}

// ─── Linked Editing ──────────────────────────────────────────────────────────

#[test]
fn job_linked_editing_range_params() {
    assert_serde_roundtrip(
        &LinkedEditingRangeParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: make_uri("file:///src/index.html"),
                },
                position: Position::new(5, 3),
            },
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
        },
        json!({
            "textDocument": {"uri": "file:///src/index.html"},
            "position": {"line": 5, "character": 3}
        }),
    );
}

#[test]
fn job_linked_editing_ranges_with_word_pattern() {
    assert_serde_roundtrip(
        &LinkedEditingRanges {
            ranges: vec![make_range(5, 1, 5, 5), make_range(5, 12, 5, 16)],
            word_pattern: Some("[a-z][a-zA-Z0-9]*".into()),
        },
        json!({
            "ranges": [
                {"start": {"line": 5, "character": 1}, "end": {"line": 5, "character": 5}},
                {"start": {"line": 5, "character": 12}, "end": {"line": 5, "character": 16}}
            ],
            "wordPattern": "[a-z][a-zA-Z0-9]*"
        }),
    );
}

// ─── Workspace ───────────────────────────────────────────────────────────────

#[test]
fn job_workspace_symbol_params_with_query() {
    assert_serde_roundtrip(
        &WorkspaceSymbolParams {
            query: "HashMap".into(),
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({"query": "HashMap"}),
    );
}

#[test]
fn job_workspace_symbol_response_nested_with_range_location() {
    // WorkspaceSymbolResponse::Nested and Flat share the same JSON shape when using
    // a Location with a range — the untagged enum matches Flat first on deserialization.
    // Test serialization only: verify the Nested variant serializes to the correct JSON.
    let val = WorkspaceSymbolResponse::Nested(vec![WorkspaceSymbol {
        name: "HashMap".into(),
        kind: SymbolKind::STRUCT,
        tags: None,
        container_name: Some("std::collections".into()),
        location: OneOf::Left(Location {
            uri: make_uri("file:///std/collections/hash_map.rs"),
            range: make_range(50, 0, 80, 1),
        }),
        data: None,
    }]);
    let serialized = serde_json::to_value(&val).expect("Failed to serialize");
    assert_eq!(
        serialized,
        json!([{
            "name": "HashMap",
            "kind": 23,
            "containerName": "std::collections",
            "location": {
                "uri": "file:///std/collections/hash_map.rs",
                "range": {"start": {"line": 50, "character": 0}, "end": {"line": 80, "character": 1}}
            }
        }]),
        "Serialized JSON does not match expected"
    );
}

#[test]
fn job_workspace_symbol_response_nested_with_workspace_location() {
    assert_serde_roundtrip(
        &WorkspaceSymbolResponse::Nested(vec![WorkspaceSymbol {
            name: "Vec".into(),
            kind: SymbolKind::STRUCT,
            tags: None,
            container_name: None,
            location: OneOf::Right(WorkspaceLocation {
                uri: make_uri("file:///std/vec.rs"),
            }),
            data: None,
        }]),
        json!([{
            "name": "Vec",
            "kind": 23,
            "location": {"uri": "file:///std/vec.rs"}
        }]),
    );
}

#[test]
fn job_execute_command_params_with_arguments() {
    assert_serde_roundtrip(
        &ExecuteCommandParams {
            command: "editor.action.formatDocument".into(),
            arguments: vec![json!({"insertSpaces": true, "tabSize": 4})],
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
        },
        json!({
            "command": "editor.action.formatDocument",
            "arguments": [{"insertSpaces": true, "tabSize": 4}]
        }),
    );
}

#[test]
fn job_apply_workspace_edit_params() {
    assert_serde_roundtrip(
        &ApplyWorkspaceEditParams {
            label: Some("Rename Symbol".into()),
            edit: WorkspaceEdit {
                changes: None,
                document_changes: None,
                ..Default::default()
            },
        },
        json!({
            "label": "Rename Symbol",
            "edit": {}
        }),
    );
}

#[test]
fn job_apply_workspace_edit_response_success() {
    assert_serde_roundtrip(
        &ApplyWorkspaceEditResponse {
            applied: true,
            failure_reason: None,
            failed_change: None,
        },
        json!({"applied": true}),
    );
}

#[test]
fn job_apply_workspace_edit_response_failure() {
    assert_serde_roundtrip(
        &ApplyWorkspaceEditResponse {
            applied: false,
            failure_reason: Some("File is read-only".into()),
            failed_change: Some(0),
        },
        json!({
            "applied": false,
            "failureReason": "File is read-only",
            "failedChange": 0
        }),
    );
}

// ─── Publish Diagnostics (notification) ──────────────────────────────────────

#[test]
fn job_publish_diagnostics_params_full_diagnostic() {
    assert_serde_roundtrip(
        &PublishDiagnosticsParams {
            uri: make_uri("file:///src/main.rs"),
            diagnostics: vec![Diagnostic {
                range: make_range(5, 4, 5, 15),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::Number(1001)),
                code_description: Some(CodeDescription {
                    href: make_uri("https://example.com/errors/E1001"),
                }),
                source: Some("my-linter".into()),
                message: "undefined variable 'foo'".into(),
                related_information: Some(vec![DiagnosticRelatedInformation {
                    location: Location {
                        uri: make_uri("file:///src/main.rs"),
                        range: make_range(1, 4, 1, 7),
                    },
                    message: "'foo' declared here".into(),
                }]),
                tags: Some(vec![DiagnosticTag::UNNECESSARY]),
                data: Some(json!({"fixAvailable": true})),
            }],
            version: Some(3),
        },
        json!({
            "uri": "file:///src/main.rs",
            "diagnostics": [{
                "range": {"start": {"line": 5, "character": 4}, "end": {"line": 5, "character": 15}},
                "severity": 1,
                "code": 1001,
                "codeDescription": {"href": "https://example.com/errors/E1001"},
                "source": "my-linter",
                "message": "undefined variable 'foo'",
                "relatedInformation": [{
                    "location": {
                        "uri": "file:///src/main.rs",
                        "range": {"start": {"line": 1, "character": 4}, "end": {"line": 1, "character": 7}}
                    },
                    "message": "'foo' declared here"
                }],
                "tags": [1],
                "data": {"fixAvailable": true}
            }],
            "version": 3
        }),
    );
}

// ─── Additional edge cases ────────────────────────────────────────────────────

#[test]
fn job_number_or_string_number_variant() {
    assert_serde_roundtrip(&NumberOrString::Number(0), json!(0));
}

#[test]
fn job_number_or_string_string_variant() {
    assert_serde_roundtrip(&NumberOrString::String("abc".into()), json!("abc"));
}

#[test]
fn job_completion_item_kind_text() {
    assert_serde_roundtrip(&CompletionItemKind::TEXT, json!(1));
}

#[test]
fn job_diagnostic_severity_all_values() {
    assert_serde_roundtrip(&DiagnosticSeverity::ERROR, json!(1));
    assert_serde_roundtrip(&DiagnosticSeverity::WARNING, json!(2));
    assert_serde_roundtrip(&DiagnosticSeverity::INFORMATION, json!(3));
    assert_serde_roundtrip(&DiagnosticSeverity::HINT, json!(4));
}

#[test]
fn job_text_document_sync_kind_values() {
    assert_serde_roundtrip(&TextDocumentSyncKind::NONE, json!(0));
    assert_serde_roundtrip(&TextDocumentSyncKind::FULL, json!(1));
    assert_serde_roundtrip(&TextDocumentSyncKind::INCREMENTAL, json!(2));
}

#[test]
fn job_completion_text_edit_insert_and_replace() {
    assert_serde_roundtrip(
        &CompletionTextEdit::InsertAndReplace(InsertReplaceEdit {
            new_text: "some_completion".into(),
            insert: make_range(3, 5, 3, 10),
            replace: make_range(3, 5, 3, 15),
        }),
        json!({
            "newText": "some_completion",
            "insert": {"start": {"line": 3, "character": 5}, "end": {"line": 3, "character": 10}},
            "replace": {"start": {"line": 3, "character": 5}, "end": {"line": 3, "character": 15}}
        }),
    );
}

#[test]
fn job_hover_contents_scalar_marked_string() {
    assert_serde_roundtrip(
        &Hover {
            contents: HoverContents::Scalar(MarkedString::LanguageString(LanguageString {
                language: "rust".into(),
                value: "fn hello() -> &str".into(),
            })),
            range: None,
        },
        json!({
            "contents": {"language": "rust", "value": "fn hello() -> &str"}
        }),
    );
}

#[test]
fn job_workspace_symbol_response_flat_symbol_information() {
    #[allow(deprecated)]
    assert_serde_roundtrip(
        &WorkspaceSymbolResponse::Flat(vec![SymbolInformation {
            name: "MyEnum".into(),
            kind: SymbolKind::ENUM,
            tags: None,
            deprecated: None,
            location: Location {
                uri: make_uri("file:///src/types.rs"),
                range: make_range(10, 0, 20, 1),
            },
            container_name: None,
        }]),
        json!([{
            "name": "MyEnum",
            "kind": 10,
            "location": {
                "uri": "file:///src/types.rs",
                "range": {"start": {"line": 10, "character": 0}, "end": {"line": 20, "character": 1}}
            }
        }]),
    );
}

#[test]
fn job_inlay_hint_string_label() {
    assert_serde_roundtrip(
        &InlayHint {
            position: Position::new(2, 18),
            label: InlayHintLabel::String(": String".into()),
            kind: Some(InlayHintKind::TYPE),
            text_edits: None,
            tooltip: None,
            padding_left: None,
            padding_right: None,
            data: None,
        },
        json!({
            "position": {"line": 2, "character": 18},
            "label": ": String",
            "kind": 1
        }),
    );
}

#[test]
fn job_semantic_tokens_empty_data() {
    assert_serde_roundtrip(
        &SemanticTokens {
            result_id: None,
            data: vec![],
        },
        json!({"data": []}),
    );
}

#[test]
fn job_document_diagnostic_params_no_previous_result() {
    assert_serde_roundtrip(
        &DocumentDiagnosticParams {
            text_document: TextDocumentIdentifier {
                uri: make_uri("file:///src/new_file.rs"),
            },
            identifier: None,
            previous_result_id: None,
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: PartialResultParams::default(),
        },
        json!({
            "textDocument": {"uri": "file:///src/new_file.rs"},
            "identifier": null,
            "previousResultId": null
        }),
    );
}
