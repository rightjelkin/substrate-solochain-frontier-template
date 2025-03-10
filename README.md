# Simple Solochain Substrate Template

This workspace is a simple solochain substrate template based on the [Polkadot SDK Solochain Template](https://github.com/paritytech/polkadot-sdk-solochain-template) with full integration of [Frontier](https://github.com/polkadot-evm/frontier).

## Features

- **Consensus Mechanisms**: Utilizes Aura for block authoring and GRANDPA for finality.
- **Validator Set Management**: For changing the validators list on-chain, this workspace uses the code of [Substrate Validator Set](https://github.com/web3gautam/substrate-validator-set) project but with bumped Polkadot dependencies to stable2412 to be compatible with frontier. Look at its documentation for more details.
- **Ethereum Compatibility**: Fully compatible with Ethereum-like APIs, allowing you to use tools like MetaMask to interact with the chain.

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

@TODO

### Connect with Polkadot-JS Apps Front-End

After you start the node template locally, you can interact with it using the hosted version of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) front-end by connecting to the local node endpoint. A hosted version is also available on [IPFS](https://dotapps.io/). You can also find the source code and instructions for hosting your own instance in the [`polkadot-js/apps`](https://github.com/polkadot-js/apps) repository. Do not forget to apply [`types.json`](./types.json) to metadata in [`polkadot-js/apps`](https://dotapps.io/)

### Connect with MetaMask

After you start the node template locally, you can add the network to MetaMask by using the following configuration:

- **Network Name**: `Solochain Local Testnet` or any other name you want to use
- **New RPC URL**: `http://localhost:9933` or other port if you changed it
- **Chain ID**: `32769` (32769 is the default chain id for local testnet, but you are able to change it in `local_config_genesis` funtion of [`genesis_config_presets.rs`](./runtime/src/genesis_config_presets.rs) file)
- **Currency Symbol**: `TEST` or any other symbol you want to use
- **Block Explorer URL**: leave blank
