#!/bin/bash
set -e

sh ./build.sh
near delete game.tamagotchi.testnet tamagotchi.testnet || :
near create-account game.tamagotchi.testnet --masterAccount tamagotchi.testnet --initialBalance 5
near deploy --wasmFile res/tamagotchi_contract.wasm --accountId game.tamagotchi.testnet --initFunction new --initArgs '{}'