# Substrate SSVM Node

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
  -v $(pwd):/root \
  -w /root \
  secondstate/ssvm:latest
```

In docker container, install rust yarn:

- Install rust
```bash
(docker) curl https://sh.rustup.rs -sSf | sh -s -- -y
(docker) source ~/.cargo/env
(docker) rustup update nightly && rustup update stable
(docker) rustup target add wasm32-unknown-unknown --toolchain nightly
```

- Install yarn
```bash
(docker) curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
(docker) echo "deb https://dl.yarnpkg.com/debian/ stable main" > /etc/apt/sources.list.d/yarn.list
(docker) apt update && apt install -y yarn
```

Build substrate-cli-tools:

```bash
(docker) cd ~/substrate-cli-tools/typescript
(docker) yarn install && yarn run tsc
(docker) chmod +x dist/*.js
```

Build substrate-ssvm-node:

```bash
(docker) cd ~/substrate-ssvm-node
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
(docker-2) ~/substrate-cli-tools/typescript/dist/info.js
(docker-2) ~/substrate-cli-tools/typescript/dist/events.js +ssvm
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
(docker-3) ~/substrate-cli-tools/typescript/dist/ssvm.js create -p 99 -g 5000000 -c 0x$(cat erc20.hex)
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
(docker-3) ~/substrate-cli-tools/typescript/dist/balances.js --ssvm //Alice
//Alice's SSVM address is 0x9621dde636de098b43efb0fa9b61facfe328f99d
//Alice's balance is 1000000

(docker-3) ~/substrate-cli-tools/typescript/dist/balances.js --ssvm //Bob
//Bob's SSVM address is 0x41dccbd49b26c50d34355ed86ff0fa9e489d1e01
//Bob's balance is 0
```

We'd like to call `balanceOf(address)` and `transfer(address,uint256)`. Use `ssvm.js selector` to get Ewasm function signature:

```bash
(docker-3) ~/substrate-cli-tools/typescript/dist/ssvm.js selector 'balanceOf(address)'
0x70a08231
(docker-3) ~/substrate-cli-tools/typescript/dist/ssvm.js selector 'transfer(address,uint256)'
0xa9059cbb
```

Get ERC20 balance of Alice:

```bash
(docker-3) ALICE=9621dde636de098b43efb0fa9b61facfe328f99d
(docker-3) BOB=41dccbd49b26c50d34355ed86ff0fa9e489d1e01
(docker-3) balanceOf=70a08231
(docker-3) transfer=a9059cbb
(docker-3) ~/substrate/substrate-cli-tools/typescript/dist/ssvm.js call -p 99 -g 5000000 \
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
(docker-3) ~/substrate/substrate-cli-tools/typescript/dist/ssvm.js call -p 99 -g 5000000 \
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
(docker-3) ~/substrate/substrate-cli-tools/typescript/dist/ssvm.js call -p 99 -g 5000000 \
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
(docker-3) ~/substrate/substrate-cli-tools/typescript/dist/ssvm.js call -p 99 -g 5000000 \
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

## Test

Apart from our intergeration demo before, we also provide an isolated ewasm test flow to test our SSVM with evmc.

To get started with our test flow, you will need to prepare three components at first.

1. [Testeth](https://github.com/ethereum/aleth.git) with version 1.6.0
2. [Official ewasm test suite](https://github.com/ewasm/tests)
3. [SSVM](https://github.com/second-state/SSVM) with version adopted by [rust-ssvm](https://github.com/second-state/rust-ssvm)

- Fetch resource
```bash
> git clone --branch v1.6.0 --recursive https://github.com/ethereum/aleth.git
> git clone https://github.com/ewasm/tests.git
> git clone --branch 0.5.0 https://github.com/second-state/SSVM.git
```

- Copy EWASM test suite into Aleth test folder
```bash
> cp tests/GeneralStateTests/stEWASMTests/* aleth/test/jsontests/GeneralStateTests/stEWASMTests/.
> cp tests/src/GeneralStateTestsFiller/stEWASMTests/* aleth/test/jsontests/src/GeneralStateTestsFiller/stEWASMTests/.
```

- Into Docker container (reuse secondstate/ssvm docker-image)
```bash
> docker run -it --rm \
      -v $(pwd):/root/workspace \
      -w /root/workspace \
      secondstate/ssvm
```

- Build testeth inside Aleth
```bash
(docker) cd aleth
(docker) mkdir build; cd build  # Create a build directory.
(docker) cmake ..               # Configure the project.
(docker) cmake --build .        # Build all default targets.

```

- Build SSVM
```bash
(docker) cd ../../SSVM
(docker) mkdir -p build && cd build
(docker) cmake -DCMAKE_BUILD_TYPE=Release -DBUILD_TESTS=ON .. && make

```

- Move vm library to test folder
```bash
(docker) cp tools/ssvm-evmc/libssvmEVMC.so ../../aleth/build/test/
```

- Execute test
```bash
(docker) cd ../../aleth/build/test/
(docker) ./testeth -t GeneralStateTests/stEWASMTests -- --vm ./libssvmEVMC.so --singlenet "Byzantium"
```

- Result
```bash
Running tests using path: "../../test/jsontests"
Running 1 test case...
Test Case "stEWASMTests":
2020-05-27 11:09:10,742 ERROR [default] Execution failed. Code: 23
2020-05-27 11:09:10,753 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:10,756 ERROR [default] Execution failed. Code: 23
/root/workspace/aleth/test/tools/libtesteth/ImportTest.cpp(768): error: in "GeneralStateTests/stEWASMTests": malformedBytecodeInvalidPreamble on Byzantium: Expected another postState hash! expected: 0x57693cf5e000607a5bf3ea9d721358cd9b5cd7f9cd3cbb7c40caf616ed7f5730 actual: 0xc47ac12abb7b3ac7f9333a6a542d5bcdd6958461846bf3f2b98275a88b969cd6 in Byzantium data: 0 gas: 0 val: 0
2020-05-27 11:09:10,771 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:10,785 ERROR [default] Execution failed. Code: 25
2020-05-27 11:09:10,793 ERROR [default] Execution failed. Code: 13
2020-05-27 11:09:10,799 ERROR [default] Execution failed. Code: 31
2020-05-27 11:09:10,805 ERROR [default] Execution failed. Code: 13
2020-05-27 11:09:10,812 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:10,816 ERROR [default] Execution failed. Code: 13
2020-05-27 11:09:10,822 ERROR [default] Execution failed. Code: 23
24%...
2020-05-27 11:09:10,833 ERROR [default] Execution failed. Code: 25
2020-05-27 11:09:10,839 ERROR [default] Execution failed. Code: 31
2020-05-27 11:09:10,841 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:10,847 ERROR [default] Execution failed. Code: 23
2020-05-27 11:09:10,876 ERROR [default] Execution failed. Code: 23
2020-05-27 11:09:10,879 ERROR [default] Execution failed. Code: 23
2020-05-27 11:09:10,882 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:10,889 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:10,897 ERROR [default] Execution failed. Code: 19
2020-05-27 11:09:10,908 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:10,915 ERROR [default] Execution failed. Code: 23
49%...
2020-05-27 11:09:10,918 ERROR [default] Execution failed. Code: 31
2020-05-27 11:09:10,920 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:10,944 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:10,948 ERROR [default] Reverted.
2020-05-27 11:09:10,965 ERROR [default] Execution failed. Code: 25
2020-05-27 11:09:10,971 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:10,976 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:10,977 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:10,982 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:10,993 ERROR [default] Execution failed. Code: 13
74%...
2020-05-27 11:09:11,009 ERROR [default] Execution failed. Code: 13
2020-05-27 11:09:11,011 ERROR [default] Execution failed. Code: 13
2020-05-27 11:09:11,014 ERROR [default] Execution failed. Code: 23
2020-05-27 11:09:11,069 ERROR [default] Execution failed. Code: 31
2020-05-27 11:09:11,086 ERROR [default] Reverted.
2020-05-27 11:09:11,095 ERROR [default] Execution failed. Code: 28
2020-05-27 11:09:11,171 ERROR [default] Execution failed. Code: 28
99%...
2020-05-27 11:09:11,177 ERROR [default] Execution failed. Code: 31
100%

*** 1 failure is detected (5 failures are expected) in the test module "Master Test Suite"
```

> **Known issue**
The only one mismatch test case `malformedBytecodeInvalidPreamble` is already fixed in SSVM@d3f4ebd and we will release next rust-ssvm cover the solution.
