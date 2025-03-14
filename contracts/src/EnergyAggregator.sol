pragma solidity ^0.8.20;

import { IRiscZeroVerifier } from "risc0/IRiscZeroVerifier.sol";
import { ImageID } from "./ImageID.sol"; // auto-generated contract after running `cargo build`.

import { DataTypes } from "./utils/DataTypes.sol";

/// @title the Energy Aggregator contract
contract EnergyAggregator {
    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public immutable verifier;

    /// @notice Image ID of the only zkVM binary to accept verification from.
    ///         The image ID is similar to the address of a smart contract.
    ///         It uniquely represents the logic of that guest program,
    ///         ensuring that only proofs generated from a pre-defined guest program
    ///         (in this case, checking if a number is even) are considered valid.
    bytes32 public constant imageId = ImageID.IS_SMART_METER_ID;

    uint256 public sellOrderId;
    mapping(uint256 => DataTypes.SellOrder) public sellOrders; /// @dev - sellOrderId -> SellOrder struct
    mapping(uint256 => uint256) public energyAmountToBeSolds; /// @dev - sellOrderId -> energyAmountToBeSold / This is the energy amount that a Producer want to sell (NOTE: This is "not" all amount of energy available in the Producer, which is measured by the Producer's smart meter).
    mapping(uint256 => address) public energySellers;         /// @dev - sellOrderId -> energySeller address

    mapping(bytes => mapping(bytes32 => bool)) public monitoredNullifiers; /// @dev - To prevent from a proof double-spending attack.

    /// @notice Initialize the contract, binding it to a specified RISC Zero verifier.
    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
        //energyAmountToBeSold = 0;
    }

    /// @notice - Store a given publicInputs into the contract. Requires a RISC Zero proof that the can prove whether or not an given energyAmountToBeSold exceed the all amount of energy avaiable in a producer's smart meter.
    function submitEnergyAmountToBeSold( /// [TODO]: Rename this function name with "createSellOrderOfEnergy()"
        uint256 _energyAmountToBeSold, 
        uint256 _monitoredTime,
        bytes32 _monitoredMerkleRoot,
        //uint256 _monitored_hash_path,
        bytes32 _monitoredNullifier,    /// @dev - Nullifier (Hash) is a unique identifier for a proof, which is used to prevent double-spending attacks.
        bytes calldata seal
    ) public { /// @dev - Submitted by a Producer.
        // @dev - Validation in the smart contract level
        require(_energyAmountToBeSold > 0, "Energy amount to be sold must be greater than 0");

        // Construct the expected journal data. Verify will fail if journal does not match.
        bytes memory journal = abi.encode(_energyAmountToBeSold, _monitoredTime, _monitoredMerkleRoot, _monitoredNullifier);
        verifier.verify(seal, imageId, sha256(journal)); /// @dev - "journal" is an "encoded-publicInputs" in bytes type data.

        /// @dev - sellOrderId is counted from 1.
        sellOrderId++;
        DataTypes.SellOrder memory sellOrder = DataTypes.SellOrder({
            energyAmountToBeSold: _energyAmountToBeSold,
            energySeller: msg.sender,
            monitoredTime: _monitoredTime,
            monitoredMerkleRoot: _monitoredMerkleRoot,
            monitoredNullifier: _monitoredNullifier,
            seal: seal,
            imageId: imageId,
            journal: sha256(journal)
        });
        sellOrders[sellOrderId] = sellOrder;
        
        /// @dev - This may be able to be removed:
        //energyAmountToBeSolds[sellOrderId] = _energyAmountToBeSold;
        //energySellers[sellOrderId] = msg.sender;

        /// @dev - To prevent from a proof double-spending attack.
        monitoredNullifiers[seal][_monitoredNullifier] = true;
    }

    /// @notice - Get a energy sell order by a given sellOrderId.
    function getSellOrder(uint256 sellOrderId) public view returns (DataTypes.SellOrder memory _sellOrder) {
        return sellOrders[sellOrderId];
    }  


    /// @notice - 
    // function getEnergyAmountToBeSold(uint256 sellOrderId) public view returns (uint256) {
    //     return energyAmountToBeSolds[sellOrderId];
    // }

    /// @notice -
    // function getEnergySeller(uint256 sellOrderId) public view returns (address) {
    //     return energySellers[sellOrderId];
    // }

    /// [TODO]: Implement the following functions.
    /// @notice - A energy buyer create a buy order /w the energy amount that the buyer want to buy.
    function createBuyOrderOfEnergy(uint256 energyAmountToBeBought) public {
        // [TODO]: Matching logic that the buy order can automatically match with the sell order, which was submitted /w proof via the submitEnergyAmountToBeSold() above.
        // [TODO]: Ideally, it should be matched with 2 items (= "Asking Price" and "Asking Amount")
        _matchBuyOrderWithSellOrder(energyAmountToBeBought);
    }
    
    function _matchBuyOrderWithSellOrder(uint256 energyAmountToBeBought) internal {
        // [TODO]: Implement the logic that the buy order can automatically match with the sell order, which was submitted /w proof via the submitEnergyAmountToBeSold() above.
        // [TODO]: Ideally, it should be matched with 2 items (= "Asking Price" and "Asking Amount")
        for (uint256 i = 1; i <= sellOrderId; i++) {
            if (getSellOrder(i).energyAmountToBeSold == energyAmountToBeBought) {
                address energySeller = getSellOrder(i).energySeller;
                // [TODO]: Matched -> Execute the transaction (i.e. Pay a seller-matched for buying the energy amount).
                // [TODO]: Ideally, a Seller's "Asking SellingPrice" is also needed.
            }
        }
    }
}