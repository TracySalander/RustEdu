加入create方法，create方法需要一个random函数，我们在helper的部分加入
```rust
impl<T: Config> Pallet<T> {
		/// 随机数生成
		/// ### Arguments
		/// * `who` - 生成随机数的人
		fn random_value(who: &T::AccountId) -> [u8; 16] {
			let payload =
				(T::Randomness::random_seed(), &who, <frame_system::Pallet<T>>::extrinsic_index()); //extrinsic_index()是这笔交易在这个block中的index
			payload.using_encoded(blake2_128) //用上述三个值可以通过blake2_128最终产生一个128位数
		}
}

```
这里要用到Randomness需要在上面引入，并且在Config里暴露给外面，另外在runtime的Config里也需要添加
```rust
    use frame_support::{
	      dispatch::DispatchResult,
	      pallet_prelude::*,
	      traits::Randomness
    };
    use frame_system::pallet_prelude::*;
	  use sp_io::hashing::blake2_128;
```
```rust
    #[pallet::config]
	    pub trait Config: frame_system::Config {
		      // 事件 Because this pallet emits events, it depends on the runtime's definition of an event.
		    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	 		/// 随机数模块
 		    type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
    }
```
```rust
impl pallet_kitties::Config for Runtime {
	type Event = Event;
	type Randomness = RandomnessCollectiveFlip;
}
```
另外就是create方法里的Error和Event在对应位置要加入
```rust
    pub enum Error<T> {
		KittiesCountOverflow
    }
```
```rust
    pub enum Event<T: Config> {
		/// 创建成功 [account, kitty_id]
 		KittyCreated(T::AccountId, KittyIndex),
    }
```
同理我们加入transfer
```rust
/// 转让 Kitty
/// 转让者与接收者不能相同
/// ### Arguments
/// * `origin` - 转让者
/// * `to` - 接收者
/// * `kitty_id` - 转让的 Kitty 编号
#[pallet::weight(0)]
pub fn transfer(
	origin: OriginFor<T>,
	to: T::AccountId,
	kitty_id: KittyIndex,
) -> DispatchResult {
	let sender = ensure_signed(origin)?;

	ensure!(sender != to, Error::<T>::SameOwner);

	let owner = Owner::<T>::get(&kitty_id).unwrap();
	ensure!(owner == sender, Error::<T>::NotOwnerOfKitty);

	Owner::<T>::insert(kitty_id, Some(to.clone()));

	Self::deposit_event(Event::KittyTransfered(sender, to, kitty_id));
	Ok(())
}
```
Error跟Event都加上
```rust
pub enum Error<T> {
	KittiesCountOverflow,
	NotOwnerOfKitty,
	SameOwner,
}

pub enum Event<T: Config> {
	/// 创建成功 [account, kitty_id]
	KittyCreated(T::AccountId, KittyIndex),
	/// 转让成功 [who, receiver, kitty_id]
	KittyTransfered(T::AccountId, T::AccountId, KittyIndex)
}
```
下面就是今天全部内容
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
	 		/// 随机数模块
 		    type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
    }
    
    #[pallet::pallet]
	   #[pallet::generate_store(pub(super) trait Store)]
	   pub struct Pallet<T>(_);
    
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
		/// 创建成功 [account, kitty_id]
 		KittyCreated(T::AccountId, KittyIndex),
 		/// 转让成功 [who, receiver, kitty_id]
		KittyTransfered(T::AccountId, T::AccountId, KittyIndex)
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
		KittiesCountOverflow,
		NotOwnerOfKitty,
		SameOwner,
    }
    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
	/// 创建 Kitty
	/// 创建时需要质押一定的金额: `T::ReserveOfNewCreate`
	/// ### Arguments
	/// * `origin` - 创建者
	#[pallet::weight(0)]
	pub fn create(origin: OriginFor<T>) -> DispatchResult {
		let who = ensure_signed(origin)?;

		let kitty_id = match Self::kitties_count() {
			Some(id) => {
				ensure!(id != KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
				id
			},
			None => 1
		};
		let dna = Self::random_value(&who); //这个数据以后在前端可以做展示
		// 数据准备好之后下面我们就把数据都放在区块链上去
		Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));
		Owner::<T>::insert(kitty_id, Some(&who));
		KittiesCount::<T>::put(kitty_id);
		// 最后需要向前端抛出一个Event
		Self::deposit_event(Event::KittyCreated(who, kitty_id));

		Ok(())
	}

	/// 转让 Kitty
	/// 转让者与接收者不能相同
	/// ### Arguments
	/// * `origin` - 转让者
	/// * `to` - 接收者
	/// * `kitty_id` - 转让的 Kitty 编号
	#[pallet::weight(0)]
	pub fn transfer(
		origin: OriginFor<T>,
		to: T::AccountId,
		kitty_id: KittyIndex,
	) -> DispatchResult {
		let sender = ensure_signed(origin)?;

		ensure!(sender != to, Error::<T>::SameOwner);

		let owner = Owner::<T>::get(&kitty_id).unwrap();
		ensure!(owner == sender, Error::<T>::NotOwnerOfKitty);

		Owner::<T>::insert(kitty_id, Some(to.clone()));

		Self::deposit_event(Event::KittyTransfered(sender, to, kitty_id));
		Ok(())
	}
    }

    impl<T: Config> Pallet<T> {
	/// 随机数生成
	/// ### Arguments
	/// * `who` - 生成随机数的人
	fn random_value(who: &T::AccountId) -> [u8; 16] {
		let payload =
			(T::Randomness::random_seed(), &who, <frame_system::Pallet<T>>::extrinsic_index()); //extrinsic_index()是这笔交易在这个block中的index
		payload.using_encoded(blake2_128) //用上述三个值可以通过blake2_128最终产生一个128位数
	}
    }
}
	
```
