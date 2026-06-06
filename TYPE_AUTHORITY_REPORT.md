# Type Authority Report

**Assessment:**
Should `lsp-types-max` (known on crates.io as `lsp-types`) become the canonical spec-type crate for LSIF 0.6.0, LSP Base 0.9, and LSP 3.18?

Yes, `lsp-types` is widely adopted and currently acts as the canonical data structure crate for LSP in Rust. However, its separation of concerns means that runtime behavior (JSON-RPC handling) often belongs in downstream crates like `tower-lsp` or `lsp-server`. To become the true "canonical spec-type crate" for LSP Base 0.9, it must add the missing JSON-RPC structural envelopes (`RequestMessage`, `ResponseMessage`, etc.), establishing a clear boundary between defining static types and actively routing behavior.

### Type Classification

| Type | Classification | Rationale |
| --- | --- | --- |
| **InitializeParams** | pure spec type | Standard payload defined entirely by LSP 3.18. Strong candidate for `lsp-types-max`. |
| **InitializeResult** | pure spec type | Standard payload defined entirely by LSP 3.18. |
| **RequestMessage / ResponseMessage** | mixed type/behavior | These are JSON-RPC spec types, but their serialization often requires runtime connection state or custom dispatch mapping. Should be split: plain struct in `lsp-types-max`, router logic in downstream server crates. |
| **DocumentUri / Uri** | pure spec type | Fundamental data type used across all capabilities. |
| **LanguageServer trait** | runtime/server behavior | Implements server-side event loops and request handlers. Must remain in `tower-lsp-max`. |
| **LSIF Element / Vertex / Edge** | pure spec type | Strictly defined by the LSIF 0.6.0 schema. Excellent fit for `lsp-types-max`. |
| **NotebookDocumentSyncOptions** | pure spec type | Represents capability negotiation for notebook sync. |
| **WorkDoneProgressReport** | pure spec type | Data structure for progress reporting. |
| **ErrorCodes** | pure spec type | Enumeration of JSON-RPC and LSP-specific error integers. |
| **Dispatcher / Connection** | runtime/server behavior | Networking and channel routing logic. Must not be in `lsp-types-max`. |
| **ResponseError** | pure spec type | Defined by JSON-RPC / Base 0.9. Should be added to `lsp-types-max` as a pure data struct. |
| **LSIF tool_info / args** | pure spec type | Defined statically in LSIF metadata. |
| **StaleRequestSupportClientCapabilities** | duplicate/stale type | Needs reconciliation if multiple variants exist across legacy/proposed implementations. |

### Recommendations for `lsp-types-max`
1. **Unify Base Protocol 0.9 Types:** Bring `RequestMessage`, `ResponseMessage`, and `NotificationMessage` into this crate as pure `Serialize`/`Deserialize` data structs.
2. **Promote Proposed Features:** Audit and remove `proposed` feature gates for LSP 3.18 features that have reached consensus.
3. **Canonicalize LSIF:** Maintain `lsif.rs` strictly in accordance with LSIF 0.6.0 to serve as the unified indexing format authority.
