

./build.sh
near delete privatecash.longvuit18.testnet longvuit18.testnet
near create-account privatecash.longvuit18.testnet --masterAccount longvuit18.testnet --initialBalance 20
near deploy --accountId privatecash.longvuit18.testnet --wasmFile out/NEAR_Private_Cash.wasm --initFunction new --initArgs '{"denomination": 1000000000000000000000000, "tree_levels": 30}'
