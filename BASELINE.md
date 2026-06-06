# Baseline Integrity Report

## Tests (`cargo test --workspace`)
- Tests executed: 25
- Passed: 25
- Failed: 0
- Ignored: 1 (doc test)
- Warnings during compilation:
  - `struct WorkDoneProgressOptions is never constructed` in `src/progress.rs:46:12`

## Clippy (`cargo clippy --workspace`)
- Warnings: 4
  1. `struct WorkDoneProgressOptions is never constructed` (`src/progress.rs:46:12`)
  2. `large size difference between variants` (`src/code_action.rs:134:1` on `CodeActionOrCommand`)
  3. `doc list item without indentation` (`src/inline_value.rs:135:5`)
  4. `mutable key type` (`src/lib.rs:831:25` on `HashMap<Uri, Vec<TextEdit>>`)

## Goal
No *new* warnings or failing tests may be introduced relative to this baseline during the conformance audit. Fixes to existing warnings are welcome if they align with resolving conformance issues.