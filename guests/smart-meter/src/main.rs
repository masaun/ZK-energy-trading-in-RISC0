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


use core::num;
use std::io::Read;

use alloy_primitives::{ Uint, U256 };
use alloy_sol_types::{ SolValue, Error, SolType };
use risc0_zkvm::guest::env;

use hex;
use sha2::{digest::generic_array::GenericArray, Digest, Sha256};

#[derive(Debug)]
struct ElectricityBillData {
    seller_id: u32,
    seller_physical_address: String,
    seller_wallet_addresses: String,
}

fn main() {
    // Read the input data for this application (= Host).
    let input_amount_of_energy_to_be_sold: u64 = env::read();
    let input_total_exact_amount_of_energy_available: u64 = env::read();
    let input_current_time: u64 = env::read();
    let input_monitored_time: u64 = env::read();
    let input_monitored_merkle_root: String = env::read();
    //let input_monitored_hash_path: Vec<String> = Vec::<String>::new();
    let input_monitored_nullifier: String = env::read();

    // Calculate the monitored_nullifier from the input data and store it into the variable.
    let mut hasher = Sha256::new();
    hasher.update(input_amount_of_energy_to_be_sold.to_string().as_bytes());
    hasher.update(input_monitored_time.to_string().as_bytes());
    hasher.update(input_monitored_merkle_root.as_bytes());
    let hash = hasher.finalize(); // Note that calling `finalize()` consumes hasher
    let nullifier: String = hex::encode(hash); // Convert GenericArray<u8, N> to a hexadecimal string

    // Constraint: Check the input data of the monitored_nullifier.
    assert!(input_monitored_nullifier == nullifier, "The input_monitored_nullifier is not correct.");

    // Decode and parse the input
    let amount_of_energy_to_be_sold = input_amount_of_energy_to_be_sold;
    //let number: Uint<256, 4> = <U256>::abi_decode(&input_number_bytes, true).unwrap();
    let total_exact_amount_of_energy_available = input_total_exact_amount_of_energy_available;
    //let total_exact_amount_of_energy_available = <U256>::abi_decode(&input_total_exact_amount_of_energy_available_bytes, true).unwrap();
    let current_time = input_current_time;
    let monitored_time = input_monitored_time;
    let monitored_merkle_root = input_monitored_merkle_root;
    //let monitored_hash_path = input_monitored_hash_path;
    let monitored_nullifier = input_monitored_nullifier;

    // Run the computation.
    assert!(total_exact_amount_of_energy_available >= amount_of_energy_to_be_sold, "total exact amount of energy available must be greater than the amount of energy to be sold");
    //assert!(!number.bit(0), "number is not even");
    //assert!(monitored_time <= current_time, "A given monitored time must be less than the current time");
    //assert!(&monitored_time >= &current_time - 3600, "A given monitored time must be greater than the current time - 1 hour (3600 seconds)");

    // Commit the "journal" that will be received by the application contract.
    // Journal is encoded using Solidity ABI for easy decoding in the app contract.
    env::commit(&amount_of_energy_to_be_sold);
    env::commit(&monitored_time);
    env::commit(&monitored_merkle_root);
    //env::commit(&monitored_hash_path);
    env::commit(&monitored_nullifier);
    //env::commit_slice(number.abi_encode().as_slice());
}
