test:
    cargo test --workspace

# Drift characterization against the actively-worked sibling tree (read-only).
drift:
    sh scripts/tree_drift.sh . "$(HOME)/lsp-types-max"

drift-strict:
    sh scripts/tree_drift.sh . "$(HOME)/lsp-types-max" --strict

test-full:
    cargo test --workspace -- --include-ignored

polish:
    cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings

build:
    cargo build --workspace

clean:
    cargo clean

publish: polish test-full
    cargo publish

ci: polish test-full
