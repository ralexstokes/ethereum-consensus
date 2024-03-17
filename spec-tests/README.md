# Tests from ethereum consensus spec tests

Refer to the [consensus spec tests repo](https://github.com/ethereum/consensus-spec-tests) for more information.

## Updating spec test version

The current supported spec test version corresponds to releases in the `consensus-spec-tests` repo.

To update to a new version, update the text file [spec-test-version](./spec-test-version).

For example, to update to [spec tests release v1.4.0](https://github.com/ethereum/consensus-spec-tests/releases/tag/v1.4.0), the file should simply read:

```bash
v1.4.0
```

## Running tests

In order to run the tests against the consensus spec test repo, use the following commands:

```
just download-integration-tests
just test
```

## Notes

The tests for `ssz_generic` handler are present in the [`ssz-rs` repo](https://github.com/ralexstokes/ssz-rs).
