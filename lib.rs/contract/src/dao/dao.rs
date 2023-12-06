use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};

// Define the default status
const DEFAULT_ORG_STATUS: &bool = false;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    message: String,
}

// Inititalize the DAO
#[near_bindgen]
impl Default for Contract {
    fn default() -> Self {
        Self {status: DEFAULT_ORG_STATUS.replace(status)};
    }
}

// Voter Rights & Account Statuses
#[near_bindgen]
#[derive()]
impl Contract {
    // Grab the users
    // let mut current_user = near_address;
    fn set_the_status() {
        let mut customer_info = map.get_address_metadata();
    }

}

// Allocation of funds (Allo Protocol)
#[near_bindgen]
impl Contract {
    fn allocate_funds(&mut self){
        // Input template
        let mut input_template = {
            let mut input_template = String::new();
            input_template.push_str("input_template");
            input_template
        };

        // Output template
        let mut output_template = {
            let mut output_template = String::new();
            output_template.push_str("output_template");
            output_template
        };

        // Input amount
        let mut input_amount = {
            let mut input_amount = String::new();
            input_amount.push_str("input_amount");
            input_amount
        };

        // Output amount
        let mut output_amount = {
            let mut output_amount = String::new();
            output_amount.push_str("output_amount");
            output_amount
        };

        // Input token
        let mut input_token = {
            let mut input_token = String::new();
            input_token.push_str("input_token");
            input_token
        };

        // Output token
        let mut output_token = {
            let mut output_token = String::new();
            output_token.push_str("output_token");
            output_token
        };

        // Input token amount
        let mut input_token_amount = {
            let mut input_token_amount = String::new();
            input_token_amount.push_str("input_token_amount");
            input_token_amount
        };

        // Output token amount
        let mut output_token_amount = {
            let mut output_token_amount = String::new();
            output_token_amount.push_str("output_token_amount");
            output_token_amount
        };
    }
}

