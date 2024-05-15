# substrate-spec-version

[![Crates.io](https://img.shields.io/crates/v/substrate-spec-version)](https://crates.io/crates/substrate-spec-version)
[![docs.rs](https://img.shields.io/docsrs/substrate-spec-version?label=docs)](https://docs.rs/substrate-spec-version/latest/substrate-spec-version/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/sam0x17/substrate-spec-version/ci.yaml)](https://github.com/sam0x17/substrate-spec-version/actions/workflows/ci.yaml?query=branch%3Amain)
[![MIT License](https://img.shields.io/github/license/sam0x17/substrate-spec-version)](https://github.com/sam0x17/substrate-spec-version/blob/main/LICENSE)

An extremely simple CLI utility that uses subxt to query a live substrate/polkadot-sdk-based
blockchain based on the websocket URL for the blockchain and display the current spec version
as output. Very useful for CI situations where knowing the live spec version on-chain is
desirable.

## Installation

```sh
cargo install substrate-spec-version
```

## Usage

```sh
substrate-spec-version wss://rpc.polkadot.io
100200
```
