# LSP Types Conformance Matrix

This matrix evaluates the `lsp-types` repository against LSP Base Protocol 0.9, LSP 3.18, and LSIF 0.6.0 specifications.

| Item | Spec Section | Status |
| --- | --- | --- |
| **LSP Base Protocol 0.9** | | |
| URI / DocumentUri | Base Protocol 0.9 - Base Types | CONFORMANT |
| Position / Range / Location | Base Protocol 0.9 - Basic JSON Structures | CONFORMANT |
| RequestMessage | Base Protocol 0.9 - Request Message | CONFORMANT |
| ResponseMessage | Base Protocol 0.9 - Response Message | CONFORMANT |
| ResponseError | Base Protocol 0.9 - Response Error | CONFORMANT |
| NotificationMessage | Base Protocol 0.9 - Notification Message | CONFORMANT |
| CancelParams / `$/cancelRequest` | Base Protocol 0.9 - Cancellation Support | CONFORMANT |
| **LSP 3.18** | | |
| Initialize / Initialized / Shutdown / Exit | LSP 3.18 - Lifecycle Messages | CONFORMANT |
| ShowMessage / LogMessage / Telemetry | LSP 3.18 - Window Features | CONFORMANT |
| WorkspaceFolders / Configuration | LSP 3.18 - Workspace Features | CONFORMANT |
| Diagnostic / CodeAction / CodeLens | LSP 3.18 - Language Features | CONFORMANT |
| SemanticTokens / InlayHint / InlineValue | LSP 3.18 - Language Features | CONFORMANT |
| NotebookDocument / Sync | LSP 3.18 - Notebook Features | CONFORMANT |
| InlineCompletion | LSP 3.18 - Proposed Features | CONFORMANT |
| TypeHierarchy | LSP 3.18 - Type Hierarchy | CONFORMANT |
| **LSIF 0.6.0** | | |
| Element / Vertex / Edge | LSIF 0.6.0 - Element, Vertex, Edge | CONFORMANT |
| RangeTag (Definition, Declaration, etc.) | LSIF 0.6.0 - Range Tags | CONFORMANT |
| Encoding (utf-16) | LSIF 0.6.0 - MetaData | CONFORMANT |
| DocumentSymbolOrRangeBasedVec | LSIF 0.6.0 - Document Symbol Result | CONFORMANT |

### Non-Conformant Items Detail

#### 1. JSON-RPC Message Wrappers (RequestMessage, ResponseMessage, NotificationMessage, ResponseError)
- **Status:** FIXED (Was: MISSING)
- **Affected Crate/Module:** `lsp-types` (Base crate)
- **Expected Spec Behavior:** The Base Protocol 0.9 specification defines explicit JSON-RPC structures for wrapping method calls and responses.
- **Observed Workspace Behavior:** Fixed by introducing these structs in `src/base.rs` and exposing them through `src/lib.rs`.
- **Test File:** `tests/base_protocol_tests.rs` (Includes positive and negative golden fixtures)

#### 2. LSIF Encoding
- **Status:** FALSE ALARM (Is: CONFORMANT)
- **Expected Spec Behavior:** LSIF 0.6.0 explicitly states "Currently only 'utf-16' is support due to the limitations in LSP."
- **Observed Workspace Behavior:** The crate appropriately strictly implements 'utf-16'. No changes needed.

#### 3. InlineCompletion
- **Status:** FALSE ALARM (Is: CONFORMANT)
- **Expected Spec Behavior:** Inline completion is slated as a *proposed* feature in LSP 3.18.
- **Observed Workspace Behavior:** The implementation is correctly gated behind `#[cfg(feature = "proposed")]`. No changes needed.

#### 4. LSIF Range Tags
- **Status:** FALSE ALARM (Is: CONFORMANT)
- **Expected Spec Behavior:** LSIF 0.6.0 specifically defines exactly four valid type tags for ranges: `declaration`, `definition`, `reference`, and `unknown`.
- **Observed Workspace Behavior:** The crate natively implements these exact four type tags. No changes needed.
