内容
* 区块链存储的不同点和约束
* Substrate存储单元的类型
* 存储的初始化
* 最佳实践
## 区块链存储的不同点
### 区块链应用通常几个特点，
* 开源可审查，对等节点，引入延迟和随机来达到共识
* 链式、增量地存储数据
区块链应用的节点软件依赖搞笑的键值对数据库：
* LevelDB
* RocksDB
## 区块链存储的约束
### 区块链作为业务的载体，存储相关的限制有：
* 大文件直接存储在链上的成本很高；
* 链式的区块存储结构不利于对历史数据的索引；
* 另外一个约束是，在进行数据运算时不能使用浮点数。
## Substrate去存单元的类型
### 开发链上存储单元的特点：
* Rust原生数据类型的子集，定义在核心库和alloc库中
* 原生类型构成的映射类型
* 满足一定的编解码条件
回顾storage宏
```rust
#[pallet::storage]
#[pallet::getter(fn something)]
pub type Something<T> = StorageValue<_, u32>;
```
## 单值类型
存储某种单一类型的值，如布尔，数值，枚举，结构体等；
* 数值: u8, i8, u32, i32, u64, i64, u128, i128
* 大整数: U128, U256, U512
* 布尔: bool
* 集合：Vec<T>, BTreeMap, BTreeSet
* 定点小数：Percent, Permill, Perbill, FixedU128
* 定长哈希：H128, H256, H512
* 其它复杂类型：Option<T>, tuple, enum, struct
* 内置自定义类型：Moment, AccountId

### 数值类型u8的定义:
```rust
#[pallet::storage]
#[pallet::getter(fn my_unsigned_number)]
pub type MyUnsignedNumber<T> = StorageValue<_, u8>; // 定义了一个数据类型MyUnsignedNumber，数据类型是StorageValue

#[pallet::storage]
#[pallet::getter(fn my_signed_number)]
pub type MySignedNumber<T> = StorageValue<_, i8, ValueQuery>; // 用ValueQuery读取存储，不指定的话默认是OptionQuery，没有值的话返回Option，也就是None值。如果是ValueQuery的话，没有值返回就是i8的默认值0

```
#### type别名
你可以使用 type 关键字声明另一类型的别名：
```rust
type Name = String;
```
然后，你可以就像使用一个真正的类型一样使用这种类型：
```rust
type Name = String;

let x: Name = "Hello".to_string();
```
但是请注意，这是一个别名，不完全是一个新类型。换句话说，因为 Rust 是强类型的，所以你不能比较两个不同类型：
```rust
let x: i32 = 5;
let y: i64 = 5;

if x == y {
   // ...
}
```
这会产生这样的结果：
```rust
error: mismatched types:
 expected `i32`,
found `i64`
(expected i32,
found i64) [E0308]
 if x == y {
 ^
```
但是，如果我们有一个别名：
```rust
type Num = i32;

let x: i32 = 5;
let y: Num = 5;

if x == y {
   // ...
}
```
这个编译没有错误。无论如何，Num 类型的值和 i32 类型的值是相同的。
你还可以使用泛型类型别名：
```rust
use std::result;

enum ConcreteError {
Foo,
Bar,
}

type Result<T> = result::Result<T, ConcreteError>;
```
这将创建一个 Result 类型的专门的版本 ，它总是有一个针对 Result< T E > 的 E 部分的 ConcreteError 。这常被用在标准库来为每一部分创建自定义错误。例如，io::Result 。

### 数值类型 u8, i8, u32, i32, u64, i64, u128, i128的使用：
* 增：MyUnsignedNumber::<T>::put(number);
* 查：MyUnsignedNumber::<T>::get();
* 改：MyUnsignedNumber::<T>::mutate(|v| v+1);
* 删：MyUnsignedNumber::<T>::kill();

### 数值类型u8,i8,u32,i32,u64,i64,u128,i128的安全操作：
* 返回Result类型：checked_add，checked_sub，checked_mul，checked_div
  //fail the transaction if error
  my_unsigned_num.checked_add(10)?;
* 溢出返回饱和值: saturating_add, saturating_sub, saturating_mul
  // result is 255 for u8
  my_unsigned_num.saturating_add(10000);

### 大整数U256, U512类型定义:
```rust
use sp_core::U256;

#[pallet::storage]
#[pallet::getter(fn my_big_integer)]
pub type MyBigInteger<T> = StorageValue<_, U256>;
```
操作:checked_add, overflowing_mul...

### bool 类型定义
```rust
#[pallet::storage]
#[pallet::getter(fn my_bool)]
pub type MyBool<T> = StorageValue<_, bool>;
```
- if else逻辑判断
- 对于ValueQuery，默认值为false
  
### Vec<T>类型定义：
```rust
use sp_std::prelude::*;

#[pallet::storage]
#[pallet::getter(fn my_string)]
pub type MyString<T> = StorageValue<_, Vec<u8>>;
```
- 操作：push, pop, iter...
- 对于ValueQuery，默认值为0x00
  
### Percent, Permill, Perbill类型定义:
```rust
use sp_runtime::Permill;

#[pallet::storage]
#[pallet::getter(fn my_permill)]
pub type MyPermill<T> = StorageValue<_, Permill>;
```
### Percent, Permill, Perbill类型操作:
* 构造
  * Permill::from_percent(value);
  * Permill::from_parts(value); // 100万分之value
  * Permill::From_rational(p, q);
* 计算
  * permill_one.saturating_mul(permill_two);
  * my_permill * 20000 as u32

### Moment时间类型定义：
```rust
#[pallet::config]
pub trait Config: pallet_timestamp::Config + frame_system::Config {
    // -- snippet --
}

#[pallet::storage]
#[pallet::getter(fn my_time)]
pub type MyTime<T: Config> = StorageValue<_, T::Moment>;
```
- Moment是u64的类型别名
— 获取链上时间: pallet_timestamp::Pallet::<T>::get()

### AccountId账户类型定义:
```rust
#[pallet::storage]
#[pallet::getter(fn my_account_id)]
pub type MyAccountId<T: Config> = StorageValue<_, T::AccountId>;
```
- 定义在frame_system中，通常是Public key
- 获取AccountId: let sender = ensure_signed(origin)?
  
### struct类型定义:
```rust
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct People {
    name: Vec<u8>,
    age: u8,
}
  
#[pallet::storage]
#[pallet::getter(fn my_struct)]
pub type MyStruct<T: Config> = StorageValue<_, People>;
```
  
enum类型与struct类型类似，只不过需要手动的实现Default接口，无法通过derive这种形式生成出来。
