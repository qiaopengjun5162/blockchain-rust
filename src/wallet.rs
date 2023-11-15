use std::collections::HashMap;

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
        // 这段代码是一个创建钱包的函数，其功能是生成一个新的密钥对并返回一个包含密钥对的钱包对象。
        // 1. 函数定义了一个名为new的方法，返回类型为Self（即钱包对象）。
        // 2. 创建一个长度为32的u8数组key，并将其所有元素初始化为0。
        let mut key: [u8; 32] = [0; 32];
        // 3. 使用操作系统提供的随机数生成器OsRng填充key数组。
        OsRng.fill_bytes(&mut key);
        // 4. 调用ed25519::keypair方法，传入key数组作为参数，生成一个密钥对，分别赋值给secret_key和public_key。
        let (secret_key, public_key) = ed25519::keypair(&key);
        // 5. 将secret_key转换为Vec<u8>类型，并赋值给secret_key变量。
        let secret_key = secret_key.to_vec();
        // 6. 将public_key转换为Vec<u8>类型，并赋值给public_key变量。
        let public_key = public_key.to_vec();
        // 7. 创建一个新的钱包对象，将secret_key和public_key作为属性赋值给该对象。
        // 8. 返回创建的钱包对象。
        Wallet {
            secret_key,
            public_key,
        }
    }

    pub fn get_address(&self) -> String {
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
        // 这段代码的功能是创建一个新的钱包对象，并从数据库中加载现有的钱包数据。
        // 1. 创建一个名为wlt的可变变量，类型为Wallets结构体，其中包含一个HashMap用于存储钱包数据。
        let mut wlt = Wallets {
            wallets: HashMap::<String, Wallet>::new(),
        };
        // 2. 使用sled::open函数打开名为"data/wallets"的数据库，并将返回的结果赋值给db变量。
        let db = sled::open("data/wallets")?;
        // 3. 对数据库进行迭代操作，使用for循环遍历db中的每个元素。
        for item in db.into_iter() {
            // 4. 在循环中，将当前元素赋值给变量i。
            let i = item?;
            // 5. 使用i.0.to_vec()将i的第一个元素转换成UTF-8编码的字符串，并将结果赋值给address变量。
            let address = String::from_utf8(i.0.to_vec())?;
            // 6. 使用i.1.to_vec()将i的第二个元素转换成字节向量，并使用bincode::deserialize函数将其反序列化为钱包对象，并将结果赋值给wallet变量。
            let wallet = bincode::deserialize(&i.1.to_vec())?;
            // 7. 将address和wallet插入到wlt的wallets HashMap中。
            wlt.wallets.insert(address, wallet);
        }
        // 8. 使用drop函数释放db的所有权，确保数据库资源被正确释放。
        drop(db);
        // 9. 返回一个包含wlt的Result对象，表示钱包对象的创建成功。
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
