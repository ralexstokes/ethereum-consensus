# Tests from Ethereum Consensus Spec Tests

The consensus spec tests repo is [here](https://github.com/ethereum/consensus-spec-tests).

This repo has been added as a submodule. The test data is stored using the Github Large File System (LFS) and will not be automatically downloaded if you just clone the current repo. To get the test data you have to do

```
git lfs install
git lfs pull
``` 

The tests are currently behind a feature `ef_spec_tests` which is enabled by default in `Cargo.toml`. If you don't want to run these tests, just turn it off there.

## Existing tests
* **bls_tests.rs for tests/general/phase0/bls**

The description of the specs for this category can be found [here](https://github.com/ethereum/consensus-specs/tree/master/tests/formats/bls).

Currently there are 5 types of test data for different functions: `sign`, `verify`, `aggregate`, `aggregate_verify` and `fast_aggregate_verify`. The basic testing pattern is as follows:

1. Define a custom input-output struct that matches the test data organization. The output part is one variable but the input part uses another struct to capture different input fields (publickeys, messages, signatures).

2. Read-in the test data using `serde_yaml` and parse it into the struct while accounting for hexadecimal encoding with `hex::decode`.

3. Apply the function of interest (imported from [crypto.rs](https://github.com/ralexstokes/ethereum_consensus/blob/main/src/crypto.rs) and implemented for the input-output struct) and assert that the function output matches the listed test data output.