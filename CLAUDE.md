# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build

# Run all tests
cargo test

# Run a single test by name (substring match)
cargo test job_initialize_params_minimal

# Run all tests in one file
cargo test --test jtbd_all_requests

# Build/test with proposed features (inline completion, etc.)
cargo build --features proposed
cargo test --features proposed

# Check only (faster than build)
cargo check
```

## Architecture

`lsp-types-max` is a Rust library providing complete static type coverage for:
- **LSP 3.18** — all requests, notifications, and capabilities
- **LSIF 0.6.0** — vertex/edge graph types for language server indexes
- **Base Protocol 0.9** — JSON-RPC message wrappers
- **Process mining extensions** — Van der Aalst workflow net and conformance types

### Module layout

`src/lib.rs` is the crate root and re-exports everything flat via `pub use <module>::*`. Consumers import directly from the crate root (`use lsp_types_max::*;`).

**Core type layers:**
- `src/types/` — `Position`, `Range`, `Location`, auxiliary types
- `src/protocol/` — `RequestMessage`, `ResponseMessage`, `NotificationMessage`, `ResponseError`
- `src/base_spec.rs` — `BaseAny` / `BaseObject` / `BaseArray` aliases; also re-exported under `pub mod base` as `URI`, `DocumentUri`
- `src/open_set_types.rs` — `LanguageKind`, `SemanticTokenTypes`, `SemanticTokenModifiers` (newtype wrappers around `String` with `const` known values)

**Domain modules** (one per LSP feature area, all re-exported flat):
`completion.rs`, `code_action.rs`, `semantic_tokens.rs`, `hover.rs`, `inlay_hint.rs`, `inline_value.rs`, `notebook.rs`, `workspace_symbols.rs`, `call_hierarchy.rs`, `type_hierarchy.rs`, `moniker.rs`, `document_diagnostic.rs`, `workspace_diagnostic.rs`, etc.

**LSP 3.18 feature modules** — added during the 3.18 upgrade, each owned by a parallel agent:
- `src/agent1_inline.rs` — re-exports `inline_completion` (proposed gate)
- `src/agent2_td_content.rs` — `workspace/textDocumentContent` types
- `src/agent3_ranges_format.rs` — `textDocument/rangesFormatting`
- `src/agent4_folding_refresh.rs` — `workspace/foldingRange/refresh`
- `src/agent5_notebook_sync.rs` — notebook sync notifications
- `src/agent6_cell_diag.rs` — notebook cell diagnostic pull
- `src/agent7_code_action_ext.rs` — `CodeActionKind::RefactorMove`, `CodeActionTag::LLMGenerated`
- `src/agent8_snippet_edit.rs` — `SnippetTextEdit`, `WorkspaceEditMetadata`
- `src/agent9_completion_ext.rs` — `CompletionListItemDefaults`
- `src/agent10_misc_ext.rs` — `MessageType::Debug`, `LanguageKind`, relative glob patterns

All `agent*` modules are declared with `#[allow(unused_imports)]` in `lib.rs` because some re-export types that are also defined in the canonical domain modules.

**Request/notification dispatch:**
- `src/request.rs` — `Request` trait + `lsp_request!("method/name")` macro
- `src/notification.rs` — `Notification` trait + `lsp_notification!("method/name")` macro

**Other modules:**
- `src/lsif.rs` — LSIF 0.6.0 vertices, edges, range tags
- `src/inline_completion.rs` — gated behind `#[cfg(feature = "proposed")]`
- `src/process_mining.rs` — XES event log types (`LspEventLog`, `LspTrace`, `LspEvent`, `LspActivity` with 80 variants)
- `src/conformance.rs` — protocol state machine conformance checker (5 states, 7 violation kinds)
- `src/workflow_net.rs` — Petri net model (`Place`, `Transition`, `Arc`, `Marking`, `lsp_lifecycle_net()`)

### Key patterns

**`lsp_request!` / `lsp_notification!` macros expand to types, not values:**
```rust
// Correct — use as a type alias
type T = lsp_request!("textDocument/hover");
let method = <T as Request>::METHOD;

// Wrong — cannot call as an expression
let method = lsp_request!("textDocument/hover")::METHOD; // compile error
```

**`lsp_enum!` macro** — for integer-backed enums (`MessageType`, `DiagnosticSeverity`, `SymbolKind`, etc.). Generates `Debug` (PascalCase), `TryFrom<&str>`. Defined in `src/lib.rs`.

**Open-set enums** — LSP allows unknown enum values. Instead of Rust enums, these are newtype wrappers with `const` known values:
```rust
// These are structs wrapping String, not enums
let kind: CodeActionKind = CodeActionKind::QUICKFIX;
let custom: CodeActionKind = CodeActionKind("refactor.rewrite.custom".into());
```

**`OneOf<A, B>` / `OneOf3<A, B, C>`** — `#[serde(untagged)]` union types used throughout `ServerCapabilities`. Deserialization is order-sensitive: the first variant whose shape matches wins. Used for `Option<OneOf<bool, FooOptions>>` capability fields.

**Serde conventions across all structs:**
- `#[serde(rename_all = "camelCase")]` on every struct
- `#[serde(skip_serializing_if = "Option::is_none")]` on optional fields
- `TransitionKind` in `workflow_net.rs` has no `rename_all` so variants serialize as PascalCase (`"ClientRequest"`, not `"clientRequest"`)

**`PartialResultParams`** — has a non-obvious required field:
```rust
PartialResultParams { partial_result_token: None, is_partial_result_token_null: false }
// NOT: PartialResultParams { partial_result_token: None }  // missing field — won't compile
```

### Test files

| File | Tests | Coverage |
|---|---|---|
| `tests/e2e_spec_3_18.rs` | 174 | LSP 3.18 E2E: inline completion, ranges formatting, notebooks, etc. |
| `tests/base_spec_0_9.rs` | 10 | Base Protocol 0.9 JSON-RPC types |
| `tests/jtbd_all_requests.rs` | 124 | All LSP request/notification param and result types |
| `tests/jtbd_lsp_3_18.rs` | 51 | LSP 3.18 JTBD scenarios |
| `tests/jtbd_lsif_0_6.rs` | 53 | LSIF 0.6.0 vertex/edge types |
| `tests/jtbd_conformance.rs` | 33 | Protocol state machine conformance |
| `tests/jtbd_process_mining.rs` | 32 | XES event log types |
| `tests/jtbd_workflow_net.rs` | 33 | Petri net model |

**Test helper pattern** used across all test files — structural JSON comparison, key-order agnostic:
```rust
fn assert_serde_roundtrip<T: Serialize + DeserializeOwned + Debug + PartialEq>(
    val: &T, expected: serde_json::Value) { ... }
```

`jtbd_all_requests.rs` also defines a `rt_test!` macro for concise roundtrip tests:
```rust
rt_test!(my_test_name, MyType, my_value, json!({...}));
```

### Known serialization gotchas

- **`WorkspaceSymbolResponse` untagged deserialization** — `Nested(Vec<WorkspaceSymbol>)` with a `Location` that has a range field is structurally ambiguous with `Flat(Vec<SymbolInformation>)`. Full roundtrip only works when using `WorkspaceLocation` (no `range` field). Use serialization-only tests for the `Location` variant.
- **`DeclarationTag.deprecated`** — `bool` with no `skip_serializing_if`, always serializes. Include `"deprecated": false` in expected JSON even when not set.
- **`UniquenessLevel`** — has no `Workspace` variant. Use `Global`, `Document`, `Project`, `Group`, or `Scheme`.
- **LSIF helper tests** — pass `i32` directly for vertex IDs, not `NumberOrString`. Wrap internally: `NumberOrString::Number(vid)`.

### Proposed features

Enable with `--features proposed`:
- `src/inline_completion.rs` — `InlineCompletionRequest`, `InlineCompletionParams`, `InlineCompletionList`, etc.
- No semver guarantees for types behind this flag.
