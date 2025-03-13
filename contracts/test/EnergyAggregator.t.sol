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

pragma solidity ^0.8.20;

import { console2 } from "forge-std/console2.sol";
import { Test } from "forge-std/Test.sol";
import { RiscZeroCheats } from "risc0/test/RiscZeroCheats.sol";
import { Receipt as RiscZeroReceipt } from "risc0/IRiscZeroVerifier.sol";
//import { IRiscZeroVerifier } from "risc0/IRiscZeroVerifier.sol";
import { RiscZeroMockVerifier } from "risc0/test/RiscZeroMockVerifier.sol"; /// @dev - 'Prover' contract for testing.
import { VerificationFailed } from "risc0/IRiscZeroVerifier.sol";
import { EnergyAggregator } from "../src/EnergyAggregator.sol";
import { ImageID } from "../src/ImageID.sol";

contract EnergyAggregatorTest is RiscZeroCheats, Test {
    EnergyAggregator public energyAggregator;
    //IRiscZeroVerifier public verifier;
    RiscZeroMockVerifier public verifier;

    function setUp() public {
        //address RISCZERO_VERIFIER = vm.envAddress("VERIFIER_ROUTER_ADDRESS"); /// @dev - Deployed-address of the 'RiscZeroVerifierRouter.sol' contract on Ethereum Sepolia.  
        //verifier = IRiscZeroVerifier(RISCZERO_VERIFIER);
        verifier = new RiscZeroMockVerifier(0); /// @dev - "Mock" Verifier (which is used for "proving" in this test)
        energyAggregator = new EnergyAggregator(verifier);
        assertEq(energyAggregator.getEnergyAmountToBeSold(), 0);
    }

    function test_submitEnergyAmountToBeSold() public {
        uint256 energyAmountToBeSold = 100;  /// @dev - 100 kWh
        uint256 monitoredTime = 1740641630;
        bytes32 monitoredMerkleRoot = 0xcc086fcc038189b4641db2cc4f1de3bb132aefbd65d510d817591550937818c7;
        //uint256 monitored_hash_path,
        bytes32 monitoredNullifier = 0x1efa9d6bb4dfdf86063cc77efdec90eb9262079230f1898049efad264835b6c8;

        RiscZeroReceipt memory receipt = verifier.mockProve(ImageID.IS_SMART_METER_ID, sha256(abi.encode(energyAmountToBeSold, monitoredTime, monitoredMerkleRoot, monitoredNullifier)));

        energyAggregator.submitEnergyAmountToBeSold(energyAmountToBeSold, monitoredTime, monitoredMerkleRoot, monitoredNullifier, receipt.seal);
        assertEq(energyAggregator.getEnergyAmountToBeSold(), energyAmountToBeSold);
    }

    // function test_submitEnergyAmountToBeSold_with_Zero() public {
    //     uint256 energyAmountToBeSold = 0;  /// @dev - 100 kWh
    //     uint256 monitoredTime = 1740641630;
    //     bytes32 monitoredMerkleRoot = 0xcc086fcc038189b4641db2cc4f1de3bb132aefbd65d510d817591550937818c7;
    //     //uint256 monitored_hash_path,
    //     bool monitoredNullifier = true;

    //     RiscZeroReceipt memory receipt = verifier.mockProve(ImageID.IS_SMART_METER_ID, sha256(abi.encode(energyAmountToBeSold, monitoredTime, monitoredMerkleRoot, monitoredNullifier)));

    //     energyAggregator.submitEnergyAmountToBeSold(energyAmountToBeSold, monitoredTime, monitoredMerkleRoot, monitoredNullifier, receipt.seal);
    //     assertEq(energyAggregator.getEnergyAmountToBeSold(), energyAmountToBeSold);
    // }

    // Try using a proof for the evenness of 4 to set 1 on the contract.
    function test_rejectInvalidProof() public {
        uint256 energyAmountToBeSold = 0;  /// @dev - 0 kWh
        uint256 monitoredTime = 1740641630;
        bytes32 monitoredMerkleRoot = 0xcc086fcc038189b4641db2cc4f1de3bb132aefbd65d510d817591550937818c7;
        //uint256 monitored_hash_path,
        bytes32 monitoredNullifier = 0x1efa9d6bb4dfdf86063cc77efdec90eb9262079230f1898049efad264835b6c8;

        /// @dev - Generate (= Prove) a new mock proof.
        RiscZeroReceipt memory receipt = verifier.mockProve(ImageID.IS_SMART_METER_ID, sha256(abi.encode(energyAmountToBeSold, monitoredTime, monitoredMerkleRoot, monitoredNullifier)));
        //console2.log("Receipt ID:", receipt.id);
        console2.logBytes(receipt.seal); // [Log]: 0x000000002cfcebe8cc0eeb0dbd0d347d08fb5ee468cd9747c1920d0cb81222b1e8576962

        /// @dev - Verify the mock proof-generated via the function below.
        vm.expectRevert("Energy amount to be sold must be greater than 0"); /// @dev - This expected revert message must correspond to an error message in the SC level validation in the submitEnergyAmountToBeSold().
        //vm.expectRevert(VerificationFailed.selector);
        energyAggregator.submitEnergyAmountToBeSold(energyAmountToBeSold, monitoredTime, monitoredMerkleRoot, monitoredNullifier, receipt.seal);
    }
}
