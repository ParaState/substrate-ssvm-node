# Substrate SSVM Node


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
