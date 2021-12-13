[![Crates.io](https://img.shields.io/crates/v/pdotc?style=flat-square)](https://crates.io/crates/pdotc)
[![docs.rs](https://img.shields.io/docsrs/pdotc?style=flat-square)](https://docs.rs/pdotc/latest/pdotc)
[![CI](https://github.com/trevor-crypto/pdotc/actions/workflows/rust.yml/badge.svg)](https://github.com/trevor-crypto/pdotc/actions/workflows/rust.yml)

# pdotc

### A Minimal Polkadot/Westend Client

Inspired by [`substrate-api-client`](https://github.com/scs/substrate-api-client) with some logic copied from substrate crates

## Features
- Sync http client agnostic
- Perform a simple transaction
- Perform general staking functions
- Get an account balance
- Get the fee for an extrinsic

## Goals
- To not rely on many substrate crates
- To not use substrate metadata and be easily, manually upgradable
- To be minimal and not use many dependencies

## Usage
See [examples](examples/client.rs)
