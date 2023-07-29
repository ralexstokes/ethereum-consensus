gen-spec:
    @echo "this is broken right now, sorry"
    # cargo run --features gen-spec --bin gen-spec
    # cargo fix --allow-dirty
    # cargo fmt

run-integration-tests:
    cargo test --features 'spec-tests' --test '*'
test:
    # NOTE: do not test `--all-features` here to only run unit tests
    # partition much heavier "integration tests" to a separate command
    cargo test
fmt:
    cargo +nightly fmt --all
lint: fmt
    cargo +nightly clippy --all-targets --all-features
build:
    cargo build --all-targets --all-features
run-ci: lint build test
