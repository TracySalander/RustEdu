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
