use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};

#[near_bindgen]
impl Default for Contract {
    fn default() Self -> {
        return Contract {
            name: String::from("A default string"),
        };
    }
}