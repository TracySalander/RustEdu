# 课程内容
* 链上升级和数据迁移
* Pallet功能复用
* Sudo pallet介绍
* UI开发
* membership和collective介绍
## 链上升级
1. 为什么substrate能升级
Substrate把runtime都编译成WASM，并保存在链上
Client读取WASM的代码，通过WASM Executor来进行状态转变
当新的WASM代码设置到链上之后，新的runtime逻辑就生效了

2. 升级的过程
升级spec版本号
编译新的WASM，WASM_TARGTET_DIRECTORY=$(pwd)
通过Sudo或者链上治理来更新WASM
```rust
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("node-template"),
	impl_name: create_runtime_str!("node-template"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 100, // 系统认这个号来读取数据库中的代码
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
};
```

3. substrate对迁移的支持
on_runtime_upgrade
pre_upgrade
encode/decode
```rust
#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight {
			if StorageVersion::<T>::get() == Releases::V6_0_0 {
				migrations::v7::migrate::<T>()
			} else {
				T::DbWeight::get().reads(1)
			}
		}

		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<(), &'static str> {
			if StorageVersion::<T>::get() == Releases::V6_0_0 {
				migrations::v7::pre_migrate::<T>()
			} else {
				Ok(())
			}
		}
}
```
```rust
pub mod v7 {
	use super::*;

	pub fn pre_migrate<T: Config>() -> Result<(), &'static str> {
		assert!(CounterForValidators::<T>::get().is_zero(), "CounterForValidators already set.");
		assert!(CounterForNominators::<T>::get().is_zero(), "CounterForNominators already set.");
		assert!(StorageVersion::<T>::get() == Releases::V6_0_0);
		Ok(())
	}

	pub fn migrate<T: Config>() -> Weight {
		log!(info, "Migrating staking to Releases::V7_0_0");
		let validator_count = Validators::<T>::iter().count() as u32;
		let nominator_count = Nominators::<T>::iter().count() as u32;

		CounterForValidators::<T>::put(validator_count);
		CounterForNominators::<T>::put(nominator_count);

		StorageVersion::<T>::put(Releases::V7_0_0);
		log!(info, "Completed staking migration to Releases::V7_0_0");

		T::DbWeight::get().reads_writes(validator_count.saturating_add(nominator_count).into(), 2)
	}
}
```
## Pallet功能复用
应用基本的软件设计的最佳实践
1. 模块之间做到尽量的接触耦合
2. 面向接口的编程
3. 模块可以很好的被复用
substrate和substrate里面的pallet对复用有着更高的要求
1. substrate is platform of platform
2. pallet应该是可以任意组合的
