set -e

near delete nft.tamagotchi.testnet tamagotchi.testnet || :
near create-account nft.tamagotchi.testnet --masterAccount tamagotchi.testnet --initialBalance 5
near deploy --wasmFile res/contract.wasm --accountId nft.tamagotchi.testnet
near call nft.tamagotchi.testnet new_default_meta '{"owner_id": "nft.tamagotchi.testnet"}' --accountId nft.tamagotchi.testnet