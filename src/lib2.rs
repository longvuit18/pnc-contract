use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault, Promise,
    PromiseOrValue,
};

use std::collections::HashMap;
use near_sdk::collections::LookupMap;
use lib2::merkle::*;
use lib2::util::*;
use lib2::poseidon::*;
mod util;
mod merkle;
mod poseidon;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AnonymousContract {
    owner: AccountId,
    denomination: Balance,
    fee: Balance,
    nullifier_hash: LookupMap<Vec<u8>, bool>,
    
    
}


#[near_bindgen]
impl AnonymousContract {
    #[init]
    pub fn new(_denomination: Balance, _fee: Balance, _levels: u32) -> Self {

        let merkle_tree = merkle::MerkleTree {
            levels: _levels,
            next_index: 0,
            current_root_index: 0,
            sub_trees: LookupMap::new(b"s"),
            merkle_root: LookupMap::new(b"m"),
        };

        Self {
            owner: env::predecessor_account_id(),
            denomination: _denomination,
            fee: _fee,
            merkle_tree: merkle_tree,
            nullifier_hash: LookupMap::new(vec![0u8]),
        }
    }

    #[payable]
    pub fn deposit(&mut self, _commitment: [u8; 32]) {

        let mut merkle_tree = self.merkle_tree;
        let poseidon =  Poseidon::new();

        let res = merkle_tree.insert(poseidon, _commitment);

        self.merkle_tree = merkle_tree;

        // Require xem _commitment da ton tai chua

        // Them vao tree

        // thay doi trang thai commitment

        // nhan tien
        assert_denomination(self.denomination);
        
    }

    #[payable]
    pub fn withdraw(&mut self, proof_bytes: Vec<u8>, root: [u8; 32], nullifier_hash: [u8; 32], recipient: AccountId, fee: Balance){

        let merkle_tree =  self.merkle_tree;

        if !merkle_tree.is_known_root(root) {
            // Return Falied;
            println!("Falied by not root");
        }

        if self.nullifier_hash.get(&nullifier_hash.to_vec()).unwrap() {
            println!{"Failed by nullifier"};
        }

        

    }

} 


// fn is_known_nullifier(nullifier: [u8; 32]) -> bool {

// }


