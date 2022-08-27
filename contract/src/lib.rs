use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Promise, Balance};
use near_sdk::collections::{UnorderedMap};

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

mod views;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub account: AccountId,
  pub transactions: UnorderedMap<AccountId, u128>,
}

impl Default for Contract {
  fn default() -> Self {
    Self{
      account: "v1.faucet.nonofficial.testnet".parse().unwrap(),
      transactions: UnorderedMap::new(b"d"),
    }
  }
}

#[near_bindgen]
impl Contract {
  #[init]
  #[private] // Public - but only callable by env::current_account_id()
  pub fn new(account: AccountId) -> Self {
    assert!(!env::state_exists(), "Already initialized");
    Self {
      account,
      transactions: UnorderedMap::new(b"d"),
    }
  }

  #[payable] // Public - People can attach money
  pub fn send(&mut self) -> U128 {
    // Get who is calling the method and how much $NEAR they attached
    let receiver: AccountId = env::predecessor_account_id();
    let amount: Balance = env::attached_deposit();

    
    self.transactions.insert(&receiver, &amount);
    
    log!("You've successfully sent {}!", amount);
    
    // Send the money to the beneficiary
    Promise::new(self.account.clone()).transfer(amount);

    // Return the total amount donated so far
    U128(amount)
  }
}