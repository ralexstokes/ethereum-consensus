test:
    cargo test
fmt:
    cargo +nightly fmt --all
lint: fmt
    cargo +nightly clippy --all-targets
build:
    cargo build --all-targets
run-ci: lint build test
