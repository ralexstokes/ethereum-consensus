test:
    cargo test
download-integration-tests: clean-integration-tests
    #!/usr/bin/env sh
    TESTS_TAG=v1.1.10
    REPO_NAME=consensus-spec-tests
    CONFIGS="general minimal mainnet"
    mkdir ${REPO_NAME}
    for config in ${CONFIGS}
    do
        wget https://github.com/ethereum/${REPO_NAME}/releases/download/${TESTS_TAG}/${config}.tar.gz
        tar -xzf ${config}.tar.gz -C ${REPO_NAME}
    done
    rm -f *tar.gz
clean-integration-tests:
    rm -rf consensus-spec-tests
run-integration-tests:
    cargo test --features 'spec-tests' --test '*'
fmt:
    cargo fmt
lint: fmt
    cargo clippy --all-features
build:
    cargo build --all-features
gen-spec:
    cargo run --features gen-spec --bin gen-spec
    cargo fix --allow-dirty
    cargo fmt
gen-tests:
    cargo run --features gen-tests --bin gen-tests
    cargo fmt
run-ci: lint build test
