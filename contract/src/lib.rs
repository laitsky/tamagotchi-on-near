use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, require, env, PanicOnDefault};
use near_sdk::serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TamagotchiContract {
    weight: u64,
    hungry_meter: u8, // 0..4
    happiness_meter: u8, // 0..4
    is_sick: bool,
    overfeeding_meter: u8,
}

#[near_bindgen]
impl TamagotchiContract {
    // Contract initialization
    #[init]
    pub fn new() -> Self {
        require!(!env::state_exists(), "The contract is already initialized");
        Self { weight: 2, hungry_meter: 2, happiness_meter: 2, is_sick: false, overfeeding_meter: 0 }
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

    pub fn get_state(&self) -> TamagotchiContract {
        TamagotchiContract {
            weight: self.weight,
            hungry_meter: self.hungry_meter, // 0..4
            happiness_meter: self.happiness_meter, // 0..4
            is_sick: self.is_sick,
            overfeeding_meter: self.overfeeding_meter,
        }
    }

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

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use near_sdk::{testing_env, VMContext};

    use super::*;

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string().parse().unwrap(),
            signer_account_id: "robert.testnet".to_string().parse().unwrap(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string().parse().unwrap(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            output_data_receivers: vec![],
            epoch_height: 19,
            is_view: false
        }
    }

    #[test]
    fn test_rng() {
        let ctx = get_context(vec![]);
        testing_env!(ctx);

        let mut contract = TamagotchiContract::new();

        println!("{:?}", contract.play("RIGHT".to_string()));
    }
    
}
