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

use alloy_primitives::U256;
use alloy_sol_types::SolValue;
use guests::SMART_METER_ELF;
//use guests::IS_EVEN_ELF;
use risc0_zkvm::{default_executor, default_prover, ExecutorEnv, Receipt};
use risc0_ethereum_contracts::encode_seal;

use hex;
use sha2::{digest::generic_array::GenericArray, Digest, Sha256};


#[test]
fn proves_available_electricity_amount_from_smart_meter() {
    let input_amount_of_energy_to_be_sold: u64 = 800; // @dev - Input value to be loaded into the ZK circuit.
    let input_total_exact_amount_of_energy_available: u64 = 1100;
    let input_current_time: u64 = 1740641628;  // @dev - UTC timestamp (2025-02-27 / 07:33:45)
    let input_monitored_time: u64 = 1740641630;
    let input_monitored_merkle_root: String = "0xcc086fcc038189b4641db2cc4f1de3bb132aefbd65d510d817591550937818c7".to_string();
    //let input_monitored_hash_path: Vec<String> = vec!["0x8da9e1c820f9dbd1589fd6585872bc1063588625729e7ab0797cfc63a00bd950".to_string(),"0x995788ffc103b987ad50f5e5707fd094419eb12d9552cc423bd0cd86a3861433".to_string()];

    // Calculate the monitored_nullifier from the input data and store it into the variable.
    let mut hasher = Sha256::new();
    hasher.update(input_amount_of_energy_to_be_sold.to_string().as_bytes());
    hasher.update(input_monitored_time.to_string().as_bytes());
    hasher.update(input_monitored_merkle_root.as_bytes());
    let hash = hasher.finalize(); // Note that calling `finalize()` consumes hasher
    let input_monitored_nullifier: String = hex::encode(hash); // Convert GenericArray<u8, N> to a hexadecimal string

    // Execute the guest program with the input data.
    let env = ExecutorEnv::builder()
        .write(&input_amount_of_energy_to_be_sold)
        .unwrap()
        .write(&input_total_exact_amount_of_energy_available)
        .unwrap()
        .write(&input_current_time)
        .unwrap()
        .write(&input_monitored_time)
        .unwrap()
        .write(&input_monitored_merkle_root)
        .unwrap()
        //.write(&input_monitored_hash_path)
        //.unwrap()
        .write(&input_monitored_nullifier)
        .unwrap()
        .build()
        .unwrap();

    // NOTE: Use the prover to run tests with actual proving + Produce a receipt by proving the specified ELF binary.
    let prover = default_prover();
    let _receipt = prover.prove(env, SMART_METER_ELF).unwrap().receipt;
    //let _receipt = prover.prove(env, IS_EVEN_ELF).unwrap().receipt;
    println!("I know the factors of {:?}, and I can prove it!\n", _receipt);

    // Encode the seal (= Proof) with the selector.
    let seal = encode_seal(&_receipt);
    println!("seal: {:?}\n", _receipt);

    // Extract the journal from the receipt.
    let journal = _receipt.journal.bytes.clone();
    println!("journal: {:?}\n", journal);
}

#[test]
#[should_panic(expected = "total exact amount of energy available must be greater than the amount of energy to be sold")] // @dev - This expected-error message should correspond to the panice message in the constraint in the ZK circuit. 
//#[should_panic(expected = "number must be more than 0")]
fn rejects_wrong_available_electricity_amount_from_smart_meter() {
    let input_amount_of_energy_to_be_sold: u64 = 1304; // @dev - Input value to be loaded into the ZK circuit.
    let wrong_input_total_exact_amount_of_energy_available: u64 = 300;
    let input_current_time: u64 = 1740641628;  // @dev - UTC timestamp (2025-02-27 / 07:33:45)
    let input_monitored_time: u64 = 1740641630;
    let input_monitored_merkle_root: String = "0xcc086fcc038189b4641db2cc4f1de3bb132aefbd65d510d817591550937818c7".to_string();
    //let input_monitored_hash_path: Vec<String> = vec!["0x8da9e1c820f9dbd1589fd6585872bc1063588625729e7ab0797cfc63a00bd950".to_string(),"0x995788ffc103b987ad50f5e5707fd094419eb12d9552cc423bd0cd86a3861433".to_string()];

    // Calculate the monitored_nullifier from the input data and store it into the variable.
    let mut hasher = Sha256::new();
    hasher.update(input_amount_of_energy_to_be_sold.to_string().as_bytes());
    hasher.update(input_monitored_time.to_string().as_bytes());
    hasher.update(input_monitored_merkle_root.as_bytes());
    let hash = hasher.finalize(); // Note that calling `finalize()` consumes hasher
    let input_monitored_nullifier: String = hex::encode(hash); // Convert GenericArray<u8, N> to a hexadecimal string

    // Execute the guest program with the input data.
    let env = ExecutorEnv::builder()
        .write(&input_amount_of_energy_to_be_sold)
        .unwrap()
        .write(&wrong_input_total_exact_amount_of_energy_available) // @dev - This is the fake input value.
        .unwrap()
        .write(&input_current_time)
        .unwrap()
        .write(&input_monitored_time)
        .unwrap()
        .write(&input_monitored_merkle_root)
        .unwrap()
        //.write(&input_monitored_hash_path)
        //.unwrap()
        .write(&input_monitored_nullifier)
        .unwrap()
        .build()
        .unwrap();

    // NOTE: Use the prover to run tests with actual proving + Produce a receipt by proving the specified ELF binary.
    let prover = default_prover();
    let _receipt = prover.prove(env, SMART_METER_ELF).unwrap().receipt;
}
