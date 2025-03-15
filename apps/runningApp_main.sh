echo "Read the environment variables"
. ./.env # load the environment variables from the .env file for deployment

echo "Update the guest program"
cargo build

echo "Running the app (./apps/src/main.rs) with the following environment variables:"
RUST_LOG=info cargo run --bin app -- --energy-aggregator-address ${ENERGY_AGGREGATOR_ADDRESS:?} \
                                  --amount-of-energy-to-be-sold ${AMOUNT_OF_ENERGY_TO_BE_SOLD:?} \
                                  --total-exact-amount-of-energy-available ${TOTAL_EXACT_AMOUNT_OF_ENERGY_AVAILABLE:?} \
                                  --current-time ${AMOUNT_OF_ENERGY_TO_BE_SOLD:?} \
                                  --monitored-time ${MONITORED_TIME:?} \
                                  --monitored-merkle-root ${MONITORED_MERKLE_ROOT:?} \
                                  --monitored-nullifier ${MONITORED_NULLIFIER:?}                    

#RUST_LOG=info cargo run --bin app -- --even-number-address ${EVEN_NUMBER_ADDRESS:?} --number 4