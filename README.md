# Substrate SSVM Node ![build](https://github.com/second-state/substrate-ssvm-node/workflows/build/badge.svg)

The primary objective of this project was to bring the Ethereum flavored WebAssembly (EWASM) runtime to Substrate. It enables developer collaboration between the Ethereum and Polkadot/Substrate ecosystems, and promotes inter-blockchain interoperability at the application level. You can now create a Substrate node (or Polkadot parachain) that supports the deployment and execution of EWASM smart contracts.

The project has a few components and dependencies.

1. The [EWASM Test Guide](https://github.com/second-state/rust-ssvm/blob/master/docs/EWASM_TEST.md) shows how to use the official EWASM test suite to verify the SSVM EWASM engine.
2. The [ssvm-evmc](https://github.com/second-state/evmc) project provides a Rust EVMC binding for SSVM. ([crates.io](https://crates.io/crates/ssvm-evmc-client/6.3.1-rc4))
3. The [rust-ssvm](https://github.com/second-state/rust-ssvm) project provides a Rust interface on SSVM. ([crates.io](https://crates.io/crates/rust-ssvm/0.1.0-rc2))
4. The [pallet-ssvm](https://github.com/second-state/pallet-ssvm) project creates a "pallet" (or substrate package) that uses rust-ssvm as the substrate node's EWASM engine. ([crates.io](https://crates.io/crates/pallet-ssvm/0.1.0-rc2))
5. The [substrate-ssvm-node](https://github.com/second-state/substrate-ssvm-node) project (this project) provides a full ssvm-node that incorporates the pallet-ssvm.

## Getting Started

### Environment (Docker)

We provide a docker image [secondstate/substrate-ssvm](https://hub.docker.com/r/secondstate/substrate-ssvm) for building and running the Substrate SSVM Node.

```bash
> docker pull secondstate/substrate-ssvm
```

### Build and Run Substrate SSVM Node

```bash
> git clone https://github.com/second-state/substrate-ssvm-node.git
> docker run -it --rm \
  --name substrate-ssvm \
  -v $(pwd):/root/node \
  -w /root/node \
  -p 9944:9944 \
  secondstate/substrate-ssvm
(docker) cd substrate-ssvm-node
(docker) make init
(docker) make build
(docker) cargo run --release --bin ssvm-node -- --dev --ws-external
```

### Interact with Substrate SSVM Node

You can use the [Substrate Web Interface(Polkadot.js)](https://polkadot.js.org/apps) to connect with the node.

We provide a [demo video](https://drive.google.com/open?id=1sR41n8fdLJD66Skcq8f7hyLRRQzSX9KF) to show that how polkadot.js interacts with our nodes.

And the detailed steps can be found in this [tutorial](./docs/interact-using-web.md).
