pragma solidity ^0.8.20;

import { IRiscZeroVerifier } from "risc0/IRiscZeroVerifier.sol";
import { ImageID } from "./ImageID.sol"; // auto-generated contract after running `cargo build`.

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

    /// @notice A number that is guaranteed, by the RISC Zero zkVM, to be even.
    ///         It can be set by calling the `set` function.
    uint256 public energyAmountToBeSold; /// @dev - This is the energy amount that a Producer want to sell (NOTE: This is "not" all amount of energy available in the Producer, which is measured by the Producer's smart meter).

    /// @notice Initialize the contract, binding it to a specified RISC Zero verifier.
    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
        energyAmountToBeSold = 0;
    }

    /// @notice Set the even number stored on the contract. Requires a RISC Zero proof that the number is even.
    function submitEnergyAmountToBeSold(uint256 _energyAmountToBeSold, bytes calldata seal) public { /// @dev - Submitted by a Producer.
        // Construct the expected journal data. Verify will fail if journal does not match.
        bytes memory journal = abi.encode(_energyAmountToBeSold);
        verifier.verify(seal, imageId, sha256(journal));
        energyAmountToBeSold = _energyAmountToBeSold;
    }

    /// @notice Returns the number stored.
    function getEnergyAmountToBeSold() public view returns (uint256) {
        return energyAmountToBeSold;
    }

    /// [TODO]:
}