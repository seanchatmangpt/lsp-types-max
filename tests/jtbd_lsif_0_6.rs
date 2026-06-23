/// Jobs-To-Be-Done tests for LSIF 0.6 types.
///
/// Each test represents a real indexing scenario a language server or build tool needs to emit.
/// Based on <https://microsoft.github.io/language-server-protocol/specifications/lsif/0.6.0/specification/>
use lsp_types_max::lsif::*;
use lsp_types_max::{
    Diagnostic, DiagnosticSeverity, DocumentLink, FoldingRange, Hover, HoverContents,
    LanguageString, MarkedString, Moniker, MonikerKind, NumberOrString, Position, Range,
    SymbolKind, UniquenessLevel,
};
use serde_json::json;

fn assert_entry_roundtrip(entry: &Entry, expected: serde_json::Value) {
    let serialized = serde_json::to_value(entry).expect("Failed to serialize Entry");
    assert_eq!(serialized, expected, "Serialized LSIF entry does not match expected");
    let deserialized: Entry = serde_json::from_value(expected).expect("Failed to deserialize Entry");
    assert_eq!(entry, &deserialized, "Deserialized LSIF entry does not match original");
}

fn assert_vertex_roundtrip(vid: i32, vertex: Vertex, expected: serde_json::Value) {
    let entry = Entry {
        id: NumberOrString::Number(vid),
        data: Element::Vertex(vertex),
    };
    assert_entry_roundtrip(&entry, expected);
}

fn assert_edge_roundtrip(eid: i32, edge: Edge, expected: serde_json::Value) {
    let entry = Entry {
        id: NumberOrString::Number(eid),
        data: Element::Edge(edge),
    };
    assert_entry_roundtrip(&entry, expected);
}

// ─── Job: Emit LSIF index metadata (every valid LSIF file starts here) ───────────────────────

#[test]
fn job_emit_lsif_metadata_vertex() {
    assert_vertex_roundtrip(
        1,
        Vertex::MetaData(MetaData {
            version: "0.6.0".into(),
            project_root: "file:///workspace".parse().unwrap(),
            position_encoding: Encoding::Utf16,
            tool_info: None,
        }),
        json!({
            "id": 1,
            "type": "vertex",
            "label": "metaData",
            "version": "0.6.0",
            "projectRoot": "file:///workspace",
            "positionEncoding": "utf-16"
        }),
    );
}

#[test]
fn job_emit_lsif_metadata_with_tool_info() {
    assert_vertex_roundtrip(
        1,
        Vertex::MetaData(MetaData {
            version: "0.6.0".into(),
            project_root: "file:///workspace".parse().unwrap(),
            position_encoding: Encoding::Utf8,
            tool_info: Some(ToolInfo {
                name: "rust-analyzer".into(),
                args: vec!["--project".into(), "Cargo.toml".into()],
                version: Some("0.3.1948".into()),
            }),
        }),
        json!({
            "id": 1,
            "type": "vertex",
            "label": "metaData",
            "version": "0.6.0",
            "projectRoot": "file:///workspace",
            "positionEncoding": "utf-8",
            "toolInfo": {
                "name": "rust-analyzer",
                "args": ["--project", "Cargo.toml"],
                "version": "0.3.1948"
            }
        }),
    );
}

#[test]
fn job_emit_utf32_encoding_metadata() {
    assert_vertex_roundtrip(
        1,
        Vertex::MetaData(MetaData {
            version: "0.6.0".into(),
            project_root: "file:///workspace".parse().unwrap(),
            position_encoding: Encoding::Utf32,
            tool_info: None,
        }),
        json!({
            "id": 1,
            "type": "vertex",
            "label": "metaData",
            "version": "0.6.0",
            "projectRoot": "file:///workspace",
            "positionEncoding": "utf-32"
        }),
    );
}

// ─── Job: Declare the indexing project ───────────────────────────────────────────────────────

#[test]
fn job_emit_project_vertex() {
    assert_vertex_roundtrip(
        2,
        Vertex::Project(Project {
            resource: None,
            content: None,
            kind: "rust".into(),
        }),
        json!({
            "id": 2,
            "type": "vertex",
            "label": "project",
            "kind": "rust"
        }),
    );
}

#[test]
fn job_emit_project_with_resource_uri() {
    assert_vertex_roundtrip(
        2,
        Vertex::Project(Project {
            resource: Some("file:///workspace/Cargo.toml".parse().unwrap()),
            content: None,
            kind: "rust".into(),
        }),
        json!({
            "id": 2,
            "type": "vertex",
            "label": "project",
            "resource": "file:///workspace/Cargo.toml",
            "kind": "rust"
        }),
    );
}

// ─── Job: Begin and end project scope events ─────────────────────────────────────────────────

#[test]
fn job_emit_project_begin_event() {
    assert_vertex_roundtrip(
        3,
        Vertex::Event(Event {
            kind: EventKind::Begin,
            scope: EventScope::Project,
            data: NumberOrString::Number(2),
        }),
        json!({
            "id": 3,
            "type": "vertex",
            "label": "$event",
            "kind": "begin",
            "scope": "project",
            "data": 2
        }),
    );
}

#[test]
fn job_emit_project_end_event() {
    assert_vertex_roundtrip(
        99,
        Vertex::Event(Event {
            kind: EventKind::End,
            scope: EventScope::Project,
            data: NumberOrString::Number(2),
        }),
        json!({
            "id": 99,
            "type": "vertex",
            "label": "$event",
            "kind": "end",
            "scope": "project",
            "data": 2
        }),
    );
}

// ─── Job: Index a source file (document vertex + begin/end events) ────────────────────────────

#[test]
fn job_emit_document_vertex() {
    assert_vertex_roundtrip(
        4,
        Vertex::Document(Document {
            uri: "file:///workspace/src/main.rs".parse().unwrap(),
            language_id: "rust".into(),
        }),
        json!({
            "id": 4,
            "type": "vertex",
            "label": "document",
            "uri": "file:///workspace/src/main.rs",
            "languageId": "rust"
        }),
    );
}

#[test]
fn job_emit_document_begin_event() {
    assert_vertex_roundtrip(
        5,
        Vertex::Event(Event {
            kind: EventKind::Begin,
            scope: EventScope::Document,
            data: NumberOrString::Number(4),
        }),
        json!({
            "id": 5,
            "type": "vertex",
            "label": "$event",
            "kind": "begin",
            "scope": "document",
            "data": 4
        }),
    );
}

#[test]
fn job_emit_document_end_event() {
    assert_vertex_roundtrip(
        50,
        Vertex::Event(Event {
            kind: EventKind::End,
            scope: EventScope::Document,
            data: NumberOrString::Number(4),
        }),
        json!({
            "id": 50,
            "type": "vertex",
            "label": "$event",
            "kind": "end",
            "scope": "document",
            "data": 4
        }),
    );
}

// ─── Job: Create a result set for a symbol ───────────────────────────────────────────────────

#[test]
fn job_emit_result_set_vertex() {
    assert_vertex_roundtrip(
        6,
        Vertex::ResultSet(ResultSet { key: None }),
        json!({"id": 6, "type": "vertex", "label": "resultSet"}),
    );
}

#[test]
fn job_emit_result_set_with_key() {
    assert_vertex_roundtrip(
        6,
        Vertex::ResultSet(ResultSet { key: Some("my-symbol-key".into()) }),
        json!({
            "id": 6,
            "type": "vertex",
            "label": "resultSet",
            "key": "my-symbol-key"
        }),
    );
}

// ─── Job: Create a range for a symbol definition ─────────────────────────────────────────────

#[test]
fn job_emit_bare_range_vertex() {
    assert_vertex_roundtrip(
        7,
        Vertex::Range {
            range: Range {
                start: Position::new(10, 4),
                end: Position::new(10, 11),
            },
            tag: None,
        },
        json!({
            "id": 7,
            "type": "vertex",
            "label": "range",
            "start": {"line": 10, "character": 4},
            "end": {"line": 10, "character": 11}
        }),
    );
}

#[test]
fn job_emit_definition_range_with_tag() {
    assert_vertex_roundtrip(
        7,
        Vertex::Range {
            range: Range {
                start: Position::new(10, 3),
                end: Position::new(10, 8),
            },
            tag: Some(RangeTag::Definition(DefinitionTag {
                text: "greet".into(),
                kind: SymbolKind::FUNCTION,
                deprecated: false,
                full_range: Range {
                    start: Position::new(10, 0),
                    end: Position::new(12, 1),
                },
                detail: Some("fn greet(name: &str)".into()),
            })),
        },
        json!({
            "id": 7,
            "type": "vertex",
            "label": "range",
            "start": {"line": 10, "character": 3},
            "end": {"line": 10, "character": 8},
            "tag": {
                "type": "definition",
                "text": "greet",
                "kind": 12,
                "fullRange": {
                    "start": {"line": 10, "character": 0},
                    "end": {"line": 12, "character": 1}
                },
                "detail": "fn greet(name: &str)"
            }
        }),
    );
}

#[test]
fn job_emit_declaration_range_with_tag() {
    assert_vertex_roundtrip(
        8,
        Vertex::Range {
            range: Range {
                start: Position::new(0, 10),
                end: Position::new(0, 15),
            },
            tag: Some(RangeTag::Declaration(DeclarationTag {
                text: "Trait".into(),
                kind: SymbolKind::INTERFACE,
                deprecated: false,
                full_range: Range {
                    start: Position::new(0, 0),
                    end: Position::new(5, 1),
                },
                detail: None,
            })),
        },
        json!({
            "id": 8,
            "type": "vertex",
            "label": "range",
            "start": {"line": 0, "character": 10},
            "end": {"line": 0, "character": 15},
            "tag": {
                "type": "declaration",
                "text": "Trait",
                "kind": 11,
                "deprecated": false,
                "fullRange": {
                    "start": {"line": 0, "character": 0},
                    "end": {"line": 5, "character": 1}
                }
            }
        }),
    );
}

#[test]
fn job_emit_reference_range_with_tag() {
    assert_vertex_roundtrip(
        9,
        Vertex::Range {
            range: Range {
                start: Position::new(20, 0),
                end: Position::new(20, 5),
            },
            tag: Some(RangeTag::Reference(ReferenceTag { text: "greet".into() })),
        },
        json!({
            "id": 9,
            "type": "vertex",
            "label": "range",
            "start": {"line": 20, "character": 0},
            "end": {"line": 20, "character": 5},
            "tag": {"type": "reference", "text": "greet"}
        }),
    );
}

#[test]
fn job_emit_unknown_range_with_tag() {
    assert_vertex_roundtrip(
        10,
        Vertex::Range {
            range: Range {
                start: Position::new(1, 0),
                end: Position::new(1, 3),
            },
            tag: Some(RangeTag::Unknown(UnknownTag { text: "???".into() })),
        },
        json!({
            "id": 10,
            "type": "vertex",
            "label": "range",
            "start": {"line": 1, "character": 0},
            "end": {"line": 1, "character": 3},
            "tag": {"type": "unknown", "text": "???"}
        }),
    );
}

// ─── Job: Wire up range → result set (next edge) ─────────────────────────────────────────────

#[test]
fn job_emit_next_edge_from_range_to_result_set() {
    assert_edge_roundtrip(
        11,
        Edge::Next(EdgeData {
            out_v: NumberOrString::Number(7),
            in_v: NumberOrString::Number(6),
        }),
        json!({
            "id": 11,
            "type": "edge",
            "label": "next",
            "outV": 7,
            "inV": 6
        }),
    );
}

// ─── Job: Emit hover result for a symbol ─────────────────────────────────────────────────────

#[test]
fn job_emit_hover_result_vertex() {
    assert_vertex_roundtrip(
        12,
        Vertex::HoverResult {
            result: Hover {
                contents: HoverContents::Scalar(MarkedString::LanguageString(
                    LanguageString {
                        language: "rust".into(),
                        value: "fn greet(name: &str) -> String".into(),
                    },
                )),
                range: None,
            },
        },
        json!({
            "id": 12,
            "type": "vertex",
            "label": "hoverResult",
            "result": {
                "contents": {"language": "rust", "value": "fn greet(name: &str) -> String"}
            }
        }),
    );
}

#[test]
fn job_wire_result_set_to_hover_result() {
    assert_edge_roundtrip(
        13,
        Edge::Hover(EdgeData {
            out_v: NumberOrString::Number(6),
            in_v: NumberOrString::Number(12),
        }),
        json!({
            "id": 13,
            "type": "edge",
            "label": "textDocument/hover",
            "outV": 6,
            "inV": 12
        }),
    );
}

// ─── Job: Emit definition result and wire it ─────────────────────────────────────────────────

#[test]
fn job_emit_definition_result_vertex() {
    assert_vertex_roundtrip(
        14,
        Vertex::DefinitionResult,
        json!({"id": 14, "type": "vertex", "label": "definitionResult"}),
    );
}

#[test]
fn job_wire_result_set_to_definition_result() {
    assert_edge_roundtrip(
        15,
        Edge::Definition(EdgeData {
            out_v: NumberOrString::Number(6),
            in_v: NumberOrString::Number(14),
        }),
        json!({
            "id": 15,
            "type": "edge",
            "label": "textDocument/definition",
            "outV": 6,
            "inV": 14
        }),
    );
}

// ─── Job: Emit declaration result ────────────────────────────────────────────────────────────

#[test]
fn job_emit_declaration_result_vertex() {
    assert_vertex_roundtrip(
        16,
        Vertex::DeclarationResult,
        json!({"id": 16, "type": "vertex", "label": "declarationResult"}),
    );
}

#[test]
fn job_wire_result_set_to_declaration_result() {
    assert_edge_roundtrip(
        17,
        Edge::Declaration(EdgeData {
            out_v: NumberOrString::Number(6),
            in_v: NumberOrString::Number(16),
        }),
        json!({
            "id": 17,
            "type": "edge",
            "label": "textDocument/declaration",
            "outV": 6,
            "inV": 16
        }),
    );
}

// ─── Job: Emit reference result with item edges ───────────────────────────────────────────────

#[test]
fn job_emit_reference_result_vertex() {
    assert_vertex_roundtrip(
        18,
        Vertex::ReferenceResult,
        json!({"id": 18, "type": "vertex", "label": "referenceResult"}),
    );
}

#[test]
fn job_wire_result_set_to_reference_result() {
    assert_edge_roundtrip(
        19,
        Edge::References(EdgeData {
            out_v: NumberOrString::Number(6),
            in_v: NumberOrString::Number(18),
        }),
        json!({
            "id": 19,
            "type": "edge",
            "label": "textDocument/references",
            "outV": 6,
            "inV": 18
        }),
    );
}

#[test]
fn job_emit_item_edge_for_definitions() {
    assert_edge_roundtrip(
        20,
        Edge::Item(Item {
            document: NumberOrString::Number(4),
            property: Some(ItemKind::Definitions),
            edge_data: EdgeDataMultiIn {
                out_v: NumberOrString::Number(18),
                in_vs: vec![NumberOrString::Number(7)],
            },
        }),
        json!({
            "id": 20,
            "type": "edge",
            "label": "item",
            "document": 4,
            "property": "definitions",
            "outV": 18,
            "inVs": [7]
        }),
    );
}

#[test]
fn job_emit_item_edge_for_references() {
    assert_edge_roundtrip(
        21,
        Edge::Item(Item {
            document: NumberOrString::Number(4),
            property: Some(ItemKind::References),
            edge_data: EdgeDataMultiIn {
                out_v: NumberOrString::Number(18),
                in_vs: vec![NumberOrString::Number(9), NumberOrString::Number(10)],
            },
        }),
        json!({
            "id": 21,
            "type": "edge",
            "label": "item",
            "document": 4,
            "property": "references",
            "outV": 18,
            "inVs": [9, 10]
        }),
    );
}

#[test]
fn job_emit_item_edge_for_declarations() {
    assert_edge_roundtrip(
        22,
        Edge::Item(Item {
            document: NumberOrString::Number(4),
            property: Some(ItemKind::Declarations),
            edge_data: EdgeDataMultiIn {
                out_v: NumberOrString::Number(18),
                in_vs: vec![NumberOrString::Number(8)],
            },
        }),
        json!({
            "id": 22,
            "type": "edge",
            "label": "item",
            "document": 4,
            "property": "declarations",
            "outV": 18,
            "inVs": [8]
        }),
    );
}

#[test]
fn job_emit_item_edge_for_reference_results() {
    assert_edge_roundtrip(
        23,
        Edge::Item(Item {
            document: NumberOrString::Number(4),
            property: Some(ItemKind::ReferenceResults),
            edge_data: EdgeDataMultiIn {
                out_v: NumberOrString::Number(18),
                in_vs: vec![NumberOrString::Number(30)],
            },
        }),
        json!({
            "id": 23,
            "type": "edge",
            "label": "item",
            "document": 4,
            "property": "referenceResults",
            "outV": 18,
            "inVs": [30]
        }),
    );
}

#[test]
fn job_emit_item_edge_for_implementation_results() {
    assert_edge_roundtrip(
        24,
        Edge::Item(Item {
            document: NumberOrString::Number(4),
            property: Some(ItemKind::ImplementationResults),
            edge_data: EdgeDataMultiIn {
                out_v: NumberOrString::Number(25),
                in_vs: vec![NumberOrString::Number(26)],
            },
        }),
        json!({
            "id": 24,
            "type": "edge",
            "label": "item",
            "document": 4,
            "property": "implementationResults",
            "outV": 25,
            "inVs": [26]
        }),
    );
}

#[test]
fn job_emit_item_edge_without_property() {
    assert_edge_roundtrip(
        25,
        Edge::Item(Item {
            document: NumberOrString::Number(4),
            property: None,
            edge_data: EdgeDataMultiIn {
                out_v: NumberOrString::Number(18),
                in_vs: vec![NumberOrString::Number(7)],
            },
        }),
        json!({
            "id": 25,
            "type": "edge",
            "label": "item",
            "document": 4,
            "outV": 18,
            "inVs": [7]
        }),
    );
}

// ─── Job: Emit implementation result ─────────────────────────────────────────────────────────

#[test]
fn job_emit_implementation_result_vertex() {
    assert_vertex_roundtrip(
        26,
        Vertex::ImplementationResult,
        json!({"id": 26, "type": "vertex", "label": "implementationResult"}),
    );
}

#[test]
fn job_wire_result_set_to_implementation_result() {
    assert_edge_roundtrip(
        27,
        Edge::Implementation(EdgeData {
            out_v: NumberOrString::Number(6),
            in_v: NumberOrString::Number(26),
        }),
        json!({
            "id": 27,
            "type": "edge",
            "label": "textDocument/implementation",
            "outV": 6,
            "inV": 26
        }),
    );
}

// ─── Job: Emit type definition result ────────────────────────────────────────────────────────

#[test]
fn job_emit_type_definition_result_vertex() {
    assert_vertex_roundtrip(
        28,
        Vertex::TypeDefinitionResult,
        json!({"id": 28, "type": "vertex", "label": "typeDefinitionResult"}),
    );
}

#[test]
fn job_wire_result_set_to_type_definition_result() {
    assert_edge_roundtrip(
        29,
        Edge::TypeDefinition(EdgeData {
            out_v: NumberOrString::Number(6),
            in_v: NumberOrString::Number(28),
        }),
        json!({
            "id": 29,
            "type": "edge",
            "label": "textDocument/typeDefinition",
            "outV": 6,
            "inV": 28
        }),
    );
}

// ─── Job: Emit folding range result ──────────────────────────────────────────────────────────

#[test]
fn job_emit_folding_range_result_vertex() {
    assert_vertex_roundtrip(
        30,
        Vertex::FoldingRangeResult {
            result: vec![
                FoldingRange {
                    start_line: 0,
                    start_character: None,
                    end_line: 10,
                    end_character: None,
                    kind: None,
                    collapsed_text: None,
                },
                FoldingRange {
                    start_line: 15,
                    start_character: None,
                    end_line: 20,
                    end_character: None,
                    kind: None,
                    collapsed_text: None,
                },
            ],
        },
        json!({
            "id": 30,
            "type": "vertex",
            "label": "foldingRangeResult",
            "result": [
                {"startLine": 0, "endLine": 10},
                {"startLine": 15, "endLine": 20}
            ]
        }),
    );
}

#[test]
fn job_wire_document_to_folding_range_result() {
    assert_edge_roundtrip(
        31,
        Edge::FoldingRange(EdgeData {
            out_v: NumberOrString::Number(4),
            in_v: NumberOrString::Number(30),
        }),
        json!({
            "id": 31,
            "type": "edge",
            "label": "textDocument/foldingRange",
            "outV": 4,
            "inV": 30
        }),
    );
}

// ─── Job: Emit document link result ──────────────────────────────────────────────────────────

#[test]
fn job_emit_document_link_result_vertex() {
    assert_vertex_roundtrip(
        32,
        Vertex::DocumentLinkResult {
            result: vec![
                DocumentLink {
                    range: Range::new(Position::new(5, 0), Position::new(5, 20)),
                    target: Some("file:///other.rs".parse().unwrap()),
                    tooltip: None,
                    data: None,
                },
            ],
        },
        json!({
            "id": 32,
            "type": "vertex",
            "label": "documentLinkResult",
            "result": [{
                "range": {"start": {"line": 5, "character": 0}, "end": {"line": 5, "character": 20}},
                "target": "file:///other.rs"
            }]
        }),
    );
}

#[test]
fn job_wire_document_to_document_link_result() {
    assert_edge_roundtrip(
        33,
        Edge::DocumentLink(EdgeData {
            out_v: NumberOrString::Number(4),
            in_v: NumberOrString::Number(32),
        }),
        json!({
            "id": 33,
            "type": "edge",
            "label": "textDocument/documentLink",
            "outV": 4,
            "inV": 32
        }),
    );
}

// ─── Job: Emit document symbol result ────────────────────────────────────────────────────────

#[test]
fn job_emit_document_symbol_result_range_based() {
    assert_vertex_roundtrip(
        34,
        Vertex::DocumentSymbolResult {
            result: DocumentSymbolOrRangeBasedVec::RangeBased(vec![
                RangeBasedDocumentSymbol {
                    id: NumberOrString::Number(7),
                    children: vec![
                        RangeBasedDocumentSymbol {
                            id: NumberOrString::Number(9),
                            children: vec![],
                        },
                    ],
                },
            ]),
        },
        json!({
            "id": 34,
            "type": "vertex",
            "label": "documentSymbolResult",
            "result": [
                {"id": 7, "children": [{"id": 9}]}
            ]
        }),
    );
}

#[test]
fn job_wire_document_to_document_symbol_result() {
    assert_edge_roundtrip(
        35,
        Edge::DocumentSymbol(EdgeData {
            out_v: NumberOrString::Number(4),
            in_v: NumberOrString::Number(34),
        }),
        json!({
            "id": 35,
            "type": "edge",
            "label": "textDocument/documentSymbol",
            "outV": 4,
            "inV": 34
        }),
    );
}

// ─── Job: Emit diagnostic result for a document ───────────────────────────────────────────────

#[test]
fn job_emit_diagnostic_result_vertex() {
    assert_vertex_roundtrip(
        36,
        Vertex::DiagnosticResult {
            result: vec![
                Diagnostic {
                    range: Range::new(Position::new(3, 0), Position::new(3, 10)),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("E0001".into())),
                    code_description: None,
                    source: Some("rustc".into()),
                    message: "cannot find value `foo` in this scope".into(),
                    related_information: None,
                    tags: None,
                    data: None,
                },
            ],
        },
        json!({
            "id": 36,
            "type": "vertex",
            "label": "diagnosticResult",
            "result": [{
                "range": {
                    "start": {"line": 3, "character": 0},
                    "end": {"line": 3, "character": 10}
                },
                "severity": 1,
                "code": "E0001",
                "source": "rustc",
                "message": "cannot find value `foo` in this scope"
            }]
        }),
    );
}

#[test]
fn job_wire_document_to_diagnostic_result() {
    assert_edge_roundtrip(
        37,
        Edge::Diagnostic(EdgeData {
            out_v: NumberOrString::Number(4),
            in_v: NumberOrString::Number(36),
        }),
        json!({
            "id": 37,
            "type": "edge",
            "label": "textDocument/diagnostic",
            "outV": 4,
            "inV": 36
        }),
    );
}

// ─── Job: Emit moniker for cross-repo symbol resolution ──────────────────────────────────────

#[test]
fn job_emit_moniker_vertex() {
    assert_vertex_roundtrip(
        38,
        Vertex::Moniker(Moniker {
            scheme: "rust-analyzer".into(),
            identifier: "std::vec::Vec::push".into(),
            unique: UniquenessLevel::Global,
            kind: Some(MonikerKind::Export),
        }),
        json!({
            "id": 38,
            "type": "vertex",
            "label": "moniker",
            "scheme": "rust-analyzer",
            "identifier": "std::vec::Vec::push",
            "unique": "global",
            "kind": "export"
        }),
    );
}

#[test]
fn job_wire_result_set_to_moniker() {
    assert_edge_roundtrip(
        39,
        Edge::Moniker(EdgeData {
            out_v: NumberOrString::Number(6),
            in_v: NumberOrString::Number(38),
        }),
        json!({
            "id": 39,
            "type": "edge",
            "label": "moniker",
            "outV": 6,
            "inV": 38
        }),
    );
}

// ─── Job: Emit package information for a moniker ─────────────────────────────────────────────

#[test]
fn job_emit_package_information_vertex() {
    assert_vertex_roundtrip(
        40,
        Vertex::PackageInformation(PackageInformation {
            name: "my-crate".into(),
            manager: "cargo".into(),
            uri: None,
            content: None,
            repository: Some(Repository {
                r#type: "git".into(),
                url: "https://github.com/org/my-crate".into(),
                commit_id: Some("abc123".into()),
            }),
            version: Some("1.2.3".into()),
        }),
        json!({
            "id": 40,
            "type": "vertex",
            "label": "packageInformation",
            "name": "my-crate",
            "manager": "cargo",
            "repository": {
                "type": "git",
                "url": "https://github.com/org/my-crate",
                "commitId": "abc123"
            },
            "version": "1.2.3"
        }),
    );
}

#[test]
fn job_wire_moniker_to_package_information() {
    assert_edge_roundtrip(
        41,
        Edge::PackageInformation(EdgeData {
            out_v: NumberOrString::Number(38),
            in_v: NumberOrString::Number(40),
        }),
        json!({
            "id": 41,
            "type": "edge",
            "label": "packageInformation",
            "outV": 38,
            "inV": 40
        }),
    );
}

// ─── Job: Chain monikers from local to global scope ───────────────────────────────────────────

#[test]
fn job_emit_local_export_moniker() {
    assert_vertex_roundtrip(
        42,
        Vertex::Moniker(Moniker {
            scheme: "rust-analyzer".into(),
            identifier: "crate::local::Symbol".into(),
            unique: UniquenessLevel::Document,
            kind: Some(MonikerKind::Export),
        }),
        json!({
            "id": 42,
            "type": "vertex",
            "label": "moniker",
            "scheme": "rust-analyzer",
            "identifier": "crate::local::Symbol",
            "unique": "document",
            "kind": "export"
        }),
    );
}

#[test]
fn job_wire_next_moniker_from_local_to_global() {
    assert_edge_roundtrip(
        43,
        Edge::NextMoniker(EdgeData {
            out_v: NumberOrString::Number(42),
            in_v: NumberOrString::Number(38),
        }),
        json!({
            "id": 43,
            "type": "edge",
            "label": "nextMoniker",
            "outV": 42,
            "inV": 38
        }),
    );
}

// ─── Job: Contain ranges within a document ───────────────────────────────────────────────────

#[test]
fn job_emit_contains_edge_from_document_to_ranges() {
    assert_edge_roundtrip(
        44,
        Edge::Contains(EdgeDataMultiIn {
            out_v: NumberOrString::Number(4),
            in_vs: vec![
                NumberOrString::Number(7),
                NumberOrString::Number(8),
                NumberOrString::Number(9),
            ],
        }),
        json!({
            "id": 44,
            "type": "edge",
            "label": "contains",
            "outV": 4,
            "inVs": [7, 8, 9]
        }),
    );
}

// ─── Job: String-based IDs for LSIF entries ──────────────────────────────────────────────────

#[test]
fn job_emit_entry_with_string_id() {
    let entry = Entry {
        id: NumberOrString::String("v-metadata".into()),
        data: Element::Vertex(Vertex::MetaData(MetaData {
            version: "0.6.0".into(),
            project_root: "file:///ws".parse().unwrap(),
            position_encoding: Encoding::Utf16,
            tool_info: None,
        })),
    };
    assert_entry_roundtrip(&entry, json!({
        "id": "v-metadata",
        "type": "vertex",
        "label": "metaData",
        "version": "0.6.0",
        "projectRoot": "file:///ws",
        "positionEncoding": "utf-16"
    }));
}

// ─── Job: Parse and re-emit a complete minimal LSIF index ────────────────────────────────────

#[test]
fn job_parse_and_reemit_minimal_lsif_index() {
    let lines = vec![
        json!({"id":1,"type":"vertex","label":"metaData","version":"0.6.0","projectRoot":"file:///ws","positionEncoding":"utf-16"}),
        json!({"id":2,"type":"vertex","label":"project","kind":"rust"}),
        json!({"id":3,"type":"vertex","label":"$event","kind":"begin","scope":"project","data":2}),
        json!({"id":4,"type":"vertex","label":"document","uri":"file:///ws/src/main.rs","languageId":"rust"}),
        json!({"id":5,"type":"vertex","label":"$event","kind":"begin","scope":"document","data":4}),
        json!({"id":6,"type":"vertex","label":"resultSet"}),
        json!({"id":7,"type":"vertex","label":"moniker","kind":"export","unique":"global","scheme":"rust-analyzer","identifier":"main::main"}),
        json!({"id":8,"type":"edge","label":"moniker","outV":6,"inV":7}),
        json!({"id":9,"type":"vertex","label":"range","start":{"line":0,"character":3},"end":{"line":0,"character":7}}),
        json!({"id":10,"type":"edge","label":"next","outV":9,"inV":6}),
        json!({"id":11,"type":"edge","label":"contains","outV":4,"inVs":[9]}),
        json!({"id":12,"type":"vertex","label":"$event","kind":"end","scope":"document","data":4}),
        json!({"id":13,"type":"vertex","label":"$event","kind":"end","scope":"project","data":2}),
    ];

    for expected in &lines {
        let entry: Entry = serde_json::from_value(expected.clone())
            .expect("Failed to deserialize LSIF entry");
        let actual = serde_json::to_value(&entry)
            .expect("Failed to serialize LSIF entry");
        assert_eq!(&actual, expected);
    }
}
