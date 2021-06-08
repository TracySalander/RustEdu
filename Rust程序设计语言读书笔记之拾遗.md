# Rust程序设计语言读书笔记之拾遗

## 2 猜猜看游戏教程

`::new` 那一行的 `::` 语法表明 `new` 是 `String` 类型的一个 **关联函数**（*associated function*）。关联函数是针对类型实现的，在这个例子中是 `String`，而不是 `String` 的某个特定实例。一些语言中把它称为 **静态方法**（*static method*）。

```rust
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

`read_line` 的工作是，无论用户在标准输入中键入什么内容，都将其存入一个字符串中，因此它需要字符串作为参数。这个字符串参数应该是可变的，以便 `read_line` 将用户输入附加上去。read_line返回一个枚举类型Result

`use`：`use rand::Rng`。`Rng` 是一个 trait，它定义了随机数生成器应实现的方法

`rand::thread_rng` 函数提供实际使用的随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取 seed。接下来，调用随机数生成器的 `gen_range` 方法。这个方法由刚才引入到作用域的 `Rng` trait 定义。`gen_range` 方法获取两个数字作为参数，并生成一个范围在两者之间的随机数。它包含下限但不包含上限，所以需要指定 `1` 和 `101` 来请求一个 1 和 100 之间的数。

从标准库引入了一个叫做 `std::cmp::Ordering` 的类型。同 `Result` 一样， `Ordering` 也是一个枚举，不过它的成员是 `Less`、`Greater` 和 `Equal`。这是比较两个值时可能出现的三种结果。

```rust
		let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
```

这里创建了一个叫做 `guess` 的变量。不过等等，不是已经有了一个叫做 `guess` 的变量了吗？确实如此，不过 Rust 允许用一个新值来 **隐藏** （*shadow*） `guess` 之前的值。这个功能常用在需要转换值类型之类的场景。

我们将 `guess` 绑定到 `guess.trim().parse()` 表达式上。表达式中的 `guess` 是包含输入的原始 `String`类型。`String` 实例的 `trim` 方法会去除字符串开头和结尾的空白字符。`u32` 只能由数字字符转换，不过用户必须输入 enter 键才能让 `read_line` 返回，然而用户按下 enter 键时，会在字符串中增加一个换行（newline）符。例如，用户输入 5 并按下 enter，`guess` 看起来像这样：`5\n`。`\n` 代表 “换行”，回车键。`trim` 方法消除 `\n`，只留下 `5`。

[字符串的 `parse` 方法](https://doc.rust-lang.org/std/primitive.str.html#method.parse) 将字符串解析成数字。因为这个方法可以解析多种数字类型，因此需要告诉 Rust 具体的数字类型，这里通过 `let guess: u32` 指定。返回Result枚举。

## 3.2 数据类型

char, int, float, bool + tuple array全都存在stack上。（所有编译时候就知道大小的都存在stack上，比如字符串字面量，String, Vector， HashMap则存在于Heap上）

`tup` 变量绑定到整个元组上，因为元组是一个单独的复合元素。为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值，像这样：

文件名: src/main.rs

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

可以像这样编写数组的类型：在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Rust的数组是在栈（stack）上为数据分配内存空间，只有能在栈上存放的元素才可以存放在该类型的数组中

下面是一个使用 `for` 循环来倒计时的例子，它还使用了一个我们还未讲到的方法，`rev`，用来反转 range：

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

## 3.5 控制流题目

你做到了！这是一个大章节：你学习了变量、标量和复合数据类型、函数、注释、 `if` 表达式和循环！如果你想要实践本章讨论的概念，尝试构建如下程序：

- 相互转换摄氏与华氏温度。
- 生成 n 阶斐波那契数列。
- 打印圣诞颂歌 “The Twelve Days of Christmas” 的歌词，并利用歌曲中的重复部分（编写循环）。

```rust
pub fn f_to_c(f: f64)-> f64{
    (f - 32.00) / 1.80000
}

pub fn c_to_f(c: f64)->f64{
    c * 1.8000 + 32.00
}

pub fn fib(n: i32) -> i32{
    if n == 1{
        return 1;
    } else if n == 2{
        return 1;
    } 
    return fib(n-1) + fib(n-2);
}

pub fn lyrics(){
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", 
    "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let mut s1 = "a partridge in a pear tree ";
    let mut s2 = "two turtle doves ";
    let mut s3 = "three french hens ";
    let mut s4 = "four calling birds ";
    let mut s5 = "five gold rings ";
    let mut s6 = "six geese a laying ";
    let mut s7 = "seven swans a swimming ";
    let mut s8 = "eight maids a milking ";
    let mut s9 = "nine ladies dancing ";
    let mut s10 = "ten lords a leaping ";
    let mut s11 = "eleven pipers piping ";
    let mut s12 = "twelve drummers drumming ";
    let mut lyrics_store = [s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12];
    let mut help = 1;

    for i in days.iter(){
        println!("on the {} day of Christmas my true love gave to me", i);
        for j in (0..help).rev(){
            if help > 1 && j == 0 {
                print!("and ");
            }
            print!("{}", lyrics_store[j]);
        }
        help = help + 1;
        println!();
        println!();
    }
}

pub fn get_fib_vec(n: i32) -> Vec<i32>{
    let mut result = Vec::new();
    for i in (1..n+1){
        result.push(fib(i));
    }
    result
}
fn main(){
    println!("{}", f_to_c(32.0));
    println!("{}", c_to_f(100.0));

    let re = get_fib_vec(10);
    for i in re.iter(){
        print!("{} ", i);
    }

    println!();
    println!();
    
    lyrics();
}
```



