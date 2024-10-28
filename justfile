fmt:
    cargo +nightly fmt
    pre-commit run -a

clippy:
    cargo clippy

build:
    cargo build --release --all-targets

camera_sim:
    cargo run --example camera_sim --release --features std

viewer_ex:
    cargo run --example viewer_ex --release --features std

test:
    cargo test --release
