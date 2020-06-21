# Substrate Node Template

A new FRAME-based Substrate node, ready for hacking.

## Build

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Initialize your Wasm Build environment:

```bash
./scripts/init.sh
```

Build Wasm and native code:

```bash
cargo build --release
```

## Run

### Single Node Development Chain

Purge any existing developer chain state:

```bash
./target/release/node-template purge-chain --dev
```

Start a development chain with:

```bash
./target/release/node-template --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a local testnet with two validator nodes for Alice and Bob, who are the initial authorities of the genesis chain that have been endowed with testnet units.

Optionally, give each node a name and expose them so they are listed on the Polkadot [telemetry site](https://telemetry.polkadot.io/#/Local%20Testnet).

You'll need two terminal windows open.

We'll start Alice's substrate node first on default TCP port 30333 with her chain database stored locally at `/tmp/alice`. The bootnode ID of her node is `QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR`, which is generated from the `--node-key` value that we specify below:

```bash
cargo run -- \
  --base-path /tmp/alice \
  --chain=local \
  --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator
```

In the second terminal, we'll start Bob's substrate node on a different TCP port of 30334, and with his chain database stored locally at `/tmp/bob`. We'll specify a value for the `--bootnodes` option that will connect his node to Alice's bootnode ID on TCP port 30333:

```bash
cargo run -- \
  --base-path /tmp/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR \
  --chain=local \
  --bob \
  --port 30334 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator
```

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can also replace the default command (`cargo build --release && ./target/release/node-template --dev --ws-external`) by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

## Advanced: Generate Your Own Substrate Node Template

A substrate node template is always based on a certain version of Substrate. You can inspect it by
opening [Cargo.toml](Cargo.toml) and see the template referred to a specific Substrate commit(
`rev` field), branch, or version.

You can generate your own Substrate node-template based on a particular Substrate
version/commit by running following commands:

```bash
# git clone from the main Substrate repo
git clone https://github.com/paritytech/substrate.git
cd substrate

# Switch to a particular branch or commit of the Substrate repo your node-template based on
git checkout <branch/tag/sha1>

# Run the helper script to generate a node template.
# This script compiles Substrate and takes a while to complete. It takes a relative file path
#   from the current dir. to output the compressed node template.
.maintain/node-template-release.sh ../node-template.tar.gz
```

Noted though you will likely get faster and more thorough support if you stick with the releases
provided in this repository.

## 设计赠与小猫transfer
### 1.修改存储结构
总体思路，使用链表结构实现某用户下小猫的增减，以达到O(n)复杂度
#### 1.1 首先定义KittyIndex
type KittyIndex: Parameter + Member + SimpleArithmetic + Bounded + Default + Copy;
#### 1.2 其次，定义
`pub struct KittyLinkedItem<T: Trait> {
	pub prev: Option<T::KittyIndex>,
	pub next: Option<T::KittyIndex>,
}`
#### 1.3 再次，修改部分原先存储变量
`pub Kitties get(kitty): map T::KittyIndex => Option<Kitty>;`
`pub KittiesCount get(kitties_count): T::KittyIndex;`
`pub OwnedKitties get(owned_kitties): map (T::AccountId, Option<T::KittyIndex>) => Option<KittyLinkedItem<T>>;`
#### 1.4 最后，删除不需要的存储变量
`pub OwnedKittiesCount get(fn owned_kitties_count): map hasher(blake2_128_concat) T::AccountId => u32;`

### 2.定义函数头
`pub fn transfer_kitty(origin, to: T::AccountId, kitty_id: T::KittyIndex)`

### 3.伪代码流程
#### 3.1 解析origin,得到sender
#### 3.2 获取kitty_id的owner
#### 3.3 验证KittiesCount是否达到最大值，若是，给错误提示并弹出
#### 3.4 验证sender是否与to相同，若相同，给错误提示并弹出
#### 3.5 在sender中，根据sender和kitty_id，remove链表结构中对应的小猫
#### 3.6 在to中，append链表结构中对应的小猫，给出小猫kitty_id
#### 3.7 KittiesCount总数+1

## 设计交易小猫ask,buy
### 1.修改存储结构
#### 1.1 首先，定义类型Currency和BalanceOf
`type Currency: Currency<Self::AccountId>;`
`type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;`
#### 1.2 然后，增加存储变量
`pub KittyOwners get(kitty_owner): map T::KittyIndex => Option<T::AccountId>;`
`pub KittyPrices get(kitty_price): map T::KittyIndex => Option<BalanceOf<T>>`

### 2.定义函数头ask和buy
`pub fn ask(origin, kitty_id: T::KittyIndex, setPrice: Option<BalanceOf<T>>)`
`pub fn buy(origin, kitty_id: T::KittyIndex, buyPrice: BalanceOf<T>)`

### 3.伪代码流程
#### 3.1 ask伪代码
##### 3.1.1 解析origin,得到sender
##### 3.1.2 获取kitty_id的owner
##### 3.1.3 验证sender是否为owner，若不是，给错误提示并弹出
##### 3.1.4 根据sender和kitty_id，设置对应小猫的Price
 
#### 3.2 buy伪代码
##### 3.2.1 解析origin,得到sender
##### 3.2.2 获取kitty_id的owner
##### 3.2.3 获取kitty_id的setPrice
##### 3.2.4 验证sender不等于owner，若相同，给错误提示并弹出
##### 3.2.5 验证buyPrice要大于等于setPrice，若小于，给错误提示并弹出
##### 3.2.6 根据owner,sender和kitty_id，调用transfer_kitty函数完成小猫转移
##### 3.2.7 发起交易，将buyPrice由sender转给owner
