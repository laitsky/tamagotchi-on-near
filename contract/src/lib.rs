mod external;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, log, near_bindgen, require, AccountId, Balance, BorshStorageKey, Gas,
    PanicOnDefault, Promise, PromiseResult,
};

const DEPOSIT: Balance = 1_000_000_000_000_000_000_000_000; // 1 NEAR
const BASIC_GAS: Gas = Gas(5_000_000_000_000);
const MINT_GAS: Gas = Gas(30_000_000_000_000);

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    UserDetail,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TamagotchiContract {
    user_list: LookupMap<AccountId, UserDetail>,
}
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserDetail {
    token_id: String,
    receiver_id: AccountId,
    stats: TamagotchiStats,
}
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TamagotchiStats {
    pub weight: u8,
    pub hungry_meter: u8,    // 0..4
    pub happiness_meter: u8, // 0..4
    pub is_sick: bool,
    pub overfeeding_meter: u8,
}

#[ext_contract(nft)]
trait ExtNft {
    fn nft_mint(
        &mut self,
        token_id: String,
        metadata: external::TokenMetadata,
        receiver_id: AccountId,
        perpetual_royalties: Option<std::collections::HashMap<AccountId, u32>>,
    );
}

#[ext_contract(this_contract)]
trait ExtSelf {
    fn mint_cb(&self, token_id: String, receiver_id: AccountId);
}

#[near_bindgen]
impl TamagotchiContract {
    // Contract initialization
    #[init]
    pub fn new() -> Self {
        require!(!env::state_exists(), "The contract is already initialized");
        Self {
            user_list: LookupMap::new(StorageKeys::UserDetail),
        }
    }

    pub fn tamagotchi_mint(
        &mut self,
        token_id: String,
        metadata: external::TokenMetadata,
        receiver_id: AccountId,
        perpetual_royalties: Option<std::collections::HashMap<AccountId, u32>>,
    ) -> Promise {
        require!(
            self.check_user_exists(env::signer_account_id()) == false,
            "User already exist!"
        );
        let nft_account: AccountId = AccountId::new_unchecked("nft.tamagotchi.testnet".to_string());

        let promise = nft::ext(nft_account)
            .with_static_gas(MINT_GAS)
            .with_attached_deposit(DEPOSIT)
            .nft_mint(
                token_id.clone(),
                metadata,
                receiver_id.clone(),
                perpetual_royalties,
            );

        return promise.then(
            this_contract::ext(env::current_account_id())
                .with_static_gas(BASIC_GAS)
                .mint_cb(token_id, receiver_id),
        );
    }

    #[private]
    pub fn mint_cb(&mut self, token_id: String, receiver_id: AccountId) {
        // check if XCC succeeded
        if !external::did_promise_succeed() {
            log!("There was an error contacting NFT Tamagotchi Contract");
        }

        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                self.user_list.insert(
                    &env::signer_account_id(),
                    &UserDetail {
                        token_id,
                        receiver_id,
                        stats: TamagotchiStats {
                            weight: 2,
                            hungry_meter: 2,
                            happiness_meter: 2,
                            is_sick: false,
                            overfeeding_meter: 0,
                        },
                    },
                );
            }
            _ => {
                log!("There was an error contacting NFT Tamagotchi Contract");
            }
        }
    }

    pub fn feed(&mut self, food_type: String) {
        require!(
            self.check_user_exists(env::signer_account_id()),
            "User does not exist!"
        );
        require!(
            food_type == "MEAL" || food_type == "SNACK",
            "Food type incompatible"
        );
        require!(!self.is_tamagotchi_sick(env::signer_account_id()), "Tamagotchi is sick. Give medicine!");

        let stats = self.get_user_tamagotchi(env::signer_account_id());
        let mut new_weight = stats.weight;
        let mut new_hungry_meter = stats.hungry_meter;
        let mut new_overfeeding_meter = stats.overfeeding_meter;

        if food_type == "MEAL" {
            new_weight = new_weight + 1;
            match new_hungry_meter {
                u8::MIN..=3 => new_hungry_meter = 4,
                4 => new_overfeeding_meter = new_overfeeding_meter + 3,
                _ => (),
            }
        } else if food_type == "SNACK" {
            new_weight = new_weight + 2;
            match new_hungry_meter {
                u8::MIN..=3 => new_hungry_meter = new_hungry_meter + 1,
                4 => new_overfeeding_meter = new_overfeeding_meter + 1,
                _ => (),
            }
        }

        let prev = self.user_list.remove(&env::signer_account_id()).unwrap();
        let should_tamagotchi_sick: bool = match new_overfeeding_meter {
            u8::MIN..=4 => false,
            5..=u8::MAX => true,
        };
        // Character will get sick if:
        // Overfeeding meter reaches more than 5
        self.user_list.insert(
            &env::signer_account_id(),
            &UserDetail {
                token_id: prev.token_id,
                receiver_id: prev.receiver_id,
                stats: TamagotchiStats {
                    weight: new_weight,
                    hungry_meter: new_hungry_meter,
                    happiness_meter: match should_tamagotchi_sick {
                        true => external::safe_sub_u8(stats.happiness_meter , 3),
                        false => stats.happiness_meter
                    },
                    is_sick: should_tamagotchi_sick,
                    overfeeding_meter: new_overfeeding_meter,
                }
            },
        );
    }

    pub fn play(&mut self, guess: String) {
        require!(
            self.check_user_exists(env::signer_account_id()),
            "User does not exist!"
        );
        require!(
            guess == "LEFT" || guess == "RIGHT",
            "You can only move left or right!"
        );
        require!(!self.is_tamagotchi_sick(env::signer_account_id()), "Tamagotchi is sick. Give medicine!");

        let stats = self.get_user_tamagotchi(env::signer_account_id());
        require!(stats.weight > 0 || stats.hungry_meter > 0, "Tamagotchi is hungry, give some food!");

        // Playing the game will reduce weight by 1,
        // hungry meter by 1, and reset overfeeding meter
        let new_weight = external::safe_sub_u8(stats.weight, 1);
        let new_hungry_meter = external::safe_sub_u8(stats.hungry_meter, 1);
        let mut new_happiness_meter = stats.happiness_meter;
        let new_overfeeding_meter = 0;

        // Guess the direction
        // Generate pseudorandom number
        // 0 means "LEFT", 1 means "RIGHT"
        let rand: u8 = (*env::random_seed().get(0).unwrap()) % 2;
        if (guess == "LEFT" && rand == 0) || (guess == "RIGHT" && rand == 1) {
            match new_happiness_meter {
                0..=3 => new_happiness_meter = new_happiness_meter + 1,
                4 => (),
                _ => (),
            }
        }

        let prev = self.user_list.remove(&env::signer_account_id()).unwrap();
        // Character will get sick if:
        // Weight or hungry or happiness meter reaches 0
        self.user_list.insert(
            &env::signer_account_id(),
            &UserDetail {
                token_id: prev.token_id,
                receiver_id: prev.receiver_id,
                stats: TamagotchiStats {
                    weight: new_weight,
                    hungry_meter: new_hungry_meter,
                    happiness_meter: new_happiness_meter,
                    is_sick: prev.stats.is_sick,
                    overfeeding_meter: new_overfeeding_meter,
                },
            },
        );
    }

    pub fn cure(&mut self) {
        require!(
            self.check_user_exists(env::signer_account_id()),
            "User does not exist!"
        );
        let prev = self.user_list.remove(&env::signer_account_id()).unwrap();

        self.user_list.insert(
            &env::signer_account_id(),
            &UserDetail {
                token_id: prev.token_id,
                receiver_id: prev.receiver_id,
                stats: TamagotchiStats {
                    weight: 1,
                    hungry_meter: 1,
                    happiness_meter: 1,
                    is_sick: false,
                    overfeeding_meter: 0,
                },
            },
        );
    }

    pub fn check_user_exists(&self, address: AccountId) -> bool {
        self.user_list.contains_key(&address)
    }

    pub fn get_user_tamagotchi(&self, address: AccountId) -> TamagotchiStats {
        require!(
            self.check_user_exists(address.clone()),
            "User does not exist!"
        );
        self.user_list.get(&address).unwrap().stats
    }

    pub fn is_tamagotchi_sick(&self, address: AccountId) -> bool {
        require!(
            self.check_user_exists(address.clone()),
            "User does not exist!"
        );
        match self.get_user_tamagotchi(address).is_sick {
            true => true,
            false => false,
        }
    }
}
