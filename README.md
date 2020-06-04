# Substrate SSVM Node

The primary objective of this project was to bring EWASM runtime from Ethereum community to Substrate to extend substrate ecosystem. Here we will demonstrate how to establish a substrate node that binding SSVM by EVMC.

Our achievement is divided into the following several parts.

1. Provdie a [EWASM Test Guide](https://github.com/second-state/rust-ssvm/blob/master/docs/EWASM_TEST.md) show how to use official EWASM test suite verify our EWASM engine (SSVM).
2. Create [rust-ssvm](https://github.com/second-state/rust-ssvm), it's SSVM with a rust interface through EVMC.
3. Create a SRML [pallet-ssvm](https://github.com/second-state/pallet-ssvm) use rust-ssvm as substrate node's EWASM engine.
4. Integrate pallet-ssvm to [substrate-ssvm-node](https://github.com/second-state/substrate-ssvm-node).
5. Extend [command line tools](https://github.com/second-state/substrate-cli-tools) for support substrate-ssvm-node just like evm pallet did.
6. Demonstrate how to deploy ewasm bytecode and interact with the contract.

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
