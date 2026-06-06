# LSP 3.18.0 End-to-End Test Infrastructure

This document outlines the testing philosophy, feature inventory, architecture, and coverage goals for the E2E verification of the LSP 3.18.0 specification upgrade in `lsp-types`.

---

## 1. Test Philosophy

Our testing approach guarantees correctness and robustness through three core principles:

1. **Opaque-Box Testing**: The test suite treats the `lsp-types` crate as a black box, verifying only through its public API surface (serialization and deserialization) rather than relying on internal implementation details.
2. **Requirement-Driven Verification**: Every test case directly traces back to specific requirements in the LSP 3.18.0 specification to ensure complete compliance.
3. **Key-Order-Agnostic Serialization Round-Tripping**: JSON serializers are free to order map keys arbitrarily. The test suite uses structural JSON comparisons (`serde_json::Value`) rather than exact string matching. This ensures key-order-agnostic validation of:
   - **Serialization**: Verifying that Rust data structures generate structurally compliant JSON shapes.
   - **Deserialization**: Verifying that compliant JSON structures map back into exact, identical Rust representations.

---

## 2. Feature Inventory

The test suite covers all 11 LSP 3.18.0 features defined in the upgrade scope:

1. **`textDocument/inlineCompletion`**: Support for inline completion requests, registration options, and client/server capabilities that enable ghost text suggestions in editors.
2. **`workspace/textDocumentContent` & `workspace/textDocumentContent/refresh`**: Support for virtual document content retrieval using custom schemas and server-initiated content refresh requests.
3. **`textDocument/rangesFormatting`**: Ability to request formatting for multiple disjoint selections/ranges of a document in a single request.
4. **`workspace/foldingRange/refresh`**: Infrastructure for the server to request recalculation of folding ranges by the client.
5. **Notebook Document Synchronization**: Comprehensive synchronization structures for notebook open, change, save, and close notifications, keeping cell states aligned between client and server.
6. **Cell Diagnostic Pull**: Extend the diagnostic pull model to notebook cells, allowing clients to request diagnostics for specific cells or whole notebooks.
7. **Proposed `CodeActionKind.RefactorMove` and `CodeActionTag.LLMGenerated`**: Support for moving refactoring kinds and marking code actions as AI/LLM-generated.
8. **`SnippetTextEdit` and `WorkspaceEditMetadata`**: Support for edits that can insert snippets (with tab stops/placeholders) and workspace edit metadata for user-facing documentation and annotations.
9. **`CompletionList.itemDefaults`**: Optimized payload size using list-level defaults (for data, edit range, apply kind, etc.) that merge into individual completion items when omitted.
10. **`MessageType.Debug`**: Support for a dedicated debugging message type in window/log notification APIs.
11. **Relative Glob Patterns**: Document selectors capable of matching relative to a base workspace URI or directory path.

---

## 3. Test Architecture

To avoid compilation overhead and maintain clean separation of concerns, the test infrastructure is designed as follows:

- **Unified Test File**: All integration tests reside in a single test target: `tests/e2e_spec_3_18.rs`. This prevents Cargo from compiling dozens of separate binaries, keeping the build pipeline fast and efficient.
- **Verification Helper**: The test suite utilizes a custom key-order-agnostic round-trip helper:
  
  ```rust
  fn assert_serde_roundtrip<T>(value: &T, expected_json: &str)
  where
      T: serde::Serialize + for<'de> serde::Deserialize<'de> + std::fmt::Debug + PartialEq,
  {
      // Serialize Rust struct to JSON string
      let serialized = serde_json::to_string(value).expect("failed to serialize value");
      
      // Compare structurally as serde_json::Value (key-order independent)
      let parsed_serialized: serde_json::Value = serde_json::from_str(&serialized)
          .expect("failed to parse serialized JSON");
      let parsed_expected: serde_json::Value = serde_json::from_str(expected_json)
          .expect("failed to parse expected JSON");
      assert_eq!(parsed_serialized, parsed_expected, "structural mismatch between serialized and expected JSON");

      // Deserialize back to T and verify equality
      let deserialized: T = serde_json::from_str(expected_json)
          .expect("failed to deserialize expected JSON");
      assert_eq!(*value, deserialized, "value mismatch after deserialization");
  }
  ```

- **Feature Gating**: Features that are proposed or conditional on the crate features use `#[cfg(feature = "proposed")]` to compile and execute only when configured.

---

## 4. Real-World Application Scenarios (Tier 4)

To verify real-world application compliance, the suite implements tests simulating at least 5 realistic scenarios:

1. **Scenario 1: AI-Assisted Inline Completion**
   - *Description*: An editor requests inline suggestions as a user types. The server replies with an inline completion containing snippet edits, utilizing `textDocument/inlineCompletion` and marking the results with LLM-generated metadata/tags.
2. **Scenario 2: Disjoint Multi-Selection Formatting**
   - *Description*: A developer highlights three non-adjacent blocks of code and triggers formatting. The client sends a single `textDocument/rangesFormatting` request, and the server returns localized text edits for each range.
3. **Scenario 3: Interactive Notebook Document Editing**
   - *Description*: A user opens a Jupyter notebook. The editor synchronizes notebook state changes (adding, modifying, and moving cells) using notebook document synchronization, and pulls diagnostic reports for individual cells using cell diagnostic pull APIs.
4. **Scenario 4: Custom Virtual Document Content Generation**
   - *Description*: A user clicks "Go to decompiled definition". The client uses `workspace/textDocumentContent` with a custom schema (`jar:` or `git:`) to pull the read-only generated source. When the source compiles or updates, the server notifies the client via `workspace/textDocumentContent/refresh` to reload.
5. **Scenario 5: High-Performance Auto-Complete Payload**
   - *Description*: A client requests autocomplete options. The server returns hundreds of items. To optimize transmission bandwidth, the server populates `CompletionList.itemDefaults` with shared fields (e.g. data contexts, edit ranges), and the client resolves item-level overrides.

---

## 5. Coverage Thresholds

The test suite enforces four distinct tiers of testing to guarantee complete coverage:

| Tier | Focus | Threshold Requirement | Minimum Count |
|---|---|---|---|
| **Tier 1** | **Serialization & Structural Integrity** | Verifies basic round-trip serialization/deserialization of every new struct and enum. | >= 5 tests per feature (55 total) |
| **Tier 2** | **Edge Case & Boundary Testing** | Verifies empty collections, optional/None fields, maximum limits, and boundary cases. | >= 5 tests per feature (55 total) |
| **Tier 3** | **Pairwise & Interaction Testing** | Verifies combinations where two features interact (e.g. SnippetTextEdit inside InlineCompletion). | >= 11 pairwise tests |
| **Tier 4** | **Real-World Application Scenarios** | Verifies end-to-end integration flows simulating editor actions. | >= 5 scenario tests |
| **Total** | **Full Suite Completeness** | Sum of all tests across all four tiers. | **>= 126 tests** |
