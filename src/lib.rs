use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise
};
use near_sdk::log;
use near_sdk::collections::LookupMap;

use crate::util::*;
use crate::poseidon::*;
mod util;
mod merkle;
mod poseidon;

pub const ROOT_HISTORY_SIZE: u32 = 100;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AnonymousContract {
    owner: AccountId,
    denomination: Balance,
    nullifier_hash: LookupMap<Vec<u8>, bool>,
    commitments: LookupMap< [u8; 32], bool>,
    levels: u32,
    next_index: u32,
    current_root_index: u32,
    sub_trees: LookupMap<String, [u8; 32]>,
    merkle_root: LookupMap<String, [u8; 32]>,
    verifier: MixerVerifier,
}

#[near_bindgen]
impl AnonymousContract {
    #[init]
    pub fn new(denomination: Balance, tree_levels: u32) -> Self {

        Self {
            owner: env::predecessor_account_id(),
            denomination: denomination,
            nullifier_hash: LookupMap::new(vec![0u8]),
            commitments: LookupMap::new(vec![0u8]),
            levels: tree_levels,
            next_index: 0,
            current_root_index: 0,
            sub_trees: LookupMap::new(b"s"),
            merkle_root: LookupMap::new(b"m"),
            verifier: MixerVerifier::new(),
        }
    }

    #[payable]
    pub fn deposit(&mut self, commitment: [u8; 32]) {
        assert!(self.commitments.get(&&commitment).unwrap_or(false) == false, "The commitment has been submitted");

        let poseidon =  Poseidon::new();

        self.insert(poseidon, commitment);

        assert_denomination(self.denomination);
        
    }

    #[payable]
    pub fn withdraw(&mut self, proof_bytes: Vec<u8>, root: [u8; 32], nullifier_hash: [u8; 32], recipient: AccountId, relayer: AccountId, fee: u128, refund:u8){
        
        assert!(fee < self.denomination, "Fee is greater than denomination");
        assert!(self.nullifier_hash.get(&nullifier_hash.to_vec()).unwrap_or(false) == false, "The note has been already spent");

        assert!(self.is_known_root(root) == true, "Cannot find your merkle root");
        let element_encoder = |v: &[u8]| {
            let mut output = [0u8; 32];
            output.iter_mut().zip(v).for_each(|(b1, b2)| *b1 = *b2);
            output
        };
        

        let mut recipient_bytes = recipient.to_string().into_bytes();
        let mut relayer_bytes = relayer.to_string().into_bytes();

        recipient_bytes = truncate_and_pad(&recipient_bytes);
        relayer_bytes = truncate_and_pad(&relayer_bytes);
        
        let fee_bytes = element_encoder(&fee.to_be_bytes());
        let refund_bytes = element_encoder(&refund.to_be_bytes());
        
        

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&nullifier_hash);
        bytes.extend_from_slice(&root);
        bytes.extend_from_slice(&recipient_bytes);
        bytes.extend_from_slice(&relayer_bytes);
        bytes.extend_from_slice(&fee_bytes);
        bytes.extend_from_slice(&refund_bytes);

        
        let ver = MixerVerifier::new();
        let result = ver
            .verify(bytes, proof_bytes);

        assert!(result.unwrap_or(false) == true, "verify failed");
        self.nullifier_hash.insert(&nullifier_hash.to_vec(), &true);
        Promise::new(recipient).transfer(self.denomination - fee);

    }

    pub fn insert(&mut self, hasher: Poseidon, leaf: [u8; 32]) {
        let next_index = self.next_index;
        
        assert!(
            next_index != (2 as u32).pow(self.levels),
            "Merkle tree is full"
        );

        let mut current_index = next_index;
        let mut current_level_hash = leaf;
        let mut left: [u8; 32];
        let mut right: [u8; 32];


        for i in 0..self.levels {
            if current_index % 2 == 0 {
                left = current_level_hash;
                right = zeroes(i);
                self.sub_trees.insert(&i.to_string(), &current_level_hash);

            } else {
                left = self.sub_trees.get(&i.to_string()).unwrap_or([0u8; 32]);
                right = current_level_hash;
            }

            let inputs = vec![left, right];
            current_level_hash = hasher.hash(inputs).unwrap();
            current_index /= 2;
        }

        let new_root_index = (self.current_root_index + 1) % ROOT_HISTORY_SIZE;
        self.current_root_index = new_root_index;
        self.merkle_root.insert(&new_root_index.to_string(), &current_level_hash);
        self.next_index = next_index + 1;

    }

    pub fn is_known_root(&self, root: [u8; 32]) -> bool {
        assert!(root != [0u8; 32], "Root is zeros");
        
        let mut i = self.current_root_index;

        for _i in 0..ROOT_HISTORY_SIZE {
            let r = self.merkle_root.get(&i.to_string()).unwrap_or([0u8; 32]);
            if r == root {
                return true;
            }

            if i == 0 {
                i = ROOT_HISTORY_SIZE - 1;
            } else {
                i -=1;
            }
        }

        false
    }

    
    
}

fn truncate_and_pad(t: &[u8]) -> Vec<u8> {
    let mut truncated_bytes = t[..20].to_vec();
    truncated_bytes.extend_from_slice(&[0u8; 12]);
    truncated_bytes
}


