use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, PromiseResult};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::Base64VecU8;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

pub fn did_promise_succeed() -> bool {
    if env::promise_results_count() != 1 {
        log!("Expected a result on the callback");
        return false;
    }

    match env::promise_result(0) {
        PromiseResult::Successful(_) => true,
        _ => false,
    }
}

// Safe substract [max(a-b, 0)]
pub fn safe_sub_u8(a: u8, b: u8) -> u8 {
    let result: u8 = match a.checked_sub(b) {
        Some(val) => val,
        None => u8::MIN, // minimum val of u8
    };
    result
}