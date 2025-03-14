echo "Read the environment variables..."
. ./.env # load the environment variables from the .env file for deployment

echo "Compile the contracts to be tested..."
forge build

echo "Running the test of the EnergyAggregator on Ethereum Sepolia testnet..."
forge test --optimize --optimizer-runs 5000 --evm-version cancun --match-contract EnergyAggregatorTest --rpc-url ${RPC_URL:?} -vvv