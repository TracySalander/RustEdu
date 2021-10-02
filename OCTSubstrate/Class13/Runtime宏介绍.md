内容
* Rust宏
* Runtime常用的宏
* cargo expand
* 其它宏
## Rust宏
宏(Macro)是一种元编程的方式，常见还有Java里的反射，Rust提供了两种宏:
* 声明宏
* 过程宏
## Substrate为什么使用宏
为了简化Runtime的开发，Substrate使用宏建立了一套DSL(Domain Specific Language)，设计合理的SDL可以：
* 很好的被用户理解
* 代码更加简洁，提升效率
* 解放应用开发者，只需实现业务组件
## Runtime模块的组成
使用Substrate进行Runtime模块开发的过程中，常用到的宏有：
* frame_support::pallet 定义功能模块
* pallet::config 定义配置接口
* pallet::storage 存储单元
* pallet::event 事件
* pallet::error 错误信息
* pallet::call 包含可调用函数
* pallet::hooks 区块不同时期的执行逻辑
* construct_runtime 添加模块到Runtime
## storage宏
不管是Web2.0传统的互联网应用，还是采用区块链技术的web3.0应用，关键数据都需要存起来

storage宏，就是用来定义runtime模块的存储单元。
### storage宏举例
```rust
#[frame_support::pallet]
pub mod pallet{
    // -- snippet --
}

#[pallet::config]
pub trait Config: frame_system::Config {
}

#[pallet::storage]
#[pallet::getter(fn something)]
pub type Something<T> = StorageValue<_, u32>;
```
数据类型：
* 单值StorageValue
* 映射StorageMap
* 双键映射StorageDoubleMap

### call宏
区块链的链上状态变化由交易触发，Substrate不仅支持自定义的存储数据结构，还支持自定义的交易，例如转账、注册身份、投票等等，也叫做extrinsic外部交易。
call用来定义模块里可调用函数，每一个外部交易都会触发一个可调用函数，并根据交易体信息也就是函数参数，更新链上状态。
```rust
#[pallet::call]
impl<T: Config> Pallet<T>{
    #[]
}
```
### call宏举例
```rust
#[pallet::call]
impl<T: Config> Pallet<T>{
    #[pallet::weight(10_000)] // 区块的执行事件，整个权重最终会转换为交易费用
    pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo{
        let who = ensure_signed(origin)?;
        
        // Update storage.
        <Something<T>>::put(something);
        
        // Emit an event.
        Self::deposit_event(Event::SomethingStored(something, who));
        
        Ok(().into())
    }
}
```
```rust
#[pallet::call]
impl<T: Config> Pallet<T>{
    #[pallet::weight(10_000)]
    pub fn cause_error(origin: OriginFor<T>) -> DispatchResultWithPostInfo{
        let _who = ensure_signed(origin)?;
        
        match <Something<T>>::get(){
            None => Err(Error::<T>::NoneValue)?,
            Some(old) => {
                let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                <Something<T>>::put(new);
                Ok(().into())
            }
        }
    }
}
```
## event宏
区块链是一个异步系统，runtime通过触发事件通知交易执行结果。
```rust
#[pallet::event]
#[pallet::metadata(T::AccountId = "AccountId")]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    SomethingStored(u32, T::AccountId),
}

// -- snippet --
Self::deposit_event(Event::SomethingStored(Something, who))
```
## error宏
当可调用函数执行过程发生错误时，通过error信息通知客户端。
```rust
#[pallet::error]
pub enum Error<T>{
    /// Error names should be descriptive.
    NoneValue,
    /// Errors should have helpful documentation associated with them.
    StorageOverFlow,
}
```
可调用函数里的错误类型，
* 不能给它们添加数据;
* 通过metadata暴露给客户端;
* 错误发生时触发system.ExtrinsicFailed事件，包含了对应错误的索引信息。

## hooks宏
Runtime模块里存在保留函数，用于执行特定的逻辑：
* on_initialize，在每个区块的开头执行;
* on_finalize，在每个区块结束时执行;
* offline_worker：开头且是链外执行，不占用链上的资源;
* on_runtime_upgrade：当有runtime升级时才会执行，用来迁移数据。
