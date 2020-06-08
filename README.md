# Substrate SSVM Node

The primary objective of this project was to bring the Ethereum flavored WebAssembly (EWASM) runtime to Substrate. It enables developer collaboration between the Ethereum and Polkadot/Substrate ecosystems, and promotes inter-blockchain interoperability at the application level. You can now create a Substrate node (or Polkadot parachain) that supports the deployment and execution of EWASM smart contracts.

The project has a few components and dependencies.

1. The [EWASM Test Guide](https://github.com/second-state/rust-ssvm/blob/master/docs/EWASM_TEST.md) shows how to use the official EWASM test suite to verify the SSVM EWASM engine.
2. The [rust-ssvm](https://github.com/second-state/rust-ssvm) project provides a Rust EVMC interface for SSVM.
3. The [pallet-ssvm](https://github.com/second-state/pallet-ssvm) project creates a "pallet" (or substrate package) that uses rust-ssvm as the substrate node's EWASM engine.
4. The [substrate-ssvm-node](https://github.com/second-state/substrate-ssvm-node) project (this project) provides a full substrate node that incorporates the pallet-ssvm.
5. The [command line tools](https://github.com/second-state/substrate-cli-tools) is an extension to the substrate CLI to support substrate-ssvm-node. It is the same approach as evm pallet.

## Getting Started

Use [secondstate/substrate-ssvm](https://hub.docker.com/r/secondstate/substrate-ssvm) as building & running environment:

```bash
> git clone https://github.com/second-state/substrate-ssvm-node.git
> docker run -it --rm \
  --name ssvm \
  -v $(pwd):/root/node \
  -w /root/node \
  secondstate/substrate-ssvm
(docker) cd substrate-ssvm-node
(docker) cargo run --release --bin node-template -- --dev
```

See our documents for more information about interacting with node:
  - Use command line tools
    - [Demo video](https://drive.google.com/open?id=149AgZkvXeQZEAlZlNQ7bcZ8V2zCjpQf6)
    - [Doc](./docs/interact-using-cli.md)
  - Use Substrate web interface
    - [Demo video](https://drive.google.com/open?id=119L2oCVuaGAZJ1yQj3YVsKAepc0DcFq-)
    - [Doc](./docs/interact-using-web.md)
