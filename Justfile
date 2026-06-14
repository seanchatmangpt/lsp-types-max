test:
    cargo test --workspace

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
