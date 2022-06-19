// use serde::{Deserialize, Serialize};
// use near_sdk::{
//     env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault, Promise,
//     PromiseOrValue,
// };

// use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

// use near_sdk::collections::LookupMap;

// use crate::poseidon::Poseidon;
// use std::result::Result;
// use crate::util::*;

// pub const ROOT_HISTORY_SIZE: u32 = 100;


// #[derive(Debug, BorshDeserialize, BorshSerialize)]
// pub struct MerkleTree {
//     pub levels: u32,
//     pub next_index: u32,
//     pub current_root_index: u32,
//     pub sub_trees: LookupMap<String, [u8; 32]>,
//     pub merkle_root: LookupMap<String, [u8; 32]>,
// }


// impl MerkleTree {
//     // fn hash_left_right(&mut self, hasher: Poseidon, left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
//     //     let inputs = vec![left, right];
//     //     hasher.hash(inputs)
//     // }

//     pub fn insert(&mut self, hasher: Poseidon, leaf: [u8; 32]) {
//         let next_index = self.next_index;

//         assert!(
//             next_index != 2u32.pow(self.levels),
//             "Merkle tree is full"
//         );

//         let mut current_index = next_index;
//         let mut current_level_hash = leaf;
//         let mut left: [u8; 32];
//         let mut right: [u8; 32];

//         for i in 0..self.levels {
//             if current_index %2 == 0 {
//                 left = current_level_hash;
//                 //right = zeroes::zeroes(i);
//                 // TODO Fix zeroes
//                 right = zeroes(i);

                
//                 // self.sub_trees.insert(&i.to_string(), &current_level_hash);
//             } else {

//                 // left = self.sub_trees.get(&i.to_string()).unwrap_or([0u8; 32]);
//                 right = current_level_hash;
//             }

//             // current_level_hash = self.hash_left_right(hasher.clone(), left, right);
//             let inputs = vec![left, right];
//             current_level_hash = hasher.hash(inputs).unwrap();
//             current_index /=2;
//         }

//         let new_root_index  = (self.current_root_index + 1) % ROOT_HISTORY_SIZE;
//         self.current_root_index = new_root_index;

    

//         // self.merkle_root.insert(&new_root_index.to_string(), &current_level_hash);

//         self.next_index =  next_index + 1;

//     }

//     pub fn is_known_root(&self, root: [u8; 32]) -> bool {
//         if root == [0u8; 32] {
//             return false
//         }

//         let mut i = self.current_root_index;

//         // for _i in 0..ROOT_HISTORY_SIZE {
//         //     let r = self.merkle_root.get(&i.to_string()).unwrap_or([0u8; 32]);
//         //     if r == root {
//         //         return true;
//         //     }

//         //     if i == 0 {
//         //         i = ROOT_HISTORY_SIZE - 1;
//         //     } else {
//         //         i -=1;
//         //     }
//         // }

//         false
//     }
// }