# `test-gen`

Generates `rustc` tests that correspond to each test case in the `consensus-spec-tests`.

## How to use

```bash
just download-integration-tests
just gen-tests
```

## How to update

The `test-gen` utility provides the exact same set of effects for a given version of the `consensus-spec-tests`
and so should only need to be run when the underlying test corpus changes.

To update to a newer set of tests, update the [`spec-test-version`](./spec-test-version) file and re-run the above steps.
