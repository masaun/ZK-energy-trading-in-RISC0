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

interface IEnergyAggregator {
    function createSellOrder(
        uint256 _energyAmountToBeSold, 
        uint256 _monitoredTime,
        bytes32 _monitoredMerkleRoot,
        //uint256 _monitored_hash_path,
        bytes32 _monitoredNullifier,    /// @dev - Nullifier (Hash) is a unique identifier for a proof, which is used to prevent double-spending attacks.
        bytes calldata seal) external;


    function getEnergyAmountToBeSold() external view returns (uint256);
}
