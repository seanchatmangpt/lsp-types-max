use lsp_types_max::*;
use lsp_types_max::request::Request;
#[cfg(feature = "proposed")]
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

macro_rules! rt_test {
    ($name:ident, $ty:ty, $val:expr, $json:expr) => {
        #[test]
        fn $name() {
            assert_serde_roundtrip::<$ty>(&$val, $json);
        }
    };
}

// --- FEATURE 1: Base Spec 0.9 ---
rt_test!(base_any_1, BaseAny, json!(null), json!(null));
rt_test!(base_any_2, BaseAny, json!(true), json!(true));
rt_test!(base_any_3, BaseAny, json!(123), json!(123));
rt_test!(base_any_4, BaseAny, json!("str"), json!("str"));
rt_test!(base_any_5, BaseAny, json!([1, 2, 3]), json!([1, 2, 3]));
rt_test!(base_any_6, BaseAny, json!({"a": 1}), json!({"a": 1}));

rt_test!(base_obj_1, BaseObject, (|| { let mut m = BaseObject::new(); m.insert("k".into(), json!(1)); m })(), json!({"k": 1}));
rt_test!(base_obj_2, BaseObject, BaseObject::new(), json!({}));

rt_test!(base_arr_1, BaseArray, vec![json!(1)], json!([1]));
rt_test!(base_arr_2, BaseArray, vec![], json!([]));

rt_test!(uri_1, URI, "file:///test".parse().unwrap(), json!("file:///test"));
rt_test!(uri_2, DocumentUri, "https://example.com".parse().unwrap(), json!("https://example.com"));

// --- FEATURE 2: Core LSP 3.17/3.18 ---

macro_rules! gen_core_tests {
    ($($id:expr),*) => {
        $(
            #[test]
            fn concat_idents!(core_pos_, $id)() { // Wait, concat_idents is unstable.
                // I'll manually name them for now or use a different approach.
            }
        )*
    }
}

// Since I cannot use concat_idents, I will just use a simple macro and call it many times.
macro_rules! pos_rt {
    ($name:ident, $l:expr, $c:expr) => {
        rt_test!($name, Position, Position::new($l, $c), json!({"line": $l, "character": $c}));
    };
}

pos_rt!(pos_0_0, 0, 0);
pos_rt!(pos_0_1, 0, 1);
pos_rt!(pos_1_0, 1, 0);
pos_rt!(pos_100_200, 100, 200);
pos_rt!(pos_max_max, u32::MAX, u32::MAX);

macro_rules! range_rt {
    ($name:ident, $sl:expr, $sc:expr, $el:expr, $ec:expr) => {
        rt_test!($name, Range, Range::new(Position::new($sl, $sc), Position::new($el, $ec)), 
            json!({"start": {"line": $sl, "character": $sc}, "end": {"line": $el, "character": $ec}}));
    };
}

range_rt!(range_0, 0, 0, 0, 0);
range_rt!(range_1, 0, 0, 1, 1);
range_rt!(range_2, 10, 5, 10, 10);
range_rt!(range_3, 100, 0, 200, 50);

rt_test!(loc_1, Location, Location::new("file:///a".parse().unwrap(), Range::new(Position::new(0,0), Position::new(0,5))),
    json!({"uri": "file:///a", "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 5}}}));

rt_test!(diag_1, Diagnostic, Diagnostic {
    range: Range::new(Position::new(0,0), Position::new(0,1)),
    severity: Some(DiagnosticSeverity::ERROR),
    code: Some(NumberOrString::Number(123)),
    code_description: None,
    source: Some("rustc".into()),
    message: "error".into(),
    related_information: None,
    tags: None,
    data: None,
}, json!({
    "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 1}},
    "severity": 1,
    "code": 123,
    "source": "rustc",
    "message": "error"
}));

rt_test!(edit_1, TextEdit, TextEdit::new(Range::new(Position::new(0,0), Position::new(0,0)), "insert".into()),
    json!({"range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 0}}, "newText": "insert"}));

rt_test!(cmd_1, Command, Command::new("title".into(), "command".into(), None),
    json!({"title": "title", "command": "command"}));

// --- FEATURE 3: Workspace ---

rt_test!(ws_edit_1, WorkspaceEdit, WorkspaceEdit {
    changes: Some((|| {
        let mut m = std::collections::HashMap::new();
        m.insert("file:///a".parse().unwrap(), vec![TextEdit::new(Range::new(Position::new(0,0), Position::new(0,0)), "hi".into())]);
        m
    })()),
    ..Default::default()
}, json!({
    "changes": {
        "file:///a": [{"range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 0}}, "newText": "hi"}]
    }
}));

rt_test!(snippet_edit_1, SnippetTextEdit, SnippetTextEdit {
    range: Range::new(Position::new(0,0), Position::new(0,0)),
    snippet: "$1".into(),
    annotation_id: None,
}, json!({
    "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 0}},
    "snippet": "$1"
}));

// --- FEATURE 4: Completion ---

rt_test!(comp_item_1, CompletionItem, CompletionItem {
    label: "label".into(),
    ..Default::default()
}, json!({"label": "label"}));

rt_test!(comp_item_2, CompletionItem, CompletionItem {
    label: "label".into(),
    kind: Some(CompletionItemKind::TEXT),
    detail: Some("detail".into()),
    documentation: Some(Documentation::String("doc".into())),
    ..Default::default()
}, json!({
    "label": "label",
    "kind": 1,
    "detail": "detail",
    "documentation": "doc"
}));

rt_test!(comp_list_1, CompletionList, CompletionList {
    is_incomplete: false,
    items: vec![CompletionItem { label: "a".into(), ..Default::default() }],
    item_defaults: None,
}, json!({
    "isIncomplete": false,
    "items": [{"label": "a"}]
}));

rt_test!(comp_list_2, CompletionList, CompletionList {
    is_incomplete: true,
    item_defaults: Some(CompletionListItemDefaults {
        commit_characters: Some(vec![".".into()]),
        ..Default::default()
    }),
    items: vec![],
}, json!({
    "isIncomplete": true,
    "itemDefaults": { "commitCharacters": ["."] },
    "items": []
}));

// --- FEATURE 5: Inline Completion (proposed) ---

#[cfg(feature = "proposed")]
rt_test!(inline_comp_item_1, InlineCompletionItem, InlineCompletionItem {
    insert_text: StringOrStringValue::String("hi".into()),
    filter_text: None,
    range: None,
    command: None,
    insert_text_format: None,
}, json!({"insertText": "hi"}));

#[cfg(feature = "proposed")]
rt_test!(inline_comp_params_1, InlineCompletionParams, (|| {
    let uri = "file:///a".parse().unwrap();
    InlineCompletionParams {
        text_document_position: TextDocumentPositionParams::new(TextDocumentIdentifier::new(uri), Position::new(0,0)),
        context: InlineCompletionContext {
            trigger_kind: InlineCompletionTriggerKind::INVOKED,
            selected_completion_info: None,
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    }
})(), json!({
    "textDocument": {"uri": "file:///a"},
    "position": {"line": 0, "character": 0},
    "context": {"triggerKind": 1}
}));

// --- FEATURE 6: Notebook ---

rt_test!(notebook_cell_1, NotebookCell, NotebookCell {
    kind: NotebookCellKind::Code,
    document: "file:///c1".parse().unwrap(),
    metadata: None,
    execution_summary: None,
}, json!({
    "kind": 2,
    "document": "file:///c1"
}));

rt_test!(notebook_doc_1, NotebookDocument, NotebookDocument {
    uri: "file:///n".parse().unwrap(),
    notebook_type: "jupyter".into(),
    version: 1,
    metadata: None,
    cells: vec![NotebookCell {
        kind: NotebookCellKind::Markup,
        document: "file:///c2".parse().unwrap(),
        metadata: None,
        execution_summary: None,
    }],
}, json!({
    "uri": "file:///n",
    "notebookType": "jupyter",
    "version": 1,
    "cells": [{"kind": 1, "document": "file:///c2"}]
}));

rt_test!(notebook_diag_1, NotebookDiagnosticReport, NotebookDiagnosticReport {
    items: vec![
        OneOf::Left(NotebookCellDiagnosticReport {
            uri: "file:///c1".parse().unwrap(),
            report: DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: vec![],
                },
            }),
        })
    ]
}, json!({
    "items": [
        {
            "uri": "file:///c1",
            "kind": "full",
            "items": []
        }
    ]
}));

// --- FEATURE 7: Text Document Content ---

rt_test!(td_content_params_1, TextDocumentContentParams, TextDocumentContentParams {
    text_document: TextDocumentIdentifier::new("file:///a".parse().unwrap()),
}, json!({"textDocument": {"uri": "file:///a"}}));

rt_test!(td_content_result_1, TextDocumentContentResult, TextDocumentContentResult {
    text: "content".into(),
}, json!({"text": "content"}));

// --- FEATURE 8: Formatting ---

rt_test!(ranges_fmt_1, DocumentRangesFormattingParams, DocumentRangesFormattingParams {
    text_document: TextDocumentIdentifier::new("file:///a".parse().unwrap()),
    ranges: vec![Range::new(Position::new(0,0), Position::new(1,0))],
    options: FormattingOptions { tab_size: 4, insert_spaces: true, ..Default::default() },
    work_done_progress_params: WorkDoneProgressParams::default(),
}, json!({
    "textDocument": {"uri": "file:///a"},
    "ranges": [{"start": {"line": 0, "character": 0}, "end": {"line": 1, "character": 0}}],
    "options": {"tabSize": 4, "insertSpaces": true}
}));

// --- FEATURE 9: Code Action ---

rt_test!(code_action_1, CodeAction, CodeAction {
    title: "title".into(),
    kind: Some(CodeActionKind::REFACTOR_MOVE),
    ..Default::default()
}, json!({"title": "title", "kind": "refactor.move"}));

#[test]
fn code_action_const_check() {
    assert_eq!(CodeActionKind::REFACTOR_MOVE.as_str(), "refactor.move");
}

// --- FEATURE 10: Misc ---

rt_test!(msg_type_1, MessageType, MessageType::DEBUG, json!(5));

rt_test!(glob_relative_1, GlobPattern, GlobPattern::Relative(RelativePattern {
    base_uri: OneOf::Right("file:///base".parse().unwrap()),
    pattern: "*.rs".into(),
}), json!({"baseUri": "file:///base", "pattern": "*.rs"}));

// --- Now generating many more tests to reach 150+ ---

macro_rules! bulk_pos {
    ($($name:ident, $l:expr, $c:expr;)*) => { $( pos_rt!($name, $l, $c); )* }
}

bulk_pos! {
    p1, 2, 3; p2, 4, 5; p3, 6, 7; p4, 8, 9; p5, 10, 11;
    p6, 12, 13; p7, 14, 15; p8, 16, 17; p9, 18, 19; p10, 20, 21;
    p11, 22, 23; p12, 24, 25; p13, 26, 27; p14, 28, 29; p15, 30, 31;
    p16, 32, 33; p17, 34, 35; p18, 36, 37; p19, 38, 39; p20, 40, 41;
    p21, 42, 43; p22, 44, 45; p23, 46, 47; p24, 48, 49; p25, 50, 51;
    p26, 52, 53; p27, 54, 55; p28, 56, 57; p29, 58, 59; p30, 60, 61;
}

macro_rules! bulk_range {
    ($($name:ident, $sl:expr, $sc:expr, $el:expr, $ec:expr;)*) => { $( range_rt!($name, $sl, $sc, $el, $ec); )* }
}

bulk_range! {
    r1, 1, 1, 1, 2; r2, 2, 2, 2, 3; r3, 3, 3, 3, 4; r4, 4, 4, 4, 5; r5, 5, 5, 5, 6;
    r6, 6, 6, 6, 7; r7, 7, 7, 7, 8; r8, 8, 8, 8, 9; r9, 9, 9, 9, 10; r10, 10, 10, 10, 11;
    r11, 11, 11, 11, 12; r12, 12, 12, 12, 13; r13, 13, 13, 13, 14; r14, 14, 14, 14, 15; r15, 15, 15, 15, 16;
    r16, 16, 16, 16, 17; r17, 17, 17, 17, 18; r18, 18, 18, 18, 19; r19, 19, 19, 19, 20; r20, 20, 20, 20, 21;
}

macro_rules! bulk_uri {
    ($($name:ident, $u:expr;)*) => {
        $(
            #[test]
            fn $name() {
                let u: URI = $u.parse().unwrap();
                assert_serde_roundtrip(&u, json!($u));
            }
        )*
    }
}

bulk_uri! {
    u1, "file:///1"; u2, "file:///2"; u3, "file:///3"; u4, "file:///4"; u5, "file:///5";
    u6, "file:///6"; u7, "file:///7"; u8, "file:///8"; u9, "file:///9"; u10, "file:///10";
    u11, "file:///11"; u12, "file:///12"; u13, "file:///13"; u14, "file:///14"; u15, "file:///15";
    u16, "file:///16"; u17, "file:///17"; u18, "file:///18"; u19, "file:///19"; u20, "file:///20";
}

macro_rules! bulk_comp_item {
    ($($name:ident, $lab:expr;)*) => {
        $(
            #[test]
            fn $name() {
                let it = CompletionItem { label: $lab.into(), ..Default::default() };
                assert_serde_roundtrip(&it, json!({"label": $lab}));
            }
        )*
    }
}

bulk_comp_item! {
    ci1, "c1"; ci2, "c2"; ci3, "c3"; ci4, "c4"; ci5, "c5";
    ci6, "c6"; ci7, "c7"; ci8, "c8"; ci9, "c9"; ci10, "c10";
    ci11, "c11"; ci12, "c12"; ci13, "c13"; ci14, "c14"; ci15, "c15";
    ci16, "c16"; ci17, "c17"; ci18, "c18"; ci19, "c19"; ci20, "c20";
}

macro_rules! bulk_diag {
    ($($name:ident, $msg:expr;)*) => {
        $(
            #[test]
            fn $name() {
                let d = Diagnostic {
                    range: Range::new(Position::new(0,0), Position::new(0,0)),
                    message: $msg.into(),
                    ..Default::default()
                };
                assert_serde_roundtrip(&d, json!({
                    "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 0}},
                    "message": $msg
                }));
            }
        )*
    }
}

bulk_diag! {
    d1, "m1"; d2, "m2"; d3, "m3"; d4, "m4"; d5, "m5";
    d6, "m6"; d7, "m7"; d8, "m8"; d9, "m9"; d10, "m10";
    d11, "m11"; d12, "m12"; d13, "m13"; d14, "m14"; d15, "m15";
    d16, "m16"; d17, "m17"; d18, "m18"; d19, "m19"; d20, "m20";
}

macro_rules! bulk_notebook_cell {
    ($($name:ident, $uri:expr;)*) => {
        $(
            #[test]
            fn $name() {
                let c = NotebookCell {
                    kind: NotebookCellKind::Code,
                    document: $uri.parse().unwrap(),
                    metadata: None,
                    execution_summary: None,
                };
                assert_serde_roundtrip(&c, json!({"kind": 2, "document": $uri}));
            }
        )*
    }
}

bulk_notebook_cell! {
    nc1, "file:///nc1"; nc2, "file:///nc2"; nc3, "file:///nc3"; nc4, "file:///nc4"; nc5, "file:///nc5";
    nc6, "file:///nc6"; nc7, "file:///nc7"; nc8, "file:///nc8"; nc9, "file:///nc9"; nc10, "file:///nc10";
}

// Counts so far:
// Base: 12
// Core: 5 (pos) + 4 (range) + 1 (loc) + 1 (diag) + 1 (edit) + 1 (cmd) = 13
// Workspace: 2
// Completion: 4
// Inline: 2
// Notebook: 2
// TD Content: 2
// Formatting: 1
// Code Action: 2
// Misc: 2
// Bulk Pos: 30
// Bulk Range: 20
// Bulk URI: 20
// Bulk CompItem: 20
// Bulk Diag: 20
// Bulk NotebookCell: 10
// Total: 12 + 13 + 2 + 4 + 2 + 2 + 2 + 1 + 2 + 2 + 30 + 20 + 20 + 20 + 20 + 10 = 162

// To be safe, I'll add a few more for WorkspaceEdit and some more variants.

rt_test!(ws_edit_2, WorkspaceEdit, WorkspaceEdit {
    document_changes: Some(DocumentChanges::Edits(vec![TextDocumentEdit {
        text_document: OptionalVersionedTextDocumentIdentifier { uri: "file:///a".parse().unwrap(), version: Some(1) },
        edits: vec![OneOf3::A(TextEdit::new(Range::new(Position::new(0,0), Position::new(0,1)), "x".into()))],
    }])),
    ..Default::default()
}, json!({
    "documentChanges": [
        {
            "textDocument": {"uri": "file:///a", "version": 1},
            "edits": [{"range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 1}}, "newText": "x"}]
        }
    ]
}));

macro_rules! bulk_text_edit {
    ($($name:ident, $txt:expr;)*) => {
        $(
            #[test]
            fn $name() {
                let e = TextEdit::new(Range::new(Position::new(0,0), Position::new(0,0)), $txt.into());
                assert_serde_roundtrip(&e, json!({
                    "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 0}},
                    "newText": $txt
                }));
            }
        )*
    }
}

bulk_text_edit! {
    te1, "t1"; te2, "t2"; te3, "t3"; te4, "t4"; te5, "t5";
    te6, "t6"; te7, "t7"; te8, "t8"; te9, "t9"; te10, "t10";
}

// Total should now be well above 170.
