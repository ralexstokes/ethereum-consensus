# `spec-gen`

Generates Rust source of each fork of the `consensus-specs` based on prior forks
and the patches applied for each target fork.

## How to use

From the workspace root:

```bash
just gen-spec
```

## Structure

Each fork has the following structure:

A "fork diff" that implements the new logic to be applied on top of the previous fork, written as (private) modules in the top-level under the forks' top-level module. For example, `block_processing` in `phase0`.

Modules to add to a fork diff as discovered by adding them to the data returned from the `Fork::modules` function defined in this tool.

A "fork diff" is applied, on top of any previous specs to generate the target spec which is written as a single file under the fork's `spec` module. For example `phase0::spec` under `phase0/spec/mod.rs`. The choice of the nested `mod.rs` file is arbitrary and mainly used to obscure the generated file (if done properly, the user shouldn't really need to think about it).

Further, each fork has "presets", like `mainnet`, which fix all of the type generics with the appropriate values defined in the preset. This preset pulls in everything defined in the `spec` module and then specializes the relevant types so that a user can simply pull from a preset and not have to bother with any generic types.

In general, an author of a new fork should only expose the presets, generated spec and any other modules not touched the spec generator tool.

## Steps to add a new fork

Write the `BeaconState` and `BeaconBlock` types. These tend to change from fork to fork and will almost certainly touch many of the supporting functions from earlier specs.

Run the generator. This should write a "partial" spec in the `spec` module of the new fork and any functionality from a previous fork should be found here, ready to be used with the new fork's types (e.g. the new `BeaconState`).

The spec generator should be run a final time once all of the new fork's functionality is in place.
