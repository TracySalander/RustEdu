内容
* 泛型的介绍
* trait使用
* 生命周期

## 为什么使用泛型
* 减少相似代码
* 通过抽象，增加扩展性
* 常用于结构体，枚举和函数签名

## 泛型-结构体
```rust
  struct Point<T>{
      x: T,
      y: T,
  }
  
  impl<T> Point<T>{
    fn x(&self) -> &T{
      &self.x
    }
  }
  
  fn main(){
    let integer = Point{x: 5, y: 10};
    let float = Point{x: 1.0, y: 4.0};
  }
```
## 泛型-枚举
```rust
enum Option<T>{
  Some(T),
  None,
}

enum Result<T, E>{
  Ok(T),
  Err(E),
}
```
## 泛型-函数签名
```rust
fn largest<T>(list: &[T]) -> &T{
    let mut largest = &list[0];
    
    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}
```
## 使用泛型的注意点
* 编译时使用具体类型替代，不影响执行效率
* 过多的泛型，可读性降低
* trait抽象了某种功能或者行为
```rust
pub trait Summary{
    fn summarize(&self) -> String;
}

pub trait Summary{
    fn summarize(&self) -> String{
        format!("read more...")
    }
}
```
## Traits
* 对泛型添加trait bound表示泛型参数满足某种约束
```rust
struct Tweet{
    author: String,
    text: String,
}

impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("{},{}". self.author, self.text)
    }
}

pub fn notify<T: Summary>(item: &T){
    println!("{}", item.summarize());
}

// 简单情形时
pub fn notify(item: &impl Summary){
    println!("{}", item.summarize());
}
```
## Traits
* 多个参数时，trait bound保证类型一致
```rust
pub fn notify<T: Summary>(item1: &T, item2: &T){
    println!("{} {}", item1.summarize(), item2.summarize());
}
```
* 多个类型约束时，使用+
```rust
pub fn notify<T:Summary + Display>(item: &T){
    println!("{}", item.summarize());
}
```
* where关键字
```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32
{}

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```
## 变量的生命周期
* 每个变量都有声明周期(lifetime)
* 生命周期确保引用的有效性，防止出现空指针

## 引用的生命周期
* 多数情况，可由编译器推断出来
* 推断不出时，使用泛型指定多个引用之间生命周期的关系
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    } else {
        y
    }
}
```
* 返回值的引用生命周期必须来自函数
* 如果来自函数内，会造成空指针，编译不通过
```rust
fn longest<'a>(x: str, y: str) -> &a' str{
    let result = String::from("really long string");
    result.as_str()
}
```
## 引用的生命周期：缺省规则
* 为每一个引用参数类型，添加生命周期泛型
    * fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
* 生命周期泛型只有一个时，所有引用类型的返回值使用此生命周期
    * fn foo<'a>(x: &'a i32) -> &'a i32
* 生命周期泛型有多个时，且其中一个为&self或者&mut self，所有引用类型的返回值使用它对应的生命周期
    * fn foo<'a, 'b>(&'a self, x: &'b i32) -> &'a i32

## 结构体的生命周期
* 此类结构体的作用域不能在引用的值之外
```rust
struct Summary<'a>{
    part: &'a str
}

fn main(){
    let text = String::from("Call me Ishmael. Some years ago...")
    let first_sentence = text.split('.').next().expect("Could not find a '.");
    let summary = Summary{
        part: first_sentence,    
    };
}
```

## 静态生命周期（static）
* 此类引用的有效性是程序的整个执行周期
* string literal默认static
    * let s: &'static str = "I have a static lifetime.";
* 谨慎使用static.修复代码可能存在的空指针或者引用生命周期不匹配
