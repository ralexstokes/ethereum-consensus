test:
    cargo test
build-integration-tests:
    git clone https://github.com/ethereum/consensus-spec-tests.git
    cd consensus-spec-tests
    git lfs install
    git lfs pull
run-integration-tests:
    cargo test --test '*'
fmt:
    cargo fmt
lint: fmt
    cargo clippy
build:
    cargo build
run-ci: lint build test
clean-tests:
    rm -rf consensus-spec-tests
