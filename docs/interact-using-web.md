# Interact using Substrate web UI

## Build & Run node

We use [secondstate/substrate-ssvm](https://hub.docker.com/r/secondstate/substrate-ssvm) as building & running environment:

```bash
> git clone https://github.com/second-state/substrate-ssvm-node.git
> docker run -it --rm \
  --name ssvm \
  -p 9944:9944 \
  -v $(pwd):/root/node \
  -w /root/node/substrate-ssvm-node \
  secondstate/substrate-ssvm:latest \
  cargo run --release --bin node-template -- --dev --ws-external
```

> **Remember to publish 9944 port from container to use Substrate web UI.**

## Connect to node

- Open https://polkadot.js.org/apps in browser.
- In [settings](https://polkadot.js.org/apps/#/settings), make sure you select `Local Node`.

![](./web-ui/local-node.png)

## Create Contract

Here we use the following ERC20 contract files as an example:
- [erc20.sol](./erc20/erc20.sol)
  - This file is an ERC20 contract written in Solidity.
- [erc20.wasm](./erc20/erc20.wasm)
  - This file is a wasm file generate from `erc20.sol` by [SOLL](https://github.com/second-state/soll)
  - Command to generate wasm file: `soll -deploy=Normal erc20.sol`
- [erc20.hex](./erc20/erc20.hex)
  - To deploy wasm file to our node, we need to convert `erc20.wasm` to hex.
  - Command to generate hex file: `xxd -p erc20.wasm | tr -d $'\n' > erc20.hex`

- In [Extrinsics](https://polkadot.js.org/apps/#/extrinsics), select `ssvm` and `create`.
- Put the content of [erc20.hex](./erc20/erc20.hex) in `code` section.
- Select proper gas limit & gas price.
- Submit Transaction

![](./web-ui/send-ssvm-tx.png)

The contract will be deployed at address `0xe2a313e210a6ec1d5a9c0806545670f2e6264f86`.

You could check logs using `events.js +ssvm` from `substrate-cli-tools`. See more information about `substrate-cli-tools` in [Interact using command line tools](./interact-using-cli.md).

```bash
[ssvm] Create (phase={"ApplyExtrinsic":1})
        H160: 0xe2a313e210a6ec1d5a9c0806545670f2e6264f86
(1 matching events received)
```

## Interact with contract

You could check Ewasm addresses of Alice and Bob using `balances.js --svm`from `substrate-cli-tools`. See more information about `substrate-cli-tools` in [Interact using command line tools](./interact-using-cli.md).

```bash
(docker) ~/node/substrate-cli-tools/typescript/dist/balances.js --ssvm //Alice
//Alice's SSVM address is 0x9621dde636de098b43efb0fa9b61facfe328f99d
//Alice's balance is 1000000

(docker) ~/node/substrate-cli-tools/typescript/dist/balances.js --ssvm //Bob
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

### Get ERC20 balance of Alice:

- Prepare call data:
  - In EVM, call data is in `{function signature}{function arguments}` format
  - Data of `balanceOf(Alice)` is `0x70a082310000000000000000000000009621dde636de098b43efb0fa9b61facfe328f99d`
- In [Extrinsics](https://polkadot.js.org/apps/#/extrinsics), select `ssvm` and `call`.
- Fill contract address `0xe2a313e210a6ec1d5a9c0806545670f2e6264f86` in target section.
- Fill call data in input section.
- Select proper gas limit & gas price.
- Submit Transaction

![](./web-ui/balance-of-alice.png)

The output `0x00000000000000000000000000000000000000000000000000000000000003e8` shows that Alice has 0x3e8 = 1000 ERC20 tokens.

```
[ssvm] Call (phase={"ApplyExtrinsic":1})
        H160: 0xe2a313e210a6ec1d5a9c0806545670f2e6264f86
[ssvm] Output (phase={"ApplyExtrinsic":1})
        Bytes: 0x00000000000000000000000000000000000000000000000000000000000003e8
(2 matching events received)
```

### Transfer 3 tokens from Alice to Bob:

- Prepare call data:
  - Data of `transfer(Bob, 3)` is `0xa9059cbb00000000000000000000000041dccbd49b26c50d34355ed86ff0fa9e489d1e010000000000000000000000000000000000000000000000000000000000000003`
- In [Extrinsics](https://polkadot.js.org/apps/#/extrinsics), select `ssvm` and `call`.
- Fill contract address `0xe2a313e210a6ec1d5a9c0806545670f2e6264f86` in target section.
- Fill call data in input section.
- Select proper gas limit & gas price.
- Submit Transaction

The output `0x0000000000000000000000000000000000000000000000000000000000000001` shows that function call is success.

```
[ssvm] Call (phase={"ApplyExtrinsic":1})
        H160: 0xe2a313e210a6ec1d5a9c0806545670f2e6264f86
[ssvm] Output (phase={"ApplyExtrinsic":1})
        Bytes: 0x0000000000000000000000000000000000000000000000000000000000000001
(2 matching events received)
```

## Check ERC20 balance of Alice again

- Prepare call data:
  - Data of `balanceOf(Alice)` is `0x70a082310000000000000000000000009621dde636de098b43efb0fa9b61facfe328f99d`
- In [Extrinsics](https://polkadot.js.org/apps/#/extrinsics), select `ssvm` and `call`.
- Fill contract address `0xe2a313e210a6ec1d5a9c0806545670f2e6264f86` in target section.
- Fill call data in input section.
- Select proper gas limit & gas price.
- Submit Transaction

The output `0x00000000000000000000000000000000000000000000000000000000000003e5` shows that Alice has 0x3e5 = 997 ERC20 tokens after transfering.

```
[ssvm] Call (phase={"ApplyExtrinsic":1})
        H160: 0xe2a313e210a6ec1d5a9c0806545670f2e6264f86
[ssvm] Output (phase={"ApplyExtrinsic":1})
        Bytes: 0x00000000000000000000000000000000000000000000000000000000000003e5
(2 matching events received)
```
