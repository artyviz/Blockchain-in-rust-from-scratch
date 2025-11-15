// to handle block creation, hashing and mining 

use chrono :: prelude :: *;
use serde :: {Deserialize, Serialize};
use sha2 :: {Digest, Sha256};
use crate :: models :: transaction :: Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index : u64,
    pub timestamp : i64,
    pub transaction : Vec<Transaction>,
    pub previous_hash: String,
    pub nounce : u64,
    pub hash: String,
    pub merkle_root: String,
}

impl Block {
    pub fn new(index: u64, transactions : Vec<Transaction>, previous_hash : String) ->Self {
        let timestamp = Utc :: now().timestamp_millis();
        let merkle_root  = Self::calc_merkle_root(&transactions);
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce : 0,
            hash: String::new(),
            merkle_root
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let block_clone = Block { hash: String :: new(), ..self.clone()};
        let serialized = serde_json :: to_string (&block_clone).unwrap();
        let mut hasher = Sha256::new();
        hasher.update (serialized.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn mine (&mut self, difficulty : usize) {
        let target = "0".repeat(difficulty);
        loop {
            self.hash = self.calculate_hash();
            if self.hash.starts_with (&target) {
                break;
            }
            self.nonce = self.nonce.wrapping_add(1);
        }
    }

    pub fn calc_merkle_root (transactions : &Vec<Transaction>) -> String {
        if transactions.is_empty() {
            return hex :: encode(Sha256 :: digest(b""));
        }
        let mut hashes : Vec<String> = transactions.iter().map( |t| t.id.clone()).collect();
        
        while hashes.len() > 1 {
            let mut next = vec![];
            for i in (0.hashes.len()).step_by(2) {
                if i + 1 == hashes.len() {
                    let concat = format! ("{}{}", hashes[i], hashes[i]);
                    next.push(hex::encode(Sha256 :: digest(concat.as_bytes())));
                }
                else {
                    let concat = format!("{}{}", hashes[i], hashes[i + 1]);
                    next.push(hex::encode(Sha256::digest(concat.as_bytes())));
                }
            }
            hashes = next;
            //updates
        }
        hashes[0].clone()
    }
}
