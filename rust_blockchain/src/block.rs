use chrono::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub data: String,
    pub hash: String,
}

impl Block {

    /// Creates a new block with the specified index, data, and previous hash.
    ///
    /// The block's timestamp is set to the current time in RFC 3339 format.
    /// The nonce is initialized to zero, and the hash is computed based on
    /// the block's content.
    ///
    /// # Arguments
    ///
    /// * `index` - The position of the block in the blockchain.
    /// * `data` - The data to be stored in the block.
    /// * `previous_hash` - The hash of the previous block in the blockchain.
    ///
    /// # Returns
    ///
    /// A new instance of `Block`.
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp: Utc::now().to_rfc3339(),
            previous_hash,
            nonce: 0,
            data,
            hash: String::new(),
        };
        block.hash = block.compute_hash();
        block
    }

    /// Compute the hash of the block.
    ///
    /// The hash is computed from the string `"{index}{timestamp}{previous_hash}{nonce}{data}"`.
    /// The timestamp is formatted as a string in the ISO 8601 (RFC 3339) format.
    /// The hash is then formatted as a lower-case hexadecimal string.
    pub fn compute_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.previous_hash, self.nonce, self.data
        ));
        format!("{:x}", hasher.finalize())
    }

    /// Mines the block by finding a nonce that results in a hash that starts with
    /// at least `diff` zero bits.
    ///
    /// # Arguments
    ///
    /// * `diff` - The number of zero bits that the hash must start with.
    ///
    /// # Effects
    ///
    /// This method increments the nonce until it finds a hash that satisfies the
    /// difficulty requirement. It then prints a message to the console indicating
    /// that the block has been mined.
    pub fn mine_block(&mut self,diff:usize) {
        let target = "0".repeat(diff);
        while self.hash[..diff] != target {
            self.nonce += 1;
            self.hash = self.compute_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}
