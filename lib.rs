use near_sdk::{
  borsh::{self, *},
  collections::*,
  json_types::*,
  serde::{self, *},
  *,
};

pub type AccountId = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Vehicle {
  reg_no: String,
  owner: AccountId,
  phone_no: String,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ParkingSlot {
  slot_id: u32,
  price: u128,
  location: String,
  status: String,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Booking {
  ticket_no: u32,
  slot_id: u32,
  duration: u32,
  amount_paid: u128,
}

#[near_bindgen]
pub struct Contract {
  owner: AccountId,
  vehicles: Vec<Vehicle>,
  parking_slots: Vec<ParkingSlot>,
  bookings: Vec<Booking>,
}

#[near_bindgen]
impl Contract {
  // add code here
  #[init]
  pub fn new(owner: AccountId) -> Self {
    let vehicles: Vec<Vehicle> = Vec::new();
    let parking_slots: Vec<ParkingSlot> = Vec::new();
    let bookings: Vec<Booking> = Vec::new();

    Contract {
      owner,
      vehicles,
      parking_slots,
      bookings,
    }
  }

  // admin is the current_account_id (the id holding the smart contract)
  pub fn add_parking_slot(&mut self, price: u128, location: String) {
    let parking_slots_length: usize = self.parking_slots.len();
    let slot = ParkingSlot {
      slot_id: parking_slots_length as u32,
      price,
      location,
      status: "available".to_string(),
    };

    if env::signer_account_id() == env::current_account_id() {
      &self.parking_slots.push(slot);
      env::log_str("parking slot added succesfully");
    } else {
      env::log_str("you dont have permission");
    }
  }

  pub fn book_a_slot(&mut self) {}

  pub fn view_parking_slots(&self) -> &Vec<ParkingSlot> {
    &self.parking_slots
  }
}

#[cfg(test)]
mod tests {
  use crate::*;
  use near_sdk::{test_utils::*, testing_env, AccountId};

  const ONE_NEAR: u128 = u128::pow(10, 24);

  fn contract_account() -> AccountId {
    "admin.testnet".parse::<AccountId>().unwrap()
  }

  fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
      .current_account_id(contract_account())
      .account_balance(15 * ONE_NEAR)
      .signer_account_id(predecessor_account_id.clone())
      .predecessor_account_id(predecessor_account_id);
    builder
  }

  #[test]
  // only admin can run this
  fn test_add_parking_slot() {
    let denniz = AccountId::new_unchecked("admin.testnet".to_string());
    let context = get_context(denniz.clone());
    testing_env!(context.build());

    let mut contract = Contract::new(denniz.to_string());
    contract.add_parking_slot(150, "Kisumu".to_string());
    contract.add_parking_slot(120, "Kisian".to_string());
    contract.add_parking_slot(110, "United Mall".to_string());

    assert_eq!(contract.parking_slots.len(), 3);
  }

  #[test]
  fn test_add_parking_slot_only_admin() {
    let denniz = AccountId::new_unchecked("denniz.testnet".to_string());
    let context = get_context(denniz.clone());
    testing_env!(context.build());

    let mut contract = Contract::new(denniz.to_string());
    contract.add_parking_slot(150, "Kisumu".to_string());
    contract.add_parking_slot(120, "Kisian".to_string());
    contract.add_parking_slot(110, "United Mall".to_string());

    assert_eq!(contract.parking_slots.len(), 0);
  }
}
