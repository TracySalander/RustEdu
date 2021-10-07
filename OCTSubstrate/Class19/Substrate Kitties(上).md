课程内容
* Metadata元数据介绍
* Kitties Pallet开发
* Frame资产相关模块
  * Balances
  * Assets

## Metadata (元数据，描述链上转移的概况，可以通过Polkadot.js API得到，在RPC的state的getMetadata方法)
其中包含了每个模块的元数据
Storage: 存放在链上的数据
Events: 异步系统，发了交易到链上要等打包然后交易，交易才算完成。需要监听Events方法得到交易的结果。
Calls: 一个交易。
Constants：一些常量，比如最小的账号能进行交易的数量
Errors: 错误

从元数据就可以知道runtime提供了哪些接口，哪些数据，对开发dapp非常重要
## Kitties Pallet
实现一个简单的Substrate Kitties
功能:
创建小猫
繁殖小猫
转移小猫（作业）
买卖小猫（作业）
显示小猫（作业）
