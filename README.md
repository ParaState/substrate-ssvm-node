# Substrate SSVM Node

The primary objective of this project was to bring EWASM runtime from Ethereum community to Substrate to extend substrate ecosystem. Here we will demonstrate how to establish a substrate node that binding SSVM by EVMC.

Our achievement is divided into the following serval parts.

1. Provdie a [EWASM Test Guide](https://github.com/second-state/rust-ssvm/docs/EWASM-TEST.md) show how to use official EWASM test suite verify our EWASM engine (SSVM).
2. Create [rust-ssvm](https://github.com/second-state/rust-ssvm), it's SSVM with a rust interface through EVMC.
3. Create a SRML [pallet-ssvm](https://github.com/second-state/pallet-ssvm) use rust-ssvm as substrate node's EWASM engine.
4. Integrate pallet-ssvm to [substrate-ssvm-node](https://github.com/second-state/substrate-ssvm-node).
5. Extend [command line tool](https://github.com/second-state/substrate-cli-tools) for support substrate-ssvm-node just like evm pallet did.
6. Demonstrate how to deploy ewasm bytecode and interact with the contract.

## Build

To build SSVM enabled Substrate node, we need the following repositories:

```bash
> git clone https://github.com/second-state/substrate-ssvm-node.git
> git clone https://github.com/second-state/substrate-cli-tools.git
```

and use [secondstate/ssvm](https://hub.docker.com/r/secondstate/ssvm) as building environment:

```bash
> docker run -it --rm \
  --name ssvm \
  -v $(pwd):/root/node \
  -w /root/node \
  secondstate/substrate-ssvm
```

Build substrate-cli-tools:

```bash
(docker) cd ~/node/substrate-cli-tools/typescript
(docker) yarn install && yarn run tsc
(docker) chmod +x dist/*.js
```

Build substrate-ssvm-node:

```bash
(docker) cd ~/node/substrate-ssvm-node
(docker) cargo build --release --verbose
```

## Run

After building substrate-ssvm-node, we could start a Substrate node that supports Ewasm by our [pallet-ssvm](https://github.com/second-state/pallet-ssvm):

```bash
(docker) cargo run --release --bin node-template -- purge-chain --dev -y
(docker) cargo run --release --bin node-template -- --dev
```

You could also start a new shell in the same docker container and check RPC information or listen events from pallet-ssvm:

```bash
> docker exec -it ssvm bash
(docker-2) ~/node/substrate-cli-tools/typescript/dist/info.js
(docker-2) ~/node/substrate-cli-tools/typescript/dist/events.js +ssvm
```

## Deploy contract

Now we could use substrate-cli-tools to deploy Ewasm contract to our node.

Here we use the following ERC20 contract files as an example:
- [erc20.sol](./erc20/erc20.sol)
    - This file is an ERC20 contract written in Solidity.
- [erc20.wasm](./erc20/erc20.wasm)
    - This file is a wasm file generate from `erc20.sol` by [SOLL](https://github.com/second-state/soll)
    - Command to generate wasm file: `soll -deploy=Normal erc20.sol`
- [erc20.hex](./erc20/erc20.hex)
    - To deploy wasm file to our node, we need to convert `erc20.wasm` to hex.
    - Command to generate hex file: `xxd -p erc20.wasm | tr -d $'\n' > erc20.hex`


Again, start a new shell in the same docker container. Now we could deploy the wasm bytecode to our node:

> **Make sure you started node before doing the following steps.**

```bash
> docker exec -it ssvm bash
(docker-3) cd ~/node/substrate-ssvm-node/erc20
(docker-3) ~/node/substrate-cli-tools/typescript/dist/ssvm.js create -p 99 -g 5000000 -c 0x$(cat erc20.hex)
```

The contract will be deployed at address `0xe2a313e210a6ec1d5a9c0806545670f2e6264f86`.
You could check it from `(docker-2)` shell, which runs `events.js +ssvm`:

```bash
[ssvm] Create (phase={"ApplyExtrinsic":1})
        H160: 0xe2a313e210a6ec1d5a9c0806545670f2e6264f86
(1 matching events received)
```

## Interact with contract

You could check Ewasm addresses of Alice and Bob using `balances.js --svm`:

```bash
(docker-3) ~/node/substrate-cli-tools/typescript/dist/balances.js --ssvm //Alice
//Alice's SSVM address is 0x9621dde636de098b43efb0fa9b61facfe328f99d
//Alice's balance is 1000000

(docker-3) ~/node/substrate-cli-tools/typescript/dist/balances.js --ssvm //Bob
//Bob's SSVM address is 0x41dccbd49b26c50d34355ed86ff0fa9e489d1e01
//Bob's balance is 0
```

We'd like to call `balanceOf(address)` and `transfer(address,uint256)`. Use `ssvm.js selector` to get Ewasm function signature:

```bash
(docker-3) ~/node/substrate-cli-tools/typescript/dist/ssvm.js selector 'balanceOf(address)'
0x70a08231
(docker-3) ~/node/substrate-cli-tools/typescript/dist/ssvm.js selector 'transfer(address,uint256)'
0xa9059cbb
```

Get ERC20 balance of Alice:

```bash
(docker-3) ALICE=9621dde636de098b43efb0fa9b61facfe328f99d
(docker-3) BOB=41dccbd49b26c50d34355ed86ff0fa9e489d1e01
(docker-3) balanceOf=70a08231
(docker-3) transfer=a9059cbb
(docker-3) ~/node/substrate-cli-tools/typescript/dist/ssvm.js call -p 99 -g 5000000 \
   -a 0xe2a313e210a6ec1d5a9c0806545670f2e6264f86 \
   -d 0x${balanceOf}000000000000000000000000${ALICE}

(docker-2)
[ssvm] Call (phase={"ApplyExtrinsic":1})
        H160: 0xe2a313e210a6ec1d5a9c0806545670f2e6264f86
[ssvm] Output (phase={"ApplyExtrinsic":1})
        Bytes: 0x00000000000000000000000000000000000000000000000000000000000003e8
(2 matching events received)
```

The output shows Alice have 0x3e8 = 1000 ERC20 tokens.

Now we could transfer 3 tokens from Alice to Bob:

```bash
(docker-3) ~/node/substrate-cli-tools/typescript/dist/ssvm.js call -p 99 -g 5000000 \
   -a 0xe2a313e210a6ec1d5a9c0806545670f2e6264f86 \
   -d 0x${transfer}000000000000000000000000${BOB}0000000000000000000000000000000000000000000000000000000000000003

(docker-2)
[ssvm] Call (phase={"ApplyExtrinsic":1})
        H160: 0xe2a313e210a6ec1d5a9c0806545670f2e6264f86
[ssvm] Output (phase={"ApplyExtrinsic":1})
        Bytes: 0x0000000000000000000000000000000000000000000000000000000000000001
(2 matching events received)
```

Check ERC20 balance of Alice again:

```bash
(docker-3) ~/node/substrate-cli-tools/typescript/dist/ssvm.js call -p 99 -g 5000000 \
   -a 0xe2a313e210a6ec1d5a9c0806545670f2e6264f86 \
   -d 0x${balanceOf}000000000000000000000000${ALICE}

(docker-2)
[ssvm] Call (phase={"ApplyExtrinsic":1})
        H160: 0xe2a313e210a6ec1d5a9c0806545670f2e6264f86
[ssvm] Output (phase={"ApplyExtrinsic":1})
        Bytes: 0x00000000000000000000000000000000000000000000000000000000000003e5
(2 matching events received)
```

Alice have 0x3e5 = 997 ERC20 tokens.

Check ERC20 balance of Bob:

```bash
(docker-3) ~/node/substrate-cli-tools/typescript/dist/ssvm.js call -p 99 -g 5000000 \
   -a 0xe2a313e210a6ec1d5a9c0806545670f2e6264f86 \
   -d 0x70a08231000000000000000000000000${BOB}

(docker-2)
[ssvm] Call (phase={"ApplyExtrinsic":1})
        H160: 0xe2a313e210a6ec1d5a9c0806545670f2e6264f86
[ssvm] Output (phase={"ApplyExtrinsic":1})
        Bytes: 0x0000000000000000000000000000000000000000000000000000000000000003
(2 matching events received)
```

Bob have 3 ERC20 tokens.

