// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{ops::Add, time::Duration};

use crate::energy_aggregator::IEnergyAggregator::IEnergyAggregatorInstance;
//use crate::even_number::IEvenNumber::IEvenNumberInstance;
use alloy::{
    primitives::{utils::parse_ether, Address, U256},
    signers::local::PrivateKeySigner,
    sol_types::SolValue,
};
use anyhow::{bail, ensure, Context, Result};
use boundless_market::{
    client::ClientBuilder,
    contracts::{Input, Offer, Predicate, ProofRequestBuilder, Requirements},
    input::InputBuilder,
    storage::StorageProviderConfig,
};
use clap::Parser;
use guests::{IS_SMART_METER_ELF, IS_SMART_METER_ID}; // "ELF" and "image ID" (ImageID.sol#IS_SMART_METER_ID)
//use guests::{IS_EVEN_ELF, IS_EVEN_ID}; // "ELF" and "image ID" (ImageID.sol#IS_EVEN_ID)
use risc0_zkvm::{default_executor, sha::Digestible};
use url::Url;

/// Timeout for the transaction to be confirmed.
pub const TX_TIMEOUT: Duration = Duration::from_secs(30);

mod energy_aggregator {
    alloy::sol!(
        #![sol(rpc, all_derives)]
        "../contracts/src/IEnergyAggregator.sol"
        //"../contracts/src/IEvenNumber.sol"
    );
}

/// Arguments of the publisher CLI.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The number to publish to the EvenNumber contract.
    #[clap(short, long)]
    amount_of_energy_to_be_sold: u64, // @dev - Used in CLI as an option / The energyAmountToBeSold to publish to the EnergyAggregator contract.
    //number: u32,           // @dev - Used in CLI as an option / The number to publish to the EvenNumber contract.

    /// URL of the Ethereum RPC endpoint.
    #[clap(short, long, env)]
    rpc_url: Url,
    /// Private key used to interact with the EvenNumber contract.
    #[clap(short, long, env)]
    wallet_private_key: PrivateKeySigner,
    /// Submit the request offchain via the provided order stream service url.
    #[clap(short, long, requires = "order_stream_url")]
    offchain: bool,
    /// Offchain order stream service URL to submit offchain requests to.
    #[clap(long, env)]
    order_stream_url: Option<Url>,
    /// Storage provider to use
    #[clap(flatten)]
    storage_config: Option<StorageProviderConfig>,

    /// Address of the EvenNumber contract.
    #[clap(short, long, env)]
    energy_aggregator_address: Address, // @dev - Used in CLI as an option / The deployed-address of the EvenNumber contract.
    //even_number_address: Address, // @dev - Used in CLI as an option / The deployed-address of the EvenNumber contract.
    
    /// Address of the RiscZeroSetVerifier contract.
    #[clap(short, long, env)]
    set_verifier_address: Address,
    /// Address of the BoundlessfMarket contract.
    #[clap(short, long, env)]
    boundless_market_address: Address,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    match dotenvy::dotenv() {
        Ok(path) => tracing::debug!("Loaded environment variables from {:?}", path),
        Err(e) if e.not_found() => tracing::debug!("No .env file found"),
        Err(e) => bail!("failed to load .env file: {}", e),
    }
    let args = Args::parse();

    // Create a Boundless client from the provided parameters.
    let boundless_client = ClientBuilder::default()
        .with_rpc_url(args.rpc_url)
        .with_boundless_market_address(args.boundless_market_address)
        .with_set_verifier_address(args.set_verifier_address)
        .with_order_stream_url(args.offchain.then_some(args.order_stream_url).flatten())
        .with_storage_provider_config(args.storage_config)
        .with_private_key(args.wallet_private_key)
        .build()
        .await?;

    // Upload the ELF to the storage provider so that it can be fetched by the market.
    ensure!(
        boundless_client.storage_provider.is_some(),
        "a storage provider is required to upload the zkVM guest ELF"
    );

    print!("\n Uploading image to storage provider..............................\n");

    let image_url = boundless_client.upload_image(IS_SMART_METER_ELF).await?; // Error: Failed to upload image
    //let image_url = boundless_client.upload_image(IS_EVEN_ELF).await?; // Error: Failed to upload image
    tracing::info!("Uploaded image to {}\n", image_url);

    // Encode the input and upload it to the storage provider.
    tracing::info!("amount_of_energy_to_be_sold to publish: {}\n", args.amount_of_energy_to_be_sold); // @dev - [NOTE]: At the moment, this is not used as the input data. Instead, the constant number ("input_number" below) is used as the input data.
    //tracing::info!("Number to publish: {}\n", args.number); // @dev - [NOTE]: At the moment, this is not used as the input data. Instead, the constant number ("input_number" below) is used as the input data.
    
    let input_amount_of_energy_to_be_sold: u64 = 800; // @dev - Input value to be loaded into the ZK circuit.
    let input_total_exact_amount_of_energy_available: u64 = 1100;
    let input_current_time: u64 = 1740641628;  // @dev - UTC timestamp (2025-02-27 / 07:33:45)
    let input_monitored_time: u64 = 1740641630;
    let input_monitored_merkle_root: String = "0xcc086fcc038189b4641db2cc4f1de3bb132aefbd65d510d817591550937818c7".to_string();
    //let input_monitored_hash_path: Vec<String> = vec!["0x8da9e1c820f9dbd1589fd6585872bc1063588625729e7ab0797cfc63a00bd950".to_string(),"0x995788ffc103b987ad50f5e5707fd094419eb12d9552cc423bd0cd86a3861433".to_string()];
    let input_monitored_nullifier: bool = true;
    tracing::info!("'input_amount_of_energy_to_be_sold' to publish: {}\n", input_amount_of_energy_to_be_sold);
    tracing::info!("'input_total_exact_amount_of_energy_available' to publish: {}\n", input_total_exact_amount_of_energy_available);
    tracing::info!("'input_current_time' to publish: {}\n", input_current_time);
    tracing::info!("'input_monitored_time' to publish: {}\n", input_monitored_time);
    tracing::info!("'input_monitored_merkle_root' to publish: {}\n", input_monitored_merkle_root);
    tracing::info!("'input_monitored_nullifier' to publish: {}\n", input_monitored_nullifier);

    //let input_builder = InputBuilder::new().write_slice(&U256::from(args.number).abi_encode());
    let input_builder = InputBuilder::new().write(&input_amount_of_energy_to_be_sold).unwrap()
                                                         .write(&input_total_exact_amount_of_energy_available).unwrap()
                                                         .write(&input_current_time).unwrap()
                                                         .write(&input_monitored_time).unwrap()
                                                         .write(&input_monitored_merkle_root).unwrap()
                                                         //.write(&input_monitored_hash_path).unwrap()
                                                         .write(&input_monitored_nullifier).unwrap();

    tracing::info!("input builder: {:?}\n", input_builder);

    let guest_env = input_builder.clone().build_env()?;
    let guest_env_bytes = guest_env.encode()?;

    // Dry run the ELF with the input to get the journal and cycle count.
    // This can be useful to estimate the cost of the proving request.
    // It can also be useful to ensure the guest can be executed correctly and we do not send into
    // the market unprovable proving requests. If you have a different mechanism to get the expected
    // journal and set a price, you can skip this step.
    let session_info = default_executor().execute(guest_env.try_into().unwrap(), IS_SMART_METER_ELF)?;
    //let session_info = default_executor().execute(guest_env.try_into().unwrap(), IS_EVEN_ELF)?;
    let mcycles_count = session_info
        .segments
        .iter()
        .map(|segment| 1 << segment.po2)
        .sum::<u64>()
        .div_ceil(1_000_000);
    let journal = session_info.journal;

    // Create a proof request with the image, input, requirements and offer.
    // The ELF (i.e. image) is specified by the image URL.
    // The input can be specified by an URL, as in this example, or can be posted on chain by using
    // the `with_inline` method with the input bytes.
    // The requirements are the image ID and the digest of the journal. In this way, the market can
    // verify that the proof is correct by checking both the committed image id and digest of the
    // journal. The offer specifies the price range and the timeout for the request.
    // Additionally, the offer can also specify:
    // - the bidding start time: the block number when the bidding starts;
    // - the ramp up period: the number of blocks before the price start increasing until reaches
    //   the maxPrice, starting from the the bidding start;
    // - the lockin price: the price at which the request can be locked in by a prover, if the
    //   request is not fulfilled before the timeout, the prover can be slashed.
    // If the input exceeds 2 kB, upload the input and provide its URL instead, as a rule of thumb.
    let request_input = if guest_env_bytes.len() > 2 << 10 {
        let input_url = boundless_client.upload_input(&guest_env_bytes).await?;
        tracing::info!("Uploaded input to {} \n", input_url);
        Input::url(input_url)
    } else {
        tracing::info!("Sending input inline with request \n");
        Input::inline(guest_env_bytes.clone())
    };

    let request = ProofRequestBuilder::new()
        .with_image_url(image_url.to_string())
        .with_input(request_input)
        .with_requirements(Requirements::new(
            IS_SMART_METER_ID,
            Predicate::digest_match(journal.digest()),
        ))
        .with_offer(
            Offer::default()
                // The market uses a reverse Dutch auction mechanism to match requests with provers.
                // Each request has a price range that a prover can bid on. One way to set the price
                // is to choose a desired (min and max) price per million cycles and multiply it
                // by the number of cycles. Alternatively, you can use the `with_min_price` and
                // `with_max_price` methods to set the price directly.
                .with_min_price_per_mcycle(parse_ether("0.001")?, mcycles_count)
                // NOTE: If your offer is not being accepted, try increasing the max price.
                .with_max_price_per_mcycle(parse_ether("0.002")?, mcycles_count)
                // The timeout is the maximum number of blocks the request can stay
                // unfulfilled in the market before it expires. If a prover locks in
                // the request and does not fulfill it before the timeout, the prover can be
                // slashed.
                .with_timeout(1000),
        )
        .build()
        .unwrap();

    // Send the request and wait for it to be completed.
    let (request_id, expires_at) = boundless_client.submit_request(&request).await?;
    tracing::info!("Request 0x{request_id:x} submitted");

    // Wait for the request to be fulfilled by the market, returning the journal and seal.
    tracing::info!("Waiting for 0x{request_id:x} to be fulfilled");
    let (_journal, seal) = boundless_client
        .wait_for_request_fulfillment(request_id, Duration::from_secs(5), expires_at)
        .await?;
    tracing::info!("Request 0x{request_id:x} fulfilled");

    // Interact with the EvenNumber contract by calling the set function with our number and
    // the seal (i.e. proof) returned by the market.
    let energy_aggregator = IEnergyAggregatorInstance::new(
        args.energy_aggregator_address,
        boundless_client.provider().clone(), // @dev - IRiscZeroVerifier contract instance
    );
    let set_number = energy_aggregator
        .submitEnergyAmountToBeSold(U256::from(args.amount_of_energy_to_be_sold), seal)  // @dev - Call the EvenNumber#set() function
        .from(boundless_client.caller());

    tracing::info!("Broadcasting tx calling EvenNumber set function");
    let pending_tx = set_number.send().await.context("failed to broadcast tx")?;
    tracing::info!("Sent tx {}", pending_tx.tx_hash());
    let tx_hash = pending_tx
        .with_timeout(Some(TX_TIMEOUT))
        .watch()
        .await
        .context("failed to confirm tx")?;
    tracing::info!("Tx {:?} confirmed", tx_hash);

    // We query the value stored at the EvenNumber address to check it was set correctly
    let amount_of_energy_to_be_sold = energy_aggregator
        .getEnergyAmountToBeSold()
        .call()
        .await
        .context("failed to get number from contract")?
        ._0;
    tracing::info!(
        "amount_of_energy_to_be_sold for address: {:?} is set to {:?}",
        boundless_client.caller(),
        amount_of_energy_to_be_sold
    );

    Ok(())
}
