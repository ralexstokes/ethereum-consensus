# ethereum-consensus

A library for interacting with ethereum consensus data.

# 🚧 WARNING 🚧

This implementation has **not** been audited for security and is primarily intended for R&D use cases.

If you need a battle-tested implementation (e.g. for consensus-critical work), refer to the [Lighthouse implementation](https://github.com/sigp/lighthouse).

# Notes

## `ethereum-consensus`

The `ethereum-consensus` crate exposes a series of modules implementing the [ethereum consensus specs](https://github.com/ethereum/consensus-specs).

There are a number of top-level modules exposing a variety of supporting types, utilities, etc.

And then a module to drive any state transition in `state_transition`.

Specialized logic for each fork can be found in the module with the same name as the fork, e.g. `bellatrix`.

Each fork supports a compile-time "preset", with two common presets `mainnet` and `minimal` provided as a convenience.
These modules can be found under each fork module in the `presets` module.

The generic types are exposed but most users will want to access each fork's logic via one of the presets. See
[Examples](#examples) for further details.

An important thing to note is that the `state_transition` module of each fork (after the `phase0` fork) is generated
by a code-generation utility in the `spec-gen` crate. This utility specializes each fork based on the prior
Rust module as an input. See the README for that binary to learn further details about operating this utility. The
generated files are checked in so you should not need to use this binary under
most circumstances.

### Examples

Refer to the code in `examples` for the suggested way to use this crate as a user. The intermediate types are laid out
so that you can define your own customizations but you can likely use the defaults.

### Support for presets

These crates provide support for the "preset" concept found in the `consensus-specs`. The `minimal` and `mainnet` presets are provided for each fork as hard-coded instances. If you need to support another type of preset, you can make your own using the generic types. Refer to an existing preset, like `ethereum_consensus::bellatrix::presets::minimal`, for an example.

### Support for networks

These crates also support several popular networks. This generally boils down to specific config, for example `ethereum_consensus::configs::goerli::Config`.

To add support for a new network, you can:

1.  add a new module under `ethereum_consensus::configs` using an existing network as a template
2. add the network's `genesis_time` and support for a `Clock` for that network in `ethereum_consensus::clock`
3. there are convenience methods on `ethereum_consensus::state_transition::Context` for each network and these should also be updated for the new network

## `beacon-api-client`

A client for the Ethereum beacon node APIs:

https://ethereum.github.io/beacon-APIs

### Examples

Refer to the code in `examples` for a demonstration on how to use the API client.

## `spec-gen`

This crate generates spec code following the "fork diff" defined in each fork of http://github.com/ethereum/consensus-specs. It is not user-facing.
