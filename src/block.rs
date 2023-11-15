use crate::{errors::Result, transaction::Transaction};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::info;
use merkle_cbt::merkle_tree::Merge;
use merkle_cbt::merkle_tree::CBMT;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

const TARGET_HEXT: usize = 4;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp: u128,
    transactions: Vec<Transaction>,
    prev_block_hash: String,
    hash: String,
    height: i32,
    nonce: i32,
}

impl Block {
    pub fn get_height(&self) -> i32 {
        self.height
    }
    pub fn get_transaction(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub(crate) fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn new_genesis_block(coninbase: Transaction) -> Block {
        Block::new_block(vec![coninbase], String::new(), 0).unwrap()
    }

    pub fn new_block(
        data: Vec<Transaction>,
        prev_block_hash: String,
        height: i32,
    ) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();
        let mut block = Block {
            timestamp: timestamp,
            transactions: data,
            prev_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };
        block.run_proof_if_work()?;
        Ok(block)
    }
    fn run_proof_if_work(&mut self) -> Result<()> {
        info!("Mining the block");
        while !self.validate()? {
            self.nonce += 1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    /// HashTransactions returns a hash of the transactions in the block
    fn hash_transactions(&self) -> Result<Vec<u8>> {
        let mut transactions = Vec::new();
        for tx in &self.transactions {
            transactions.push(tx.hash()?.as_bytes().to_owned());
        }
        let tree = CBMT::<Vec<u8>, MergeTX>::build_merkle_tree(&transactions);
        Ok(tree.root())
    }

    // 这段代码定义了一个名为 prepare_hash_data 的函数，该函数返回一个 Result<Vec<u8>> 类型的结果。函数的作用是准备用于哈希计算的数据。
    // 这段代码的作用是将一些数据按照特定的顺序组成一个元组，并将该元组序列化为字节流，最后返回字节流作为结果。
    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        // 4. 将这些变量组成一个元组 content 。
        let content = (
            // 1. 将 self.prev_block_hash 复制到一个新的变量中。
            self.prev_block_hash.clone(),
            // 2. 调用 self.hash_transactions() 函数来获取交易的哈希值，并将其结果存储到新变量中。
            self.hash_transactions()?,
            // 3. 将 self.timestamp 、 TARGET_HEXT 和 self.nonce 依次存储到新变量中。
            self.timestamp,
            TARGET_HEXT,
            self.nonce,
        );
        // 5. 使用 bincode::serialize 函数将 content 序列化为字节流，并将结果存储到新变量 bytes 中。
        let bytes = bincode::serialize(&content)?;
        // 6. 返回 Ok(bytes) ，表示成功地准备了哈希计算所需的数据。
        Ok(bytes)
    }

    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1: Vec<u8> = vec![];
        vec1.resize(TARGET_HEXT, '0' as u8);
        // println!("{:?}", vec1);
        Ok(&hasher.result_str()[0..TARGET_HEXT] == String::from_utf8(vec1)?)
    }
}

// 该代码定义了一个名为MergeTX的结构体，并实现了Merge trait。
// Merge trait 的作用是将两个Vec<u8>类型的数据合并成一个，并返回合并后的结果。
struct MergeTX {}

// 实现了Merge trait的merge方法，该方法接收两个参数left和right，类型为&Self::Item，即Vec<u8>。
// 该代码的功能是将两个Vec<u8>类型的数据合并成一个，并对合并后的数据进行SHA256哈希运算，最终返回哈希结果。
impl Merge for MergeTX {
    type Item = Vec<u8>;
    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        // 创建了一个Sha256的哈希对象hasher。
        let mut hasher = Sha256::new();
        // 创建一个可变的Vec<u8>类型的变量data，并将left的克隆添加到data中。
        let mut data: Vec<u8> = left.clone();
        // 将right的克隆追加到data中。
        data.append(&mut right.clone());
        // 将data作为输入传递给hasher。
        hasher.input(&data);
        // 创建一个长度为32的u8数组re，并将其所有元素初始化为0。
        let mut re: [u8; 32] = [0; 32];
        // 将hasher的结果存储到re中。
        hasher.result(&mut re);
        // 将re转换为Vec<u8>类型，并作为结果返回。
        re.to_vec()
    }
}
