# Homework for lesson 12

答案放在README文件里即可。

## 1. 为 template 模块的 do_something 添加 benchmark 用例，并且将 benchmark 运行的结果转换为对应的权重定义；  
### 1.1 benchmark用例  
```
	benchmarks!{
		_ {
			let b in 1 .. 1000 => ();
		}

		do_something {
			let b in ...;
			let caller = account("caller", 0, 0);
		}: _ (RawOrigin::Signed(caller), b.into())
		verify {
			let value = Something::get();
			assert_eq!(value, b.into());
		}
	}
```  
### 1.2 benchmark运行结果的命令行截图  
```  
root@7590:/home/team6/lesson12# make benchmarks
cd node;cargo build --release --features runtime-benchmarks
    Finished release [optimized] target(s) in 0.25s
target/release/node-template benchmark \
	--chain=dev \
	--execution=wasm \
	--wasm-execution=compiled \
	--pallet=pallet-template \
	--extrinsic=do_something \
	--steps=20 \
	--repeat=51
Pallet: "pallet-template", Extrinsic: "do_something", Lowest values: [], Highest values: [], Steps: [20], Repeat: 51
Median Slopes Analysis
========

Model:
Time ~=    14.98
    + b        0
              µs

Min Squares Analysis
========

Data points distribution:
    b   mean µs  sigma µs       %
    1     15.16     0.045    0.2%
   50     15.13     0.032    0.2%
   99     15.11     0.033    0.2%
  148     15.09     0.046    0.3%
  197     15.13     0.032    0.2%
  246     15.11     0.039    0.2%
  295      15.1     0.043    0.2%
  344      15.1     0.039    0.2%
  393     15.11     0.049    0.3%
  442     15.15      0.04    0.2%
  491     15.11     0.032    0.2%
  540     15.12      0.04    0.2%
  589     15.58     0.107    0.6%
  638     15.33     0.124    0.8%
  687     15.81     0.061    0.3%
  736     15.54     0.102    0.6%
  785     15.84      0.04    0.2%
  834     15.73     0.111    0.7%
  883     15.74     0.106    0.6%
  932     15.77     0.094    0.5%
  981     15.73     0.144    0.9%

Quality and confidence:
param     error
b             0

Model:
Time ~=    14.94
    + b    0.001
              µs
```  

### 1.3 最终的可调用函数代码片段（包含权重设置）
```
/// # <weight>
/// - Base Weight: 14.94 µs
/// - DB Weight: 1 Write
/// # </weight>
#[weight = T::DbWeight::get().writes(1) + 15*1_000_000]
pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
```


## 2. 选择 node-template 或者其它节点程序，生成 Chain Spec 文件（两种格式都需要）；  

### 2.1 两组密钥对生成
```  
root@7590:/home/team6/lesson12# subkey --sr25519 generate
Secret phrase `trap away divorce belt dentist slim avoid decrease void base economy secret` is account:
  Secret seed:      0x2900e06c42596c1cbdfbdc48d79449357ae33828ddd9a5e74c3c18cb812d4deb
  Public key (hex): 0x4403d8498e00a087e30f6b1fd7202de662253e8baa6360437dfeb64bfa3dfd56
  Account ID:       0x4403d8498e00a087e30f6b1fd7202de662253e8baa6360437dfeb64bfa3dfd56
  SS58 Address:     5DbtFmi2upCSEaqbZgXCAJakHXze6ogtDZKzE1b5KCu5BN4g

root@7590:/home/team6/lesson12# subkey --ed25519 inspect "trap away divorce belt dentist slim avoid decrease void base economy secret"
Secret phrase `trap away divorce belt dentist slim avoid decrease void base economy secret` is account:
  Secret seed:      0x2900e06c42596c1cbdfbdc48d79449357ae33828ddd9a5e74c3c18cb812d4deb
  Public key (hex): 0x6e8cde8f7b29f54f8dae6f917c4edf722799b9b601a14dba10a239620782c021
  Account ID:       0x6e8cde8f7b29f54f8dae6f917c4edf722799b9b601a14dba10a239620782c021
  SS58 Address:     5EZeyJ85Z3mmW1uNxiEzQZs9cPWmqPJpFQAajdfTroGjyGN6


root@7590:/home/team6/lesson12# subkey --sr25519 generate
Secret phrase `flavor fly rely inhale remain cup improve bargain captain kind bundle toddler` is account:
  Secret seed:      0x61dbdc1dfccb39f1845858212e3f0f5cce7f8266ba7616d1f48496e0e811dae1
  Public key (hex): 0x8e5f6586fb3e5539caf2f7e9d641127ed62dd8bafbbdf3fbd22081dfa4774200
  Account ID:       0x8e5f6586fb3e5539caf2f7e9d641127ed62dd8bafbbdf3fbd22081dfa4774200
  SS58 Address:     5FHNzh6jAuYn4tMtR9Udwvvq6MsCz1S1R3VCV5cqasBQuUsE

root@7590:/home/team6/lesson12# subkey --ed25519 inspect "flavor fly rely inhale remain cup improve bargain captain kind bundle toddler"
Secret phrase `flavor fly rely inhale remain cup improve bargain captain kind bundle toddler` is account:
  Secret seed:      0x61dbdc1dfccb39f1845858212e3f0f5cce7f8266ba7616d1f48496e0e811dae1
  Public key (hex): 0xb185bc8568267b0cf22206fe157c583546b1163c59adfbc47d095b7860ef74e7
  Account ID:       0xb185bc8568267b0cf22206fe157c583546b1163c59adfbc47d095b7860ef74e7
  SS58 Address:     5G5U4UfhtDmGtLwbmLdUJ4RhTV9Kie3ezZS4sFMtzSJfMrzZ
```  
### 2.2 修改customSpec配置文件，生成customSpecRaw
```  
root@7590:/home/team6/lesson12# ./target/release/node-template build-spec --disable-default-bootnode --chain local > customSpec.json

      "aura": {
        "authorities": [ "5DbtFmi2upCSEaqbZgXCAJakHXze6ogtDZKzE1b5KCu5BN4g",
          "5FHNzh6jAuYn4tMtR9Udwvvq6MsCz1S1R3VCV5cqasBQuUsE" ] },
      "grandpa": {
        "authorities": [ [ "5EZeyJ85Z3mmW1uNxiEzQZs9cPWmqPJpFQAajdfTroGjyGN6", 1 ],
          [ "5G5U4UfhtDmGtLwbmLdUJ4RhTV9Kie3ezZS4sFMtzSJfMrzZ", 1 ] ] },

root@7590:/home/team6/lesson12# ./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
```  
## 3.（附加题）根据 Chain Spec，部署公开测试网络。

Note: 上传 telemetry.polkadot.io 上你的网络节点的截图，或者apps上staking页面截图。

### 3.1 清空目录内所有内容
```
target/release/node-template purge-chain --chain customSpecRaw.json -y -d target/node01
target/release/node-template purge-chain --chain customSpecRaw.json -y -d target/node02  
```  
### 3.2 第一位参与者启动Bootnode,并使用curl插入密钥库
```  
./target/release/node-template \
  --base-path target/node01 \
  --chain=./customSpecRaw.json \
  --port 30333 \
  --ws-port 9944 \
  --rpc-port 9933 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator \
  --rpc-methods=Unsafe \
  --name GWNode01


curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@node01-aura.json"
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@node01-gran.json"

curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_localPeerId", "params":[]}' http://localhost:9933
```  

### 3.3 第二位参与者启动Bootnode,并使用curl插入密钥库
```  
./target/release/node-template \
  --base-path target/node02 \
  --chain=customSpecRaw.json \
  --port 30334 \
  --ws-port 9945 \
  --rpc-port 9934 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator \
  --rpc-methods=Unsafe \
  --name GWNode02 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWKx92FWZ6o25gCN4uuZyP94pcfJAJQd5VHr6R8oxjcrKr


curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d "@node02-aura.json"
curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d "@node02-gran.json"  
```  
### 3.4 命令行终端节点运行图
```  

![x](runnode.png)

```  
### 3.5 telemetry节点运行图
```  

![x](telemetry.png)

```  

