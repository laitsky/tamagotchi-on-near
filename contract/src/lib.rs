use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{near_bindgen, require, env, PanicOnDefault, Gas, Balance, ext_contract, AccountId, Promise, PromiseResult, log, BorshStorageKey};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{Base64VecU8};

const DEPOSIT: Balance = 1_000_000_000_000_000_000_000_000; // 1 NEAR
const BASIC_GAS: Gas = Gas(5_000_000_000_000);
const MINT_GAS: Gas = Gas(30_000_000_000_000);

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    UserList,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TamagotchiContract {
    user_list: LookupMap<AccountId, UserList>,
    weight: u64,
    hungry_meter: u8, // 0..4
    happiness_meter: u8, // 0..4
    is_sick: bool,
    overfeeding_meter: u8,
}
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserList {
    token_id: String,
    receiver_id: AccountId
}

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

#[ext_contract(nft)]
trait ExtNft {
    fn nft_mint(
        &mut self,
        token_id: String,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        perpetual_royalties: Option<std::collections::HashMap<AccountId,u32>>
    );
}

#[ext_contract(this_contract)]
trait ExtSelf {
    fn mint_cb(&self, token_id: String, receiver_id: AccountId);
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

#[near_bindgen]
impl TamagotchiContract {
    // Contract initialization
    #[init]
    pub fn new() -> Self {
        require!(!env::state_exists(), "The contract is already initialized");
        Self { user_list:LookupMap::new(StorageKeys::UserList) ,weight: 2, hungry_meter: 2, happiness_meter: 2, is_sick: false, overfeeding_meter: 0 }
    }

    pub fn tamagotchi_mint(
        &mut self,
        token_id: String,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        perpetual_royalties: Option<std::collections::HashMap<AccountId,u32>>
    ) -> Promise {
        let nft_account: AccountId = AccountId::new_unchecked("nft.tamagotchi.testnet".to_string());

        let promise = nft::ext(nft_account)
            .with_static_gas(MINT_GAS)
            .with_attached_deposit(DEPOSIT)
            .nft_mint(token_id.clone(), metadata, receiver_id.clone(), perpetual_royalties);

        return promise.then(
            this_contract::ext(env::current_account_id())
            .with_static_gas(BASIC_GAS)
            .mint_cb(token_id, receiver_id)
        )
    }

    #[private]
    pub fn mint_cb(&mut self, token_id: String, receiver_id: AccountId) {
        // check if XCC succeeded
        if !did_promise_succeed() {
            log!("There was an error contacting NFT Tamagotchi Contract");
        }

        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                self.user_list.insert(&env::signer_account_id(), &UserList { token_id, receiver_id });
            },
            _ => { log!("There was an error contacting NFT Tamagotchi Contract");}
        }
    }

    pub fn feed(&mut self, food_type: String) {
        require!(food_type == "MEAL" || food_type == "SNACK", "Food type incompatible");

        self.check_if_sick();
        require!(self.is_sick == false, "Tamagotchi is sick. Give medicine!");
        
        if food_type == "MEAL" {
            self.weight = self.weight + 1;
            if self.hungry_meter < 4 {
                self.hungry_meter = 4;
            } else if self.hungry_meter == 4 {
                self.overfeeding_meter = self.overfeeding_meter + 3;
            }
        } else if food_type == "SNACK" {
            self.weight = self.weight + 2;
            if self.hungry_meter < 4 {
                self.hungry_meter = self.hungry_meter + 1;
            } else if self.hungry_meter == 4 {
                self.overfeeding_meter = self.overfeeding_meter + 1;
            }
        }
    }
    
    pub fn play(&mut self, guess: String) {
        require!(guess == "LEFT" || guess == "RIGHT", "You can only move left or right!");

        self.check_if_sick();
        require!(self.is_sick == false, "Tamagotchi is sick. Give medicine!");

        // Playing the game will reduce weight by 1,
        // hungry meter by 1, and reset sick meter
        self.weight = self.weight - 1;
        self.hungry_meter = self.hungry_meter - 1;
        self.overfeeding_meter = 0;

        // Guess the direction
        // Generate pseudorandom number
        // 0 means "LEFT", 1 means "RIGHT"
        let rand: u8 = (*env::random_seed().get(0).unwrap()) % 2;
        
        if guess == "LEFT" && rand == 0 {
            match self.happiness_meter {
                0..=3 => self.happiness_meter = self.happiness_meter + 1,
                4 => self.happiness_meter = 4,
                4..=u8::MAX => ()
            }
        } else if guess == "RIGHT" && rand == 1 {
            match self.happiness_meter {
                0..=3 => self.happiness_meter = self.happiness_meter + 1,
                4 => self.happiness_meter = 4,
                4..=u8::MAX => ()
            }
        } 
    }

    pub fn cure(&mut self) {
        self.is_sick = false;
        self.weight = 1;
        self.happiness_meter = 1;
        self.hungry_meter = 1;
        self.overfeeding_meter = 0;
    }

    pub fn get_user(&self, address: AccountId) -> bool {
        match self.user_list.get(&address) {
            Some(_) => true,
            None => false
        }
    }

    // pub fn get_state(&self) -> TamagotchiContract {
    //     TamagotchiContract {
    //         weight: self.weight,
    //         hungry_meter: self.hungry_meter, // 0..4
    //         happiness_meter: self.happiness_meter, // 0..4
    //         is_sick: self.is_sick,
    //         overfeeding_meter: self.overfeeding_meter,
    //     }
    // }

    pub fn check_if_sick(&mut self) {
        // Character will get sick if:
        // Overfeeding meter reaches more than 5;
        // Weight or hungry or happiness meter reaches 0 
        if self.overfeeding_meter >= 5 ||
            self.weight == 0 || 
            self.hungry_meter == 0 ||
            self.happiness_meter == 0
        {
            self.is_sick = true;
            self.happiness_meter = TamagotchiContract::safe_sub_u8(self.happiness_meter, 3);
        } 
    }

    // Safe substract [max(a-b, 0)]
    fn safe_sub_u8(a: u8, b: u8) -> u8 {
        let result: u8 = match a.checked_sub(b) {
            Some(val) => val,
            None => u8::MIN // minimum val of u8
        };
        result
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

