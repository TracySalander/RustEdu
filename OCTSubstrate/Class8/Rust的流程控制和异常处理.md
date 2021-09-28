## 流程控制
1.分支
2.循环

### 分支
```rust
if a > 0 {
    println!("a is positive");
} else if a < 0 {
    println!("a is negative");
} else {
    println!("a is zero");
}

match a {
    0 => println!("a is zero"),
    1..=i32::MAX => println!("a is possible"),
    _ => println!("a is negative"),
}
```

### 循环
```rust
loop {
    println!("a is zero");
    break;
}

while a > 0 {
    println!("a is not zero");
    a = a - 1;
}

for number in 1..5 {
    println!("number is {}", number);
}
```

## 异常处理
### Option
```rust
pub enum Option<T> {
    // No value
    None,
    Some(T),
}
```
如果要模式匹配则需要把Option中的内容取出来

### Result
T和E代表程序返回正确和错误的返回类型
```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### panic
主动退出程序
```rust
panic = 'abort'

use std::panic;
fn main() {
    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
}
```
