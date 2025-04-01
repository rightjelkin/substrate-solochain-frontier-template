# Simple Solochain Substrate Template

This workspace is a simple solochain substrate template based on the [Polkadot SDK Solochain Template](https://github.com/paritytech/polkadot-sdk-solochain-template) with full integration of [Frontier](https://github.com/polkadot-evm/frontier).

## Features

- **Consensus Mechanisms**: Utilizes Aura for block authoring and GRANDPA for finality.
- **Validator Set Management**: For changing the validators list on-chain, this workspace uses the code of [Substrate Validator Set](https://github.com/web3gautam/substrate-validator-set) project but with bumped Polkadot dependencies to stable2412 to be compatible with frontier. Look at its documentation for more details.
- **Ethereum Compatibility**: Fully compatible with Ethereum-like APIs, allowing you to use tools like MetaMask to interact with the chain.

> ⚠️ Note: The benchmarking functionality has not been thoroughly tested yet. Please use caution when working with benchmarks as they may not function as expected.

## Getting Started

Fetch solochain template code:

```sh
git clone https://github.com/rightjelkin/substrate-solochain-frontier-template.git solochain-template
cd solochain-template
```

### Build

🔨 Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Launching Testnet

Both examples below assume that chain will be stored in ./chain folder

#### Alice and Bob

Fastest method of launching the testnet

```bash
./target/release/solochain-template-node --chain=local --base-path ./chain/alice --alice --rpc-cors=all --rpc-methods=Unsafe --rpc-external --port 30334 --node-key 0x0185d84f4641be58c8a30e3014006d85ef50ce8ec877e366f6a3d2e78873559b
```

```bash
./target/release/solochain-template-node --chain=local --base-path ./chain/bob --bob --rpc-cors=all --rpc-methods=Unsafe --rpc-external --port 30335 --bootnodes /ip4/127.0.0.1/tcp/30334/p2p/12D3KooWDLpRT9KFo6pKdkmdQQt599tmqVYpoeiHemY32Gf4RUpp --node-key 0xefb8bf6cb99c6093561803d7c04854a192a725ba4786a303aaf404c65f51f42a
```

Change `12D3KooWDLpRT9KFo6pKdkmdQQt599tmqVYpoeiHemY32Gf4RUpp` in second command to Alice node identity.
You can find it during launching of Alice node (see logs)

#### Alith and Baltathar

You can launch the testnet with Alith and Baltathar (frontier accounts) as validators.

1. Comment Bob and Alice lines in [genesis_config_presets.rs](./runtime/src/genesis_config_presets.rs) (204 and 205)
2. Uncomment lines 211 and 212 in the same file
3. Add Alith and Baltathar keys into keystore

    ```bash
    ./target/release/solochain-template-node key insert --base-path ./chain/alith --key-type aura --scheme Sr25519 --suri "0x3dff7d5395fbc09a035a935a5089d1eb7f94be8ef12eb35c55ab731e0c83b296"
    ./target/release/solochain-template-node key insert --base-path ./chain/alith --key-type gran --scheme Ed25519 --suri "0x3dff7d5395fbc09a035a935a5089d1eb7f94be8ef12eb35c55ab731e0c83b296"
    ```

    ```bash
    ./target/release/solochain-template-node key insert --base-path ./chain/baltathar --key-type aura --scheme Sr25519 --suri "0x4f9379c4bb9793f97796757ba0c2384f0963da4741315c9f352a4bc31cd16ac4"
    ./target/release/solochain-template-node key insert --base-path ./chain/baltathar --key-type gran --scheme Ed25519 --suri "0x4f9379c4bb9793f97796757ba0c2384f0963da4741315c9f352a4bc31cd16ac4"
    ```

4. Launch nodes with that commands (notice --base-path and do not use --alice or --bob)

    ```bash
    ./target/release/solochain-template-node --chain=local --base-path ./chain/alith --rpc-cors=all --rpc-methods=Unsafe --rpc-external --port 30334 --node-key 0x0185d84f4641be58c8a30e3014006d85ef50ce8ec877e366f6a3d2e78873559b --validator
    ```

    ```bash
    ./target/release/solochain-template-node --chain=local --base-path ./chain/baltathar --rpc-cors=all --rpc-methods=Unsafe --rpc-external --port 30335 --bootnodes /ip4/127.0.0.1/tcp/30334/p2p/12D3KooWDLpRT9KFo6pKdkmdQQt599tmqVYpoeiHemY32Gf4RUpp --node-key 0xefb8bf6cb99c6093561803d7c04854a192a725ba4786a303aaf404c65f51f42a --validator
    ```

This launch is actually an example of how you can launch validators with keys derived from your ethereum-like (ecdsa) keys.

### Connect with Polkadot-JS Apps Front-End

After you start the node template locally, you can interact with it using the hosted version of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) front-end by connecting to the local node endpoint. A hosted version is also available on [IPFS](https://dotapps.io/). You can also find the source code and instructions for hosting your own instance in the [`polkadot-js/apps`](https://github.com/polkadot-js/apps) repository. Do not forget to apply [`types.json`](./types.json) to metadata in [`polkadot-js/apps`](https://dotapps.io/)

### Connect with MetaMask

After you start the node template locally, you can add the network to MetaMask by using the following configuration:

- **Network Name**: `Solochain Local Testnet` or any other name you want to use
- **New RPC URL**: `http://localhost:9933` or other port if you changed it
- **Chain ID**: `32769` (32769 is the default chain id for local testnet, but you are able to change it in `local_config_genesis` funtion of [`genesis_config_presets.rs`](./runtime/src/genesis_config_presets.rs) file)
- **Currency Symbol**: `TEST` or any other symbol you want to use
- **Block Explorer URL**: leave blank
