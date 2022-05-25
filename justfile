test:
    cargo test
build-integration-tests:
    #!/usr/bin/env sh
    TESTS_TAG=v1.1.10
    REPO_NAME=consensus-spec-tests
    OUTPUT_DIR=./${REPO_NAME}
    wget https://github.com/ethereum/${REPO_NAME}/releases/download/${TESTS_TAG}/general.tar.gz
    wget https://github.com/ethereum/${REPO_NAME}/releases/download/${TESTS_TAG}/mainnet.tar.gz
    wget https://github.com/ethereum/${REPO_NAME}/releases/download/${TESTS_TAG}/minimal.tar.gz
    mkdir ${REPO_NAME}
    tar -xzf general.tar.gz -C ${REPO_NAME}
    tar -xzf mainnet.tar.gz -C ${REPO_NAME}
    tar -xzf minimal.tar.gz -C ${REPO_NAME}

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
