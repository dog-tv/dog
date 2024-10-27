fmt:
    cargo +nightly fmt

clippy:
    cargo clippy

build:
    cargo build --release --all-targets

test:
    cargo test --release
