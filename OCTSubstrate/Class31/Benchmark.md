https://docs.substrate.io/tutorials/v3/private-network/

## Benchmark确定交易权重
总费用 = （字节费用+权重费用）* 动态调节费率 + 小费

权重被用来定义交易产生的计算复杂度：
* 合理的权重值需要通过Benchmark来获取
* 通过WeightToFee，转换权重值为权重费用
* 通过Pays::No来取消交易费用
* 权重纠正，可调用函数返回实际权重值
介绍
https://github.com/kaichaosun/play-substrate/tree/master/pallets/benchmark-demo
之后看runtime中最后加入benchmark
https://github.com/kaichaosun/play-substrate/blob/master/runtime/src/lib.rs

## Benchmark可调用函数
编译和运行
cargo build --features runtime-benchmarks --release
./target/release/node-template benchmark \
   --chain dev
   --execution=wasm \
   --wasm-execution=compiled \
   --pallet benchmark-demo \
   --extrinsic do_something \
   --steps 20 \
   --repeat 50
   
## 切换PoA为Pos
* Substrate node
删除不需要的模块
添加自己的模块
* node template
Aura->Babe
添加staking相关模块
添加治理模块

## 切换PoA为PoS: BABE
* 出块节点随机
* 同时存在次级出块节点
* 当长时间不出块，会导致网络瘫痪

## 切换PoA为PoS: Staking
关联的模块:
* staking, session
* authorship
* offences, grandpa, im-online
* utility

## 切换PoA为PoS：治理
关联的模块:
* treasury
* collective
* membership, elections-phragmen
* democracy, scheduler

## Runtime配置参数
常见的配置项为:
* Runtime常用的类型别名如BlockNumber, Balances...
* 区块生成时间
* WeightToFee
* 初始区块配置

检查每个模块所用的配置是不是符合业务需求

## Chain Specification
Chain Spec文件包含：
* 元信息如name, id, chainType
* 启动节点bootNodes
* telemetryEndpoints
* protocolld
* properties(tokenSymbol, tokenDecimals)
* genesis信息

如何生成Chain Spec
* 修改chain_spec.rs, command.rs
* 添加初始区块账户: subkey generate
* 添加验证人账户和Session Keys
  * for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j";
    done; done
  * for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j";
    done; done
  * for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j";
    done; done
* 生成Chain Spec
  ./target/release/node-template build-spec --chain tao-staging > tao-staging.json
* 编码Chain Spec
  ./target/release/node-template build-spec --chain=tap-staging.json --raw > tao-staging-raw.json
  
启动bootnode:
./target/release/node-template \
  --node-key c12b623452464t435ytrgw5462 \
  --base-path /tmp/bootnode1 \
  --chain tao-staging-raw.json \
  --name bootnode1
  
启动验证人:
./target/release/node-template\
  --base-path /tmp/validator1 \
  --chain tao-staging-raw.json \
  --bootnodes
/ip4/your-ip/tcp/30333/p2p/12413523413tfwg5w45
Box2 \
  --name validator1 \
  --validator
  
也可将bootnode的配置信息添加到Chain Spec中

验证人启动之后，添加Chain Spec配置的对应验证节点的BABE和GRANDPA使用的key,
curl http://localhost::9933 -H "Content-Type:application/json;charset=utf-8" -d "@babe1"
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_insertKey",
  "params":[
    "babe",
    "own word vocal dog decline set bitter example forget excite gesture water//1//babe",
    "0x123rh3b4kh3245k2jh3452345cf234v52352fdvgsdfhertn"
  ]
}

## 课后作业
为template模块的do_something添加benchmark用例（也可以是其他自选模块的可调用函数），并且将benchmark运行的结果转换为对应的权重定义，选择node-template或者其它节点程序，生成Chain Spec文件（两种格式都需要）。
