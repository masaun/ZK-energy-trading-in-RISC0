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
import { RiscZeroMockVerifier } from "risc0/test/RiscZeroMockVerifier.sol";
import { VerificationFailed } from "risc0/IRiscZeroVerifier.sol";
import { EnergyAggregator } from "../src/EnergyAggregator.sol";
import { ImageID } from "../src/ImageID.sol";

contract EnergyAggregatorTest is RiscZeroCheats, Test {
    EnergyAggregator public energyAggregator;
    RiscZeroMockVerifier public verifier;

    function setUp() public {
        verifier = new RiscZeroMockVerifier(0);
        energyAggregator = new EnergyAggregator(verifier);
        assertEq(energyAggregator.getEnergyAmountToBeSold(), 0);
    }

    function test_submitEnergyAmountToBeSold() public {
        uint256 energyAmountToBeSold = 600;
        RiscZeroReceipt memory receipt = verifier.mockProve(ImageID.IS_SMART_METER_ID, sha256(abi.encode(energyAmountToBeSold)));

        energyAggregator.submitEnergyAmountToBeSold(energyAmountToBeSold, receipt.seal);
        assertEq(energyAggregator.getEnergyAmountToBeSold(), energyAmountToBeSold);
    }

    function test_submitEnergyAmountToBeSold_with_Zero() public {
        uint256 energyAmountToBeSold = 0;
        RiscZeroReceipt memory receipt = verifier.mockProve(ImageID.IS_SMART_METER_ID, sha256(abi.encode(energyAmountToBeSold)));

        energyAggregator.submitEnergyAmountToBeSold(energyAmountToBeSold, receipt.seal);
        assertEq(energyAggregator.getEnergyAmountToBeSold(), energyAmountToBeSold);
    }

    // Try using a proof for the evenness of 4 to set 1 on the contract.
    function test_rejectInvalidProof() public {
        uint256 energyAmountToBeSold = 1;
        RiscZeroReceipt memory receipt = verifier.mockProve(ImageID.IS_SMART_METER_ID, sha256(abi.encode(energyAmountToBeSold)));

        vm.expectRevert(VerificationFailed.selector);
        energyAggregator.submitEnergyAmountToBeSold(energyAmountToBeSold, receipt.seal);
    }
}
