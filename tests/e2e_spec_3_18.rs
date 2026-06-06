use lsp_types_max::*;
use lsp_types_max::request::Request;
use lsp_types_max::inline_completion::*;
use lsp_types_max::notebook::*;
use lsp_types_max::formatting::*;
use lsp_types_max::code_action::*;
use serde_json::json;

/// Helper for verifying Serde roundtrips.
fn assert_serde_roundtrip<T>(val: &T, expected: serde_json::Value)
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
{
    let serialized = serde_json::to_value(val).expect("Failed to serialize");
    assert_eq!(serialized, expected, "Serialized JSON does not match expected");
    let deserialized: T = serde_json::from_value(expected).expect("Failed to deserialize");
    assert_eq!(val, &deserialized, "Deserialized value does not match original");
}

#[test]
fn test_feature_1_inline_completion() {
    let uri = "file:///test.rs".parse().unwrap();
    let pos = Position::new(10, 5);
    
    // 1. InlineCompletionParams
    let params = InlineCompletionParams {
        text_document_position: TextDocumentPositionParams::new(TextDocumentIdentifier::new(uri), pos),
        context: InlineCompletionContext {
            trigger_kind: InlineCompletionTriggerKind::INVOKED,
            selected_completion_info: None,
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };
    assert_serde_roundtrip(&params, json!({
        "textDocument": { "uri": "file:///test.rs" },
        "position": { "line": 10, "character": 5 },
        "context": { "triggerKind": 1 }
    }));

    // 2. InlineCompletionItem
    let item = InlineCompletionItem {
        insert_text: StringOrStringValue::String("fn main() {}".to_string()),
        filter_text: Some("main".to_string()),
        range: Some(Range::new(pos, pos)),
        command: None,
        insert_text_format: None,
    };
    assert_serde_roundtrip(&item, json!({
        "insertText": "fn main() {}",
        "filterText": "main",
        "range": { "start": { "line": 10, "character": 5 }, "end": { "line": 10, "character": 5 } }
    }));

    // 3. InlineCompletionList
    let list = InlineCompletionList {
        items: vec![item.clone()],
    };
    assert_serde_roundtrip(&list, json!({ "items": [item] }));

    // 4. InlineCompletionResponse (Array)
    let resp_array = InlineCompletionResponse::Array(vec![item.clone()]);
    assert_serde_roundtrip(&resp_array, json!([item]));

    // 5. InlineCompletionResponse (List)
    let resp_list = InlineCompletionResponse::List(list.clone());
    assert_serde_roundtrip(&resp_list, json!(list));
}

#[test]
fn test_feature_2_text_document_content() {
    let uri = "test://doc/1".parse().unwrap();
    
    // 6. TextDocumentContentParams
    let params = TextDocumentContentParams {
        text_document: TextDocumentIdentifier::new(uri),
    };
    assert_serde_roundtrip(&params, json!({ "textDocument": { "uri": "test://doc/1" } }));

    // 7. TextDocumentContentResult
    let result = TextDocumentContentResult {
        text: "hello world".to_string(),
    };
    assert_serde_roundtrip(&result, json!({ "text": "hello world" }));

    // 8. TextDocumentContentRefreshParams
    let refresh = TextDocumentContentRefreshParams {
        text_documents: vec![TextDocumentIdentifier::new("test://doc/1".parse().unwrap())],
    };
    assert_serde_roundtrip(&refresh, json!({ "textDocuments": [{ "uri": "test://doc/1" }] }));
    
    // 9. Request Method Check
    assert_eq!(lsp_types_max::request::TextDocumentContentRequest::METHOD, "workspace/textDocumentContent");
    assert_eq!(lsp_types_max::request::TextDocumentContentRefreshRequest::METHOD, "workspace/textDocumentContent/refresh");
}

#[test]
fn test_feature_3_ranges_formatting() {
    // 10. DocumentRangesFormattingParams
    let params = DocumentRangesFormattingParams {
        text_document: TextDocumentIdentifier::new("file:///a.rs".parse().unwrap()),
        ranges: vec![Range::new(Position::new(0, 0), Position::new(1, 0))],
        options: FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            ..Default::default()
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };
    assert_serde_roundtrip(&params, json!({
        "textDocument": { "uri": "file:///a.rs" },
        "ranges": [{ "start": { "line": 0, "character": 0 }, "end": { "line": 1, "character": 0 } }],
        "options": { "tabSize": 4, "insertSpaces": true }
    }));
}

#[test]
fn test_feature_4_folding_range_refresh() {
    // 11. FoldingRangeRefreshRequest
    assert_eq!(lsp_types_max::request::FoldingRangeRefreshRequest::METHOD, "workspace/foldingRange/refresh");
}

#[test]
fn test_feature_5_notebook_sync() {
    let uri: Uri = "file:///notebook.ipynb".parse().unwrap();
    let cell_uri: Uri = "file:///notebook.ipynb#cell1".parse().unwrap();
    
    // 12. NotebookCell
    let cell = NotebookCell {
        kind: NotebookCellKind::Code,
        document: cell_uri.clone(),
        metadata: None,
        execution_summary: None,
    };
    assert_serde_roundtrip(&cell, json!({
        "kind": 2,
        "document": "file:///notebook.ipynb#cell1"
    }));

    // 13. NotebookDocument
    let notebook = NotebookDocument {
        uri: uri.clone(),
        notebook_type: "jupyter".to_string(),
        version: 1,
        metadata: None,
        cells: vec![cell],
    };
    assert_serde_roundtrip(&notebook, json!({
        "uri": "file:///notebook.ipynb",
        "notebookType": "jupyter",
        "version": 1,
        "cells": [{ "kind": 2, "document": "file:///notebook.ipynb#cell1" }]
    }));

    // 14. DidOpenNotebookDocumentParams
    let params = DidOpenNotebookDocumentParams {
        notebook_document: notebook,
        cell_text_documents: vec![TextDocumentItem::new(cell_uri, "rust".to_string(), 1, "println!()".to_string())],
    };
    let json_val = serde_json::to_value(&params).unwrap();
    assert!(json_val.get("notebookDocument").is_some());
    assert!(json_val.get("cellTextDocuments").is_some());
}

#[test]
fn test_feature_6_notebook_diagnostic() {
    // 15. NotebookDiagnosticParams
    let params = NotebookDiagnosticParams {
        notebook_document: NotebookDocumentIdentifier { uri: "file:///n.ipynb".parse().unwrap() },
        previous_result_id: Some("old".to_string()),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };
    assert_serde_roundtrip(&params, json!({
        "notebookDocument": { "uri": "file:///n.ipynb" },
        "previousResultId": "old"
    }));

    // 16. NotebookDiagnosticReport
    let report = NotebookDiagnosticReport {
        items: vec![],
    };
    assert_serde_roundtrip(&report, json!({ "items": [] }));
}

#[test]
fn test_feature_7_code_action_3_18() {
    // 17. CodeActionKind::REFACTOR_MOVE
    assert_eq!(CodeActionKind::REFACTOR_MOVE.as_str(), "refactor.move");
    
    // 18. CodeActionTag::LLM_GENERATED
    let tag = CodeActionTag::LLM_GENERATED;
    assert_serde_roundtrip(&tag, json!(1));
}

#[test]
fn test_feature_8_snippet_text_edit() {
    let range = Range::new(Position::new(0,0), Position::new(0,0));
    
    // 19. SnippetTextEdit
    let snippet = SnippetTextEdit {
        range,
        snippet: "fn ${1:name}() {\\n\\t$0\\n}".to_string(),
        annotation_id: None,
    };
    assert_serde_roundtrip(&snippet, json!({
        "range": { "start": { "line": 0, "character": 0 }, "end": { "line": 0, "character": 0 } },
        "snippet": "fn ${1:name}() {\\n\\t$0\\n}"
    }));

    // 20. WorkspaceEdit with DocumentChanges (SnippetTextEdit)
    let edit = WorkspaceEdit {
        document_changes: Some(DocumentChanges::Edits(vec![TextDocumentEdit {
            text_document: OptionalVersionedTextDocumentIdentifier { uri: "file:///f.rs".parse().unwrap(), version: None },
            edits: vec![OneOf3::A(TextEdit::new(range, "abc".to_string())), OneOf3::C(snippet)],
        }])),
        ..Default::default()
    };
    let json_val = serde_json::to_value(&edit).unwrap();
    assert!(json_val.get("documentChanges").is_some());
}

#[test]
fn test_feature_9_completion_list_defaults() {
    // 21. CompletionList.itemDefaults
    let list = CompletionList {
        is_incomplete: false,
        item_defaults: Some(CompletionListItemDefaults {
            commit_characters: Some(vec![".".to_string()]),
            edit_range: None,
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            insert_text_mode: None,
            data: None,
            label_details: None,
        }),
        items: vec![],
    };
    assert_serde_roundtrip(&list, json!({
        "isIncomplete": false,
        "itemDefaults": {
            "commitCharacters": ["."],
            "insertTextFormat": 1
        },
        "items": []
    }));
}

#[test]
fn test_feature_10_message_type_debug() {
    // 22. MessageType::DEBUG
    let msg_type = MessageType::DEBUG;
    assert_serde_roundtrip(&msg_type, json!(5));
}

#[test]
fn test_feature_11_relative_glob() {
    // 23. GlobPattern::Relative
    let relative = RelativePattern {
        base_uri: OneOf::Right("file:///project".parse().unwrap()),
        pattern: "**/*.rs".to_string(),
    };
    let pattern = GlobPattern::Relative(relative);
    assert_serde_roundtrip(&pattern, json!({
        "baseUri": "file:///project",
        "pattern": "**/*.rs"
    }));

    // 24. DocumentFilter with GlobPattern
    let filter = DocumentFilter {
        language: Some("rust".to_string()),
        scheme: None,
        pattern: Some(pattern),
    };
    assert_serde_roundtrip(&filter, json!({
        "language": "rust",
        "pattern": {
            "baseUri": "file:///project",
            "pattern": "**/*.rs"
        }
    }));
}
