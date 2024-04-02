gen-spec:
    cargo run -p spec-gen -- forks
    just fmt

gen-types:
    cargo run -p spec-gen -- types
    just fmt

test:
    cargo test --all-features --all-targets --workspace --exclude spec-tests
run-spec-tests filter="":
    cargo test -p spec-tests {{filter}}
fmt:
    cargo +nightly fmt --all
lint: fmt
    cargo +nightly clippy --all-targets --all-features
build:
    cargo build --all-targets --all-features
run-ci: lint build test
ec +command:
    cargo run -p ethereum-consensus --features ec {{command}}
