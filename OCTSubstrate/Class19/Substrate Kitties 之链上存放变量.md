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

By default, you'll notice that the instance of our modified template pallet name remains TemplateModule. Change it  (in runtime/src/lib.rs):

```rust
construct_runtime!(
    pub enum Runtime where
    Block = Block,
    NodeBlock = opaque::Block,
    UncheckedExtrinsic = UncheckedExtrinsic
    {
        // --snip
        ..::{Pallet, Call, Storage, Event<T>},
    }
);
```
```rust
#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	   use frame_support::{
		      dispatch::DispatchResult,
		      pallet_prelude::*,
		      traits::Randomness
	   };
	   use frame_system::pallet_prelude::*;
	   use sp_io::hashing::blake2_128;
	   
    #[pallet::config]
	   pub trait Config: frame_system::Config {
		      // 事件 Because this pallet emits events, it depends on the runtime's definition of an event.
		      type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }
    
    #[pallet::pallet]
	   #[pallet::generate_store(pub(super) trait Store)]
	   pub struct Pallet<T>(_);
    
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
    }
    
    #[pallet::error]
    pub enum Error<T> {
    }
    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
    }

    impl<T: Config> Pallet<T> {
    }
}
```
加入kitty struct
```rust
// Kitty 的状态
#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);  
```
加入kitty index
```rust
type KittyIndex = u32;
```
加入KittiesCount
```rust
	/// Kitties 总数
	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]//自动给一个方法，通过这个方法可以取到当前的值
	pub type KittiesCount<T: Config> = StorageValue<_, u32>;
```
加入存储Kitties的map
```rust
	/// Kitties
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyIndex, Option<Kitty>, ValueQuery>; // Blake2_128Concar是映射时候的Hash方法
```
加入Kitties的主人
```rust
/// Kitties 的主人
	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyIndex, Option<T::AccountId>, ValueQuery>;
```
最后结果如下
```rust
#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
	      dispatch::DispatchResult,
	      pallet_prelude::*,
	      traits::Randomness
    };
    use frame_system::pallet_prelude::*;
	use sp_io::hashing::blake2_128;

    // Kitty 的状态
   	#[derive(Encode, Decode)]
 	pub struct Kitty(pub [u8; 16]);   

	type KittyIndex = u32;

    #[pallet::config]
	   pub trait Config: frame_system::Config {
		      // 事件 Because this pallet emits events, it depends on the runtime's definition of an event.
		      type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }
    
    #[pallet::pallet]
	   #[pallet::generate_store(pub(super) trait Store)]
	   pub struct Pallet<T>(_);
    
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
    }

	/// Kitties 总数
	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	pub type KittiesCount<T: Config> = StorageValue<_, u32>;

	/// Kitties
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyIndex, Option<Kitty>, ValueQuery>;
    
	/// Kitties 的主人
	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyIndex, Option<T::AccountId>, ValueQuery>;

    #[pallet::error]
    pub enum Error<T> {
    }
    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
    }

    impl<T: Config> Pallet<T> {
    }
}
	
```
