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

# Build with proposed features (inline completion, etc.)
cargo build --features proposed
cargo test --features proposed

# Check only (faster than build)
cargo check
```

## Architecture

This is a Rust library crate (`lsp-types-max`) providing complete type coverage for:
- **LSP 3.18** — Language Server Protocol (all requests, notifications, capabilities)
- **LSIF 0.6.0** — Language Server Index Format (vertex/edge graph types)
- **Base Protocol 0.9** — JSON-RPC message wrappers
- **Process Mining extensions** — Van der Aalst-inspired workflow net and conformance types

### Module Structure

**Core type layer** — `src/lib.rs` is the root, re-exporting everything flat:
- `src/types/` — `Position`, `Range`, `Location`, auxiliary types
- `src/protocol/` — JSON-RPC wrappers: `RequestMessage`, `ResponseMessage`, `NotificationMessage`, `ResponseError`
- Domain modules (one per LSP feature area): `completion.rs`, `code_action.rs`, `semantic_tokens.rs`, `hover.rs`, `inlay_hint.rs`, `inline_value.rs`, `notebook.rs`, `workspace_symbols.rs`, etc.
- `src/request.rs` — `Request` trait + `lsp_request!("method/name")` macro mapping method strings to implementor types
- `src/notification.rs` — `Notification` trait + `lsp_notification!("method/name")` macro
- `src/lsif.rs` — LSIF 0.6.0 types (vertices, edges, range tags)
- `src/open_set_types.rs` — Open-set types like `SemanticTokenTypes` and `SemanticTokenModifiers` (newtype wrappers around `String`)
- `src/inline_completion.rs` — gated behind `#[cfg(feature = "proposed")]`

**Process mining extensions** (added in this session):
- `src/process_mining.rs` — XES-compatible event log types for LSP sessions (`LspEventLog`, `LspTrace`, `LspEvent`, `LspActivity` with 80 message variants)
- `src/conformance.rs` — Protocol state machine conformance checker (5 states, 7 violation kinds, replay-based)
- `src/workflow_net.rs` — Petri net / workflow net model (`Place`, `Transition`, `Arc`, `Marking`, `lsp_lifecycle_net()`)

### Key Patterns

**Open-set enums** — LSP uses extensible enums (e.g. `CodeActionKind`, `SemanticTokenType`). These are newtype wrappers (`struct Foo(String)`) with associated `const` for known values, rather than Rust enums. See `src/open_set_types.rs`.

**`lsp_enum!` macro** — Used for integer-backed enums (like `MessageType`, `DiagnosticSeverity`). Generates `Debug`, `TryFrom<&str>`, and PascalCase formatting automatically. Defined in `src/lib.rs`.

**`lsp_request!` / `lsp_notification!` macros** — Used as types, not values: `type T = lsp_request!("textDocument/hover"); let method = <T as Request>::METHOD;`

**Serde conventions**:
- All struct fields use `#[serde(rename_all = "camelCase")]`
- Optional fields use `#[serde(skip_serializing_if = "Option::is_none")]`
- Untagged enums (`#[serde(untagged)]`) are used for `OneOf<A, B>` and `NumberOrString` — deserialization is order-sensitive (first matching variant wins)
- `TransitionKind` in `workflow_net.rs` uses PascalCase variants (no rename_all) so serializes as `"ClientRequest"` not `"clientRequest"`

**`PartialResultParams`** — Has a non-obvious field: `is_partial_result_token_null: bool`. Set to `false` when `partial_result_token: None`.

### Test Files

| File | Count | Coverage |
|---|---|---|
| `tests/e2e_spec_3_18.rs` | 174 | LSP 3.18 feature E2E (inline completion, ranges formatting, notebooks, etc.) |
| `tests/base_spec.rs` | 10 | Base Protocol 0.9 JSON-RPC types |
| `tests/jtbd_all_requests.rs` | 124 | All LSP request/notification param and result types |
| `tests/jtbd_lsp_3_18.rs` | 51 | LSP 3.18 JTBD scenarios |
| `tests/jtbd_lsif_0_6.rs` | 53 | LSIF 0.6.0 vertex/edge types |
| `tests/jtbd_conformance.rs` | 33 | Protocol state machine conformance |
| `tests/jtbd_process_mining.rs` | 32 | XES event log types |
| `tests/jtbd_workflow_net.rs` | 33 | Petri net model |

All tests use a structural JSON comparison helper (key-order agnostic):
```rust
fn assert_serde_roundtrip<T: Serialize + DeserializeOwned + Debug + PartialEq>(
    val: &T, expected: serde_json::Value)
```

### Known Serialization Gotchas

- `WorkspaceSymbolResponse` is `#[serde(untagged)]` — `Nested(Vec<WorkspaceSymbol>)` with a ranged `Location` is ambiguous with `Flat(Vec<SymbolInformation>)` during deserialization. Roundtrip only works with `WorkspaceLocation` (no range field).
- `DeclarationTag.deprecated: bool` always serializes (no `skip_serializing_if`) — include `"deprecated": false` in expected JSON even when false.
- `UniquenessLevel` has no `Workspace` variant — use `Global`, `Document`, `Project`, `Group`, or `Scheme`.

### Proposed Features

Enable with `--features proposed`:
- `src/inline_completion.rs` — `InlineCompletionRequest`, `InlineCompletionParams`, etc.
- No semver guarantees for types behind this flag.
