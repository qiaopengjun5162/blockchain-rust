use std::{collections::HashMap, thread::sleep};

use crate::errors::Result;
use bitcoincash_addr::{Address, HashType, Scheme};
use crypto::digest::Digest;
use crypto::ed25519;
use crypto::ripemd160::Ripemd160;
use crypto::sha2::Sha256;
use log::info;
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wallet {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl Wallet {
    fn new() -> Self {
        let mut key: [u8; 32] = [0; 32];
        OsRng.fill_bytes(&mut key);
        let (secret_key, public_key) = ed25519::keypair(&key);
        let secret_key = secret_key.to_vec();
        let public_key = public_key.to_vec();
        Wallet {
            secret_key,
            public_key,
        }
    }

    fn get_address(&self) -> String {
        // 1. 将self的public_key克隆到pub_hash中。
        let mut pub_hash = self.public_key.clone();
        // 2. 调用hash_pub_key函数，对pub_hash进行哈希处理。
        hash_pub_key(&mut pub_hash);
        // 3. 创建一个Address结构体，其中body字段为pub_hash，scheme字段为Scheme::Base58，hash_type字段为HashType::Script，其他字段使用默认值。
        let address = Address {
            body: pub_hash,
            scheme: Scheme::Base58,
            hash_type: HashType::Script,
            ..Default::default()
        };
        // 0 O 1 I
        // 4. 调用address的encode方法，并使用unwrap()解包结果，得到最终的地址。
        address.encode().unwrap()
    }
}

pub fn hash_pub_key(pub_key: &mut Vec<u8>) {
    // 这段代码的功能是对给定的公钥进行哈希处理。
    // 代码的步骤如下：
    // 1. 定义了一个可变的 Vec<u8> 类型的 pub_key 参数。
    // 2. 创建一个 Sha256 类型的哈希器 hasher1。
    let mut hasher1 = Sha256::new();
    // 3. 将 pub_key 作为输入传递给 hasher1。
    hasher1.input(pub_key);
    // 4. 将 hasher1 的结果保存到 pub_key 中。
    hasher1.result(pub_key);
    // 5. 创建一个 Ripemd160 类型的哈希器 hasher2。
    let mut hasher2 = Ripemd160::new();
    // 6. 将 pub_key 作为输入传递给 hasher2。
    hasher2.input(pub_key);
    // 7. 将 pub_key 的大小调整为 20，并用 0 填充。
    pub_key.resize(20, 0);
    // 8. 将 hasher2 的结果保存到 pub_key 中。
    hasher2.result(pub_key);
}

pub struct Wallets {
    wallets: HashMap<String, Wallet>,
}

impl Wallets {
    pub fn new() -> Result<Wallets> {
        let mut wlt = Wallets {
            wallets: HashMap::<String, Wallet>::new(),
        };

        let db = sled::open("data/wallets")?;
        for item in db.into_iter() {
            let i = item?;
            let address = String::from_utf8(i.0.to_vec())?;
            let wallet = bincode::deserialize(&i.1.to_vec())?;
            wlt.wallets.insert(address, wallet);
        }
        drop(db);
        Ok(wlt)
    }

    pub fn create_wallet(&mut self) -> String {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        self.wallets.insert(address.clone(), wallet);
        info!("Created wallet with address: {}", address);
        address
    }

    pub fn get_all_address(&self) -> Vec<String> {
        let mut addresses = Vec::new();
        for (address, _) in &self.wallets {
            addresses.push(address.clone());
        }
        addresses
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallets.get(address)
    }

    pub fn save_all(&self) -> Result<()> {
        let db = sled::open("data/wallets")?;
        for (address, wallet) in &self.wallets {
            let data = bincode::serialize(wallet)?;
            db.insert(address, data)?;
        }
        db.flush()?;
        drop(db);
        Ok(())
    }
}
