# Tamagotchi on NEAR

Simple Tamagotchi clone made fully on-chain with NEAR and Rust.

This repository serves 3 main folders:
- `client`: Where the frontend code is written — basically the program where the user will interact with. Made using React and Chakra.
- `contract`: Where the smart contract is written. Made using NEAR SDK and Rust. This folder contains the business logic of the game that meets the game spec (written in the next section) such as feeding and playing.
- `nft-contract`: The repo to integrate the game with NEP-171 NFT standards so every character is a non-fungible where every account ID has different NFT. Not written from scratch due to the nature of complexities of NFT standards from NEAR itself (still ongoing development to simplify the whole process). It was taken from NEAR NFT contract templates.

# Game Specification

## Tamagotchi character
Each tamagotchi is an unique NFT based on NEP-171 NFT standards, hence, tradable.

## Feed and hungry mechanics
The player can give food to the tamagotchi. There are two types of food:
1. Meal — fills hungry hearts, adds 1 to tamagotchi weight
2. Snack — fills one hungry heart, adds 2 to the tamagotchi weight

Overfeeding will make tamagotchi sick

## Play mechanics
The player can play with the tamagotchi. The gameplay is simple, the player has to guess where the tamagotchi will be facing. The options are : left, right. If the player guess is correct, then the player wins.

Playing the game will reduce tamagotchi weight and hungry meter by 1

Winning the game will increase tamagotchi happy heart by 1

## Medicine
If tamagotchi is sick, giving medicine will cure it. A sick tamagotchi cannot be fed or played.

## Sickness
The tamagotchi will be sick if giving tamagotchi too manys nacks after the hungry meter is 4.

If tamagotchi get sick, decrease the happiness meter by 3.
