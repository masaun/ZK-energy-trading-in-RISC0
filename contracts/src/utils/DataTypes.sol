pragma solidity ^0.8.20;


/// @title the Energy Aggregator contract
library DataTypes {

    struct SellOrder { /// [Key]: sellOrderId
        uint256 energyAmountToBeSold; /// Asking amount of energy to be sold
        address energySeller;         /// Seller's address
        uint256 monitoredTime;
        bytes32 monitoredMerkleRoot;
        bytes32 monitoredNullifier;
        bytes seal;
        bytes32 imageId;
        bytes32 journal;
    }

}