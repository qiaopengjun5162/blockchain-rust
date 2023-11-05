# blockchain-rust

## Explain code

### cli.rs

这段代码是一个简单的命令行接口（CLI）程序，用于操作一个区块链。

首先，代码导入了 `clap` 库，用于解析命令行参数。同时，还导入了 `Blockchain` 和 `Result` 类型，来自其他模块。

接下来，定义了一个名为 `Cli` 的结构体，其中包含一个 `Blockchain` 实例。

在 `Cli` 的实现中，定义了一个 `new` 方法，用于创建一个新的 `Cli` 实例。该方法首先创建了一个 `Blockchain` 实例，并将其保存在 `Cli` 的 `bc` 字段中。

接下来，定义了一个 `run` 方法，用于执行命令行接口的逻辑。在该方法中，首先使用 `Command` 创建了一个命令行应用程序，并设置了版本号、作者和描述信息。然后，添加了两个子命令： `printchain` 和 `addblock` 。 `printchain` 用于打印所有的区块链块， `addblock` 用于在区块链中添加一个新的块，并接受一个名为 `DATA` 的参数。

接下来，通过调用 `get_matches` 方法获取命令行参数的匹配结果。如果匹配到了 `addblock` 子命令，就调用 `addblock` 方法，并传入 `DATA` 参数的值。如果匹配到了 `printchain` 子命令，就调用 `print_chain` 方法。

最后，定义了 `addblock` 和 `print_chain` 两个私有方法，分别用于在区块链中添加块和打印区块链。

 `addblock` 方法接受一个 `data` 参数，将其传递给 `Blockchain` 实例的 `add_block` 方法。

 `print_chain` 方法使用 `bc.iter()` 方法获取一个可变引用的迭代器，然后遍历迭代器，对每个块调用 `println!` 宏打印出来。

整个程序的逻辑就是根据命令行参数执行相应的操作，例如添加块或打印区块链。

## Usage

```shell
blockchain-rust on  main [!?] is 📦 0.1.0 via 🦀 1.75.0 via 🅒 base 
➜ cargo run addblock "this is test block"
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
warning: the following packages contain code that will be rejected by a future version of Rust: rustc-serialize v0.3.24
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
     Running `target/debug/blockchain-rust addblock 'this is test block'`

blockchain-rust on  main [!?] is 📦 0.1.0 via 🦀 1.75.0 via 🅒 base 
➜ cargo run printchain                   
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
warning: the following packages contain code that will be rejected by a future version of Rust: rustc-serialize v0.3.24
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
     Running `target/debug/blockchain-rust printchain`
block: Block {
    timestamp: 1699195144571,
    transactions: "this is test block",
    prev_block_hash: "0000bce4f96c734866c76931d8187caa8f91949289a5c0fe0f45d8f4ea2e54cc",
    hash: "0000816c4d92e0a4d514c542a0e83984df26d7867dbf82ac9ac189e6227ecc83",
    height: 4,
    nonce: 14151,
}
block: Block {
    timestamp: 1699193491819,
    transactions: "data2",
    prev_block_hash: "0000bd73b4f47a3ed9c38cb23241b40237a21955b07f91c3233945cdd70d037e",
    hash: "0000bce4f96c734866c76931d8187caa8f91949289a5c0fe0f45d8f4ea2e54cc",
    height: 4,
    nonce: 46891,
}
block: Block {
    timestamp: 1699193491666,
    transactions: "data1",
    prev_block_hash: "0000c9d19b1633ab298df1faf0ad7ea53699bc23b676aee261afdba4b8084d23",
    hash: "0000bd73b4f47a3ed9c38cb23241b40237a21955b07f91c3233945cdd70d037e",
    height: 4,
    nonce: 16809,
}
block: Block {
    timestamp: 1699193491510,
    transactions: "data",
    prev_block_hash: "0000a2e7b0624afe8bcbf86ba3e2f32b390be6d81a4ff8014401b21471b2dad4",
    hash: "0000c9d19b1633ab298df1faf0ad7ea53699bc23b676aee261afdba4b8084d23",
    height: 4,
    nonce: 17177,
}
block: Block {
    timestamp: 1699193444438,
    transactions: "data2",
    prev_block_hash: "0000786aad3907e5c70d99844680f8db9fe4ee1c41ebbf5cdb6693056c072682",
    hash: "0000a2e7b0624afe8bcbf86ba3e2f32b390be6d81a4ff8014401b21471b2dad4",
    height: 4,
    nonce: 27046,
}
block: Block {
    timestamp: 1699193444070,
    transactions: "data1",
    prev_block_hash: "00009ca0c8b9507c237ed7c6e8b3709ea8bcbdf31f482ab7007f02fbc328da71",
    hash: "0000786aad3907e5c70d99844680f8db9fe4ee1c41ebbf5cdb6693056c072682",
    height: 4,
    nonce: 40838,
}
block: Block {
    timestamp: 1699193443907,
    transactions: "data",
    prev_block_hash: "00008799ba08627333150e1d65ac344e4dee7b590c812388a2c23db722b64fdc",
    hash: "00009ca0c8b9507c237ed7c6e8b3709ea8bcbdf31f482ab7007f02fbc328da71",
    height: 4,
    nonce: 18078,
}
block: Block {
    timestamp: 1699193443690,
    transactions: "Gensis Block",
    prev_block_hash: "",
    hash: "00008799ba08627333150e1d65ac344e4dee7b590c812388a2c23db722b64fdc",
    height: 0,
    nonce: 23668,
}
```
