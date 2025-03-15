# ZK Energy Trading Platform ⚡️

## Tech stack

- `ZK circuit`: Written in [`zkVM / Boundless`](https://beboundless.xyz/) powered by [RISC Zero](https://risczero.com/)
- Smart Contract: Written in Solidity (Framework: Foundry)
- Blockchain: [`Ethereum Sepolia`](https://sepolia.etherscan.io/) (Testnet)

<br>

## Actors
- Energy Producer's (Energy Seller's) smart meter
- Energy Consumer's (Energy Buyer's) smart meter
- Energy Aggregator smart contract

<br>

## Overview

- This is the ZK (Zero-Knowledge) based Energy Trading Platform in `zkVM / Boundless` powered by `RISC Zero`, which is the Zero-Knowledge based Energy Trading Platform that consists of the ZK programs (ZK circuits) and the smart contracts.

- By trading energy through this platform, each actor would get the following merits: 
  - an energy producer ('s Smart Meter) can create a sell order of a specified-amount of energy **without revealing** a **`whole` amount of energy available** in the energy producer's energy charger.

  - an energy consumer ('s Smart Meter) can buy create a buy order of a desired-amount of energy, which is validated and a `proof` is attested via a ZK circuit.

<br>

## User flow


- 1/ A energy producer's smart meter would measure a whole amount of energy avaialble.

- 2/ A energy producer's smart meter create a sell order with a specified-amount of energy.
  - 2-1/ At this point, the energy producer's smart meter would generate (prove) a proof via a ZK program (ZK circuit) `off-chain`. 
    - During the process of generating (proving) a proof in a ZK program, it would be validated whether or not the amount of energy-created with a sell order is equal to or less than the whole amount of energy-measured by the producer's smart meter. (This is called a "constraint")
    - Once the validation (constraint) would be passed in the ZK program, a proof wold be generated and a energy producer will receive it off-chain.

  - 2-2/ Then, the energy producer would call the EnergyAggregator#`createSellOrder()` with a proof and public inputs as the arguments.

- 3/ A energy consumer would deposit certain amount of native ETH into the EnergyAggregator contract via the EnergyAggregator#`depositNativeETH()`
  - By doing so, the amount of native ETH to be send would be added to the buyer's account in the EnergyAggregator contract (`buyerBalances[buyer's address]`).

- 4/ The energy consumer would create a buy order with a desired-amount of energy via the EnergyAggregator#`createBuyOrder()`.
  - 4-1/ If this buy order would be matched with a sell order, the EnergyAggregator contract will transfer the payment to the producer's smart meter.


- NOTE:
  - In this scenario, we assume that both a producer's smart meter and a consumer's smart meter would hold a wallet (i.e. EOA /or smart contract wallet)

  - In this scenario, an energy **price** would be set as a `fixed-price` (`0.00000001 ETH / kWh`). In the future, a seller and a buyer should be able to specify a dynamic price (desired-price).

  - In the step 2/ above, the remained-amount of energy goes to a home battery stroage like [Tesla's Powerwall](https://www.tesla.com/powerwall))


<br>

## Deployed-addresses on Ethereum Sepolia testnet

<br>

## Installations

### Install the cargo packages

- 1/ Install the Cargo packages
```bash
cargo build
```

- 2/ Within the `contracts/test/Elf.sol`, the **path** (`SMART_METER_PATH`) should be fixed like this:
```solidity
library Elf {
    string public constant SMART_METER_PATH =
        "../../target/riscv-guest/guests/smart-meter/riscv32im-risc0-zkvm-elf/release/smart-meter";
}
```

<br>

### Compile the smart contracts

- Compile the smart contracts
```bash
forge build
```

<br>

### Running the test of the guest program

- Run the test of the `smart-meter` ZK program (called a guest program in zkVM / Boundless powered by RISC Zero)
```bash
sh guests/tests/runningGuestProgram_smart-meter.sh
```

<br>

### Deploy the smart contracts
- Deploy the EnergyAggregator contract.
```bash
sh ./contracts/scripts/runningScript_Deploy.sh
```

<br>

### Running the Test of SCs on Ethereum Sepolia testnet
- Run the test of the EnergyAggregator contract (`./contracts/test/EnergyAggregator.t.sol`).
```bash
sh ./contracts/test/runningTest_EnergyAggregator.sh
```

<br>

### Running the (backend) App
- Run the `./apps/src/main.rs`
```bash
sh ./apps/runningApp_main.sh
```



<br>

<hr>

# Boundless Foundry Template

This template serves as a starting point for developing an application with verifiable compute provided by [Boundless][boundless-homepage].
It is built around a simple smart contract, `EvenNumber`, and its associated RISC Zero guest, `is-even`.

## Build

To build the example run:

```bash
# Populate the `./lib` submodule dependencies
git submodule update --init --recursive
cargo build
forge build
```

## Test

Test the Solidity smart contracts with:

```bash
forge test -vvv
```

Test the Rust code including the guest with:

```bash
cargo test
```

## Deploy to Testnet

### Set up your environment

Export your Sepolia testnet wallet private key as an environment variable:

```bash
export WALLET_PRIVATE_KEY="YOUR_WALLET_PRIVATE_KEY"
```

To allow provers to access your zkVM guest binary, it must be uploaded to a public URL. For this example we will upload to IPFS using Pinata. Pinata has a free tier with plenty of quota to get started. Sign up at [[Pinata](https://pinata.cloud/)](https://pinata.cloud/), generate an API key, and set the JWT as an environment variable:

```bash
export PINATA_JWT="YOUR_PINATA_JWT"
```

A [`.env`](./.env) file is provided with the Boundless contract deployment information for Sepolia.
The example app reads from this `.env` file automatically.

### Deploy the contract

To deploy the `EvenNumber` contract run:

```bash
. ./.env # load the environment variables from the .env file for deployment
forge script contracts/scripts/Deploy.s.sol --rpc-url ${RPC_URL:?} --broadcast -vv
```

Save the `EvenNumber` contract address to an env variable:

<!-- TODO: Update me -->

```bash
# First contract deployed and top of logs is EvenNumber
export EVEN_NUMBER_ADDRESS=#COPY EVEN NUMBER ADDRESS FROM DEPLOY LOGS
```

> You can also use the following command to set the contract address if you have [`jq`][jq] installed:
>
> ```bash
> export EVEN_NUMBER_ADDRESS=$(jq -re '.transactions[] | select(.contractName == "EvenNumber") | .contractAddress' ./broadcast/Deploy.s.sol/11155111/run-latest.json)
> ```

### Run the example

The [example app](apps/src/main.rs) will upload your zkVM guest to IPFS, submit a request to the market for a proof that "4" is an even number, wait for the request to be fulfilled, and then submit that proof to the EvenNumber contract, setting the value to "4".


To run the example:

```bash
RUST_LOG=info cargo run --bin app -- --even-number-address ${EVEN_NUMBER_ADDRESS:?} --number 4
```

[jq]: https://jqlang.github.io/jq/
[boundless-homepage]: https://beboundless.xyz
[sepolia]: https://ethereum.org/en/developers/docs/networks/#sepolia
