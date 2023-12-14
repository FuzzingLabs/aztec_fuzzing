cd /home/afredefon/FuzzingLabs/aztec_fuzzing_moi/noir_generator/noir_generator/

cargo build && cargo run > testNoir/test/src/main.nr

# cd testNoir/test/
# nargo check && nargo prove && nargo verify