# Original User Request

## Initial Request — 2026-06-05T11:55:27-07:00

Upgrade the gluon-lang `lsp-types` Rust library located in `/Users/sac/lsp-types` to support the LSP 3.18.0 specification features, and verify it standalone and inside the `tower-lsp-max` workspace.

Working directory: /Users/sac/lsp-types
Integrity mode: benchmark

## Requirements

### R1. Full LSP 3.18.0 Specification Support
Upgrade the `lsp-types` crate to support all new requests, notifications, capabilities, structures, and variants defined in the LSP 3.18.0 specification. This includes:
- `textDocument/inlineCompletion`
- `workspace/textDocumentContent` & `workspace/textDocumentContent/refresh`
- `textDocument/rangesFormatting`
- `workspace/foldingRange/refresh`
- Notebook document synchronization and cell diagnostic pull structures
- Proposed `CodeActionKind.RefactorMove` and `CodeActionTag.LLMGenerated`
- `SnippetTextEdit` and `WorkspaceEditMetadata`
- `CompletionList.itemDefaults` (data, editRange, applyKind merge strategies)
- `MessageType.Debug`
- Relative glob pattern support in selectors
- Open-set definitions for `SemanticTokenTypes`, `SemanticTokenModifiers`, and `LanguageKind`

### R2. Standalone Serialization & Round-Trip Tests
Implement comprehensive serialization and deserialization unit/integration tests in the `lsp-types` crate verifying that each newly added struct or enum round-trips cleanly to and from JSON according to the specification.

### R3. tower-lsp-max Integration Verification
Link the local `lsp-types` crate into the `tower-lsp-max` workspace (located at `/Users/sac/tower-lsp-max`) using Cargo path/patch overrides. Verify that the entire `tower-lsp-max` workspace compiles cleanly and passes all workspace tests using the upgraded `lsp-types` crate.

## Acceptance Criteria

### Standalone Validation
- [ ] `cargo check` and `cargo test` pass cleanly inside `/Users/sac/lsp-types` with zero compile errors and zero warnings.
- [ ] Standalone serialization tests exist in `/Users/sac/lsp-types` verifying the new LSP 3.18.0 types.

### Integration Validation
- [ ] The `tower-lsp-max` workspace compiles cleanly with the upgraded local `lsp-types` crate.
- [ ] All unit, integration, and doc tests in the `tower-lsp-max` workspace pass successfully.
