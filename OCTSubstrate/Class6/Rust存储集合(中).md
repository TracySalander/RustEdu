## String, str
### 什么是字符串？
在开始深入这些方面之前，我们需要讨论一下术语 __字符串__ 的具体意义。Rust 的核心语言中只有一种字符串类型：str，字符串 slice，它通常以被借用的形式出现，&str。 __字符串 slice：__ 它们是一些储存在别处的 UTF-8 编码字符串数据的引用。比如字符串字面值被储存在程序的二进制输出中，字符串 slice 也是如此。

称作 String 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。当 Rustacean 们谈到 Rust 的 “字符串”时，它们通常指的是 String 和字符串 slice &str 类型，而不仅仅是其中之一。虽然本部分内容大多是关于 String 的，不过这两个类型在 Rust 标准库中都被广泛使用，String 和字符串 slice 都是 UTF-8 编码的。
### 新建字符串
很多 Vec 可用的操作在 String 中同样可用，从以 new 函数创建字符串开始。
```rust
let mut s = String::new();
```
这新建了一个叫做 s 的空的字符串，接着我们可以向其中装载数据。通常字符串会有初始数据，因为我们希望一开始就有这个字符串。为此，可以使用 to_string 方法，它能用于任何实现了 Display trait 的类型，字符串字面值也实现了它。
```rust
let data = "initial contents";

let s = data.to_string();

// 该方法也可直接用于字符串字面值：
let s = "initial contents".to_string();
```
## Set
## Struct
