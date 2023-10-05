download-integration-tests: clean-integration-tests
    #!/usr/bin/env sh
    TESTS_TAG=$(cat test-gen/spec-test-version)
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
gen-tests:
    cargo run -p test-gen --bin test-gen
    just fmt

gen-spec:
    cargo run -p spec-gen -- forks
    just fmt

gen-types:
    cargo run -p spec-gen -- types
    just fmt

run-integration-tests:
    cargo test --features 'spec-tests' --test '*'
test:
    # NOTE: do not test `--all-features` here to only run unit tests
    # partition much heavier "integration tests" to a separate command
    cargo test --features ec
fmt:
    cargo +nightly fmt --all
lint: fmt
    cargo +nightly clippy --all-targets --all-features
build:
    cargo build --all-targets --all-features
run-ci: lint build test
