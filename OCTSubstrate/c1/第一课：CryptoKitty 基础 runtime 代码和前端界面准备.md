随机数：
我们生成随机数的原因是因为要创建很多Kitty，需要对Kitty做区分，每一个Kitty都有一个唯一的id
生成随机数的方法:
1. Generating a Randome Seed（有随机数相同的可能）
<system::Module<T>>::random_seed():对于同一块中的多个事务，随机种子不会改变
Substrate使用一种安全的混合算法，它使用前面块的熵为后面块生成新的随机数据。
2. Using a Nonce
let nonce = <Nonce<T>>::get(); （有随机数相同的可能）
3. Hashing Data（结合了第一种和第二种方法，然后再加上请求者的id，用这三个值生成一个hash，作为随机数）
```rust
let sender = ensure_signed(origin)?;
let nonce = <Nonce<T>>::get();
let random_seed = <system::Module<T>>::random_seed();
let random_hash = (random_seed, sender, nonce).using_encoded(<T as system::Trail>::Hashing::hash);
<Nonce<T>>::mutate(|n| *n += 1);
```
这之后我们需要检查这个id是否已经存在
```rust
ensure!(!<Kitties<T>>::exists(new_id)), "This new id already exists"); // 如果这个id存在了，那么我们要抛出一个message说id已经存在了
// 这个ensure是Substrate的一个宏，在support::ensure
```
  
下面是介绍
1. create unique id
2. create a new kitties storage: map id=>kitty
3. create a KittyOwner storage: map id=>AccoutId
4. function: create_kitty()
  
  
存储时候定义的变量:
KittiesOwned
