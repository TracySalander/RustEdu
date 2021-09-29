内容
* 所有权概念和规则
* 所有权转移
* Copy & Clone
* 函数与所有权
* Reference和borrowing
* Slice类型

## 所有权的概念
任何程序的运行都需要依赖内存，典型的内存管理机制有：
* 垃圾收集器，如Java, GO
* 手动分配和释放，如C/C++
* 编译时的所有权系统（Ownership），只有Rust

Ownership是Rust区别于其他编程语言最核心的特性，它保证了代码的内存安全性，且性能卓越。
## 所有权的规则
* 任何值都有一个变量与之对应，称为owner
* 某一时刻，只能有一个owner
* 当owner退出作用域后，值被丢弃
```rust
{
    // s还未定义
    let s = String::from("hello"); // 定义s并分配存储
} // s作用域结束，释放存储
```
## 所有权的转移(move)
```rust
{
    let s1 = String::from("hello"); // 定义s1并分配存储
    let s2 = s1; // s1对应值的所有权转移至s2, s1失败
} // s2作用域结束，释放存储
```
如果是简单类型比如数值,bool,赋值会发生数据拷贝，而不是转移所有权。
## Coppy & Clone
赋值时可与通过数据拷贝/克隆，不去转移现有数据所有权，
* Copy，适用于基本类型或完全由基本类型组成的复杂类型
  * 如u32,bool,char,tuples
* Clone，数据存储在堆上，在堆上克隆一份新的
  * 如String,Hashmap,Vec
```rust
fn main(){
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 这里去掉clone()会发生异常，因为所有权已经转移
    
    println!("s1 = (), s2 = ()", s1, s2);
}
```
```rust
fn main(){
    let x = vec![1,2];
    let y = x;
    
    println!("y: {:?}", y);
}
```
* 自动派生Copy或Clone接口，#[derive(Copy, Clone)]
* 一般不需要显示实现Copy
* Clone更慢，clone()不可缺省

## 函数与所有权
* 和赋值类似，将值传递给函数也会转移所有权或copy
* 返回值可以把函数内变量对应值的所有权转移至函数外

## 函数所有权：参数
```rust
fn main(){
  let s = String::from("hello"); // 定义s并分配存储
  takes_ownership(s); // s对应值的所有权转移
  
  let x = 5;
  makes_copy(x);
}
fn takes_ownership(some_string: String){ // some_string获得所有权
  println!("{}", some_string);
} // some_string作用域结束，释放存储
```

## 函数与所有权：返回值
```rust
fn main(){
  let s1 = gives_ownership(); // 返回值的所有权从函数转移至s1
  
  let s2 = String::from("hello"); // 定义并初始化存储
  let s3 = takes_and_gives_back(s2); // s2对应值所有权转移至函数, s3获得返回值的所有权
}

fn give_ownership() -> String{
  let some_string = String::from("hello"); // 定义并分配存储
  some_string // 返回some_string并转移所有权至函数外
}

fn takes_and_gives_back(a_string: String) -> String{ // a_string获得所有权
  a_string // 返回a_string，所有权转移至函数外
}
```

## Reference 和 Borrowing
当需要使用某个值、但又不希望获取所有权时。可以通过引用，
* 在变量名前放置&符号，获取值的引用
* Borrowing：函数参数为引用
* 默认是不可变的（immutable），可变引用为&mut
* 引用的作用域，在最后使用的地方结束，而不是大括号的末尾。
```rust
fn main(){
  let s1 = String::from("hello"); // 定义并分配存储
  
  let len = calculate_length(&s1); // borrow变量对应的值
  
  println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // 参数s为引用类型
  s.len()
} // s退出作用域，不清空引用的值
```
```rust
fn main(){
  let mut s = String::from("hello"); // 定义可变变量并分配存储
  
  change(&mut s); // 创建可变引用并传递给函数
}

fn change(some_string: &mut String) { // 参数some_string为可变引用
  some_string.push_str(", world"); // 修改值
} // some_string退出作用域，不清空引用的值
```
## Slice类型
和引用类似，slice也不拥有值得所有权，用于引用集合内的部分连续数据，
* 与值绑定，当退出作用域，需要清空时，slice也同时失效
* 定义slice: &name[start..end]，不包含end
* 类型签名: &str为string slice，&[T]为Vector/array slice

## 定义slice
```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

## 内存安全性
所有权和引用如何保证内存安全性?
* 拥有所有权的变量退出作用域时，自动清空值得内存空间
* 同一时间，最多有一个可变引用，或者多个不可变引用
* 编译时不允许空指针
* 通过slice引用值的一部分
