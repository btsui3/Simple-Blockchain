extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate time;

use self::sha2::{Digest, Sha256};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
     sender: String,
     reciever: String,
     amount: f32,  
}
#[#[derive(Serialize, Debug)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };
        chain.generate_new_block();
        chain
    }
}