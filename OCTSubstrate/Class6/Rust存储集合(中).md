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
这些代码会创建包含 initial contents 的字符串

也可以使用 String::from 函数来从字符串字面值创建 String。
```rust
let s = String::from("initial contents");
```
### 更新字符串
String 的大小可以增加，其内容也可以改变，就像可以放入更多数据来改变 Vec 的内容一样。另外，可以方便的使用 + 运算符或 format! 宏来拼接 String 值。

#### 使用 push_str 和 push 附加字符串
可以通过 push_str 方法来附加字符串 slice，从而使 String 变长
```rust
let mut s = String::from("foo");
s.push_str("bar");
```

执行这两行代码之后，s 将会包含 foobar。push_str 方法采用字符串 slice，因为我们并不需要获取参数的所有权。
```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);
```
如果 push_str 方法获取了 s2 的所有权，就不能在最后一行打印出其值了。好在代码如我们期望那样工作！

push 方法被定义为获取一个单独的字符作为参数，并附加到 String 中。
```rust
let mut s = String::from("lo");
s.push('l');
```
执行这些代码之后，s 将会包含 “lol”。

#### 使用 + 运算符或 format! 宏拼接字符串
通常你会希望将两个已知的字符串合并在一起。一种办法是像这样使用 + 运算符
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
```
执行完这些代码之后，字符串 s3 将会包含 Hello, world!。s1 在相加后不再有效的原因，和使用 s2 的引用的原因，与使用 + 运算符时调用的函数签名有关。+ 运算符使用了 add 函数，这个函数签名看起来像这样：
```rust
fn add(self, s: &str) -> String {
```
这并不是标准库中实际的签名；标准库中的 add 使用泛型定义。这里我们看到的 add 的签名使用具体类型代替了泛型，这也正是当使用 String 值调用这个方法会发生的。

首先，s2 使用了 &，意味着我们使用第二个字符串的 引用 与第一个字符串相加。这是因为 add 函数的 s 参数：只能将 &str 和 String 相加，不能将两个 String 值相加。不过等一下 —— 正如 add 的第二个参数所指定的，&s2 的类型是 &String 而不是 &str。那么为什么示例还能编译呢？

之所以能够在 add 调用中使用 &s2 是因为 &String 可以被 强转（coerced）成 &str。当add函数被调用时，Rust 使用了一个被称为 Deref 强制转换（deref coercion）的技术，你可以将其理解为它把 &s2 变成了 &s2[..]。因为 add 没有获取参数的所有权，所以 s2 在这个操作后仍然是有效的 String。

其次，可以发现签名中 add 获取了 self 的所有权，因为 self __没有__ 使用 &。这意味着示例 8-18 中的 s1 的所有权将被移动到 add 调用中，之后就不再有效。所以虽然 let s3 = s1 + &s2; 看起来就像它会复制两个字符串并创建一个新的字符串，而实际上这个语句会获取 s1 的所有权，附加上从 s2 中拷贝的内容，并返回结果的所有权。换句话说，它看起来好像生成了很多拷贝，不过实际上并没有：这个实现比拷贝要更高效。

如果想要级联多个字符串，+ 的行为就显得笨重了：
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```
这时 s 的内容会是 “tic-tac-toe”。在有这么多 + 和 " 字符的情况下，很难理解具体发生了什么。对于更为复杂的字符串链接，可以使用 format!
宏：
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```
这些代码也会将 s 设置为 “tic-tac-toe”。format! 与 println! 的工作原理相同，不过不同于将输出打印到屏幕上，它返回一个带有结果内容的 String。这个版本就好理解的多，并且不会获取任何参数的所有权。

### 索引字符串
在很多语言中，通过索引来引用字符串中的单独字符是有效且常见的操作。然而在 Rust 中，如果你尝试使用索引语法访问 String 的一部分，会出现一个错误。考虑一下如示例 8-19 中所示的无效代码。
```rust
let s1 = String::from("hello");
let h = s1[0];
```
这段代码会导致如下错误：
```rust
error[E0277]: the trait bound `std::string::String: std::ops::Index<{integer}>` is not satisfied
 -->
  |
3 |     let h = s1[0];
  |             ^^^^^ the type `std::string::String` cannot be indexed by `{integer}`
  |
  = help: the trait `std::ops::Index<{integer}>` is not implemented for `std::string::String`
```
错误和提示说明了全部问题：Rust 的字符串不支持索引。那么接下来的问题是，为什么不支持呢？为了回答这个问题，我们必须先聊一聊 Rust 是如何在内存中储存字符串的。

### 字符串 slice
使用 [] 和一个 range 来创建含特定字节的字符串 slice：
```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```
这里，s 会是一个 &str，它包含字符串的头四个字节。早些时候，我们提到了这些字母都是两个字节长的，所以这意味着 s 将会是 “Зд”。

如果获取 &hello[0..1] 会发生什么呢？答案是：Rust 在运行时会 panic，就跟访问 vector 中的无效索引时一样：
```rust
thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/libcore/str/mod.rs:2188:4
```
你应该小心谨慎的使用这个操作，因为这么做可能会使你的程序崩溃。

### 遍历字符串的方法
幸运的是，这里还有其他获取字符串元素的方式。

如果你需要操作单独的 Unicode 标量值，最好的选择是使用 chars 方法。对 “नमस्ते” 调用 chars 方法会将其分开并返回六个 char 类型的值，接着就可以遍历其结果来访问每一个元素了：
```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```
这些代码会打印出如下内容：
```rust
न
म
स
्
त
े
```
bytes 方法返回每一个原始字节，这可能会适合你的使用场景：
```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```
这些代码会打印出组成 String 的 18 个字节：
```rust
224
164
// --snip--
165
135
```
