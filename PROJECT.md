# Project: lsp-types LSP 3.18.0 Upgrade

## Architecture
This project upgrades the gluon-lang `lsp-types` library (located at `/Users/sac/lsp-types`) to the LSP 3.18.0 specification and verifies its correctness in isolation and when patched into `tower-lsp-max` (located at `/Users/sac/tower-lsp-max`).

The code structure in `lsp-types` consists of:
- `src/lib.rs` (core types and library entry point)
- `src/completion.rs`, `src/code_action.rs`, `src/semantic_tokens.rs`, `src/notebook.rs`, `src/inline_completion.rs`, etc. (domain-specific modules)
- `src/request.rs` (request traits and list of all request methods)
- `src/notification.rs` (notification traits and list of all notification methods)
- `tests/` (integration tests)

## Milestones

| # | Name | Scope | Dependencies | Status |
|---|---|---|---|---|
| 1 | Patch Workspace | Link local `lsp-types` to `tower-lsp-max` using patches, verify baseline build/test. | None | DONE: Patched tower-lsp-max to local lsp-types 0.97.0. Workspace compiles and passes all tests. |
| 2 | Basic Types & Enums | Implement `SnippetTextEdit`, `WorkspaceEditMetadata`, proposed `CodeActionKind/Tag`, `MessageType::Debug`, `LanguageKind` open-set, relative glob patterns in selectors. | M1 | DONE |
| 3 | Core Requests & Features | Implement `CompletionList.itemDefaults`, Inline Completion capabilities/requests, Ranges Formatting, Folding Range Refresh, Text Document Content, Notebook sync & cell diagnostic pull, open-set Semantic Tokens. | M2 | DONE |
| 4 | E2E Test Suite | E2E Testing Track publishes `TEST_READY.md` (Tier 1-4 tests covering all 11 feature sets). | M1 | DONE |
| 5 | Integration & Hardening | Phase 1: Pass 100% of E2E and workspace tests. Phase 2: White-box adversarial coverage hardening. | M3, M4 | DONE |\n| 6 | Base Protocol Alignment | Structural alignment with Base Spec 0.9 (BaseAny, BaseObject, BaseArray, URI prohibition). | M1, M4 | DONE |

## Interface Contracts
- **LSP 3.18.0 types and structures**: must match the official LSP 3.18.0 specification exactly.
- **Serialization and Deserialization (Serde)**: must use camelCase renaming for struct fields and serialize/deserialize correctly according to LSP expectations.
- **Cargo patch target**: `lsp-types` version in `tower-lsp-max` must resolve to local path `/Users/sac/lsp-types`.
