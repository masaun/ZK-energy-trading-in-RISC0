echo "Read the environment variables"
. ./.env # load the environment variables from the .env file for deployment

echo "Running the app (./apps/src/main.rs) with the following environment variables:"
RUST_LOG=info cargo run --bin app -- --energy-aggregator-address ${ENERGY_AGGREGATOR_ADDRESS:?} --amount-of-energy-to-be-sold 800
#RUST_LOG=info cargo run --bin app -- --even-number-address ${EVEN_NUMBER_ADDRESS:?} --number 4