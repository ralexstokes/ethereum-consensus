# Tests from Ethereum Consensus Spec Tests

The consensus spec tests repo is [here](https://github.com/ethereum/consensus-spec-tests).

This repo has been added as a submodule. The test data is stored using the Github Large File System (LFS) and will not be automatically downloaded if you just clone the current repo. To get the test data you have to do

```
git lfs install
git lfs pull
``` 

The tests are currently behind a feature `spec-tests` which is enabled by default in `Cargo.toml`. If you don't want to run these tests, you can run `cargo` with the default features disabled.
