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

## 4 认识所有权

__对于数据内容放在Stack上的，就看成深拷贝，官方不考虑这里的深浅问题，统一叫Copy__

```rust
fn main(){
    let mut a = 3;
    let b = a;
    a = 2;
    println!("{}, {}", a, b);
}
```

__对于数据内容存在Heap上的，=赋值就相当于move，Rust里没有浅拷贝，因为安全，防止出了作用域drop多次，只要赋值了，前一个就死了。要想达到深拷贝需要用clone()方法。__



__普通类型直接传入函数之后还能用copy，复杂类型就不行了move， 但是如果对于复杂类型，我们只想用值但是不给所有权，那就用引用，引用相当于在Stack中创建了一个变量指向同在Stack中的另一个指向Heap的变量。就算新创建这个死了也不影响剩下的所有东西，引用作为函数参数的术语叫借用borrowing(有借有还)， 但是如果这两个Stack变量都加了mut那就可以通过引用改变Heap中的原始数据__

为了解决需要用一部分String时候，如果String变了，对新变量不产生影响的问题，Rust用slice解决，实际上是指针指向String指向的Heap空间的部分内容。之前的引用是指向Stack上的变量名，这里是直接指向Heap。字符串slice是个特殊的类型，写作&str。let s = "Hello, world!"是字符串字面量，但本质就是&str。当声明函数参数类型时候用&str可以传入&str和&String，但是如果声明的是&String则只能传入&String不能传入&str，所以有经验的大佬都写&str

```rust
fn main() {
    let my_string = String::from("hello world");

    // first_word 中传入 `String` 的 slice
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);
}
```

## 5 使用结构体来组织相关联的数据

struct里有一种特殊的，叫tuple structs，元组结构体，其实就是结构体只不过没有字段名。一般就是用来区分类型的：例子

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Struct定义如果有引用类型必须指明生命周期

打印Struct内容可以用{:?}或{:#?}但是需要引入#[derive(debug)]这个trait

## 6 枚举与模式匹配

枚举的用处在于可以轻易的定义一个能够处理这些不同类型的结构体的函数，因为枚举是单独一个类型。结构体和枚举还有另一个相似点：就像可以使用 `impl` 来为结构体定义方法那样，也可以在枚举上定义方法。

match是用来处理枚举的。match可以用多个if let, else代替但是我觉得还是match比较好。就像用switch能弄的用ifelse弄挺变扭的。

## 7 使用包、Crate和模块管理不断增长的项目

Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项。这是因为子模块封装并隐藏了他们的实现详情，但是子模块可以看到他们定义的上下文。继续拿餐馆作比喻，把私有性规则想象成餐馆的后台办公室：餐馆内的事务对餐厅顾客来说是不可知的，但办公室经理可以洞悉其经营的餐厅并在其中做任何事情。

Rust 选择以这种方式来实现模块系统功能，因此默认隐藏内部实现细节。这样一来，你就知道可以更改内部代码的哪些部分而不会破坏外部代码。你还可以通过使用 `pub` 关键字来创建公共项，使子模块的内部部分暴露给上级模块。

## 8 常见集合

`format!` 与 `println!` 的工作原理相同，不过不同于将输出打印到屏幕上，它返回一个带有结果内容的 `String`。

个构建哈希 map 的方法是使用一个元组的 vector 的 `collect` 方法，其中每个元组包含一个键值对。`collect` 方法可以将数据收集进一系列的集合类型，包括 `HashMap`。例如，如果队伍的名字和初始分数分别在两个 vector 中，可以使用 `zip` 方法来创建一个元组的 vector，其中 “Blue” 与 10 是一对，依此类推。接着就可以使用 `collect` 方法将这个元组 vector 转换成一个 `HashMap`

```rust
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

#### [只在键没有对应值时插入](https://kaisery.github.io/trpl-zh-cn/ch08-03-hash-maps.html#只在键没有对应值时插入)

我们经常会检查某个特定的键是否有值，如果没有就插入一个值。为此哈希 map 有一个特有的 API，叫做 `entry`，它获取我们想要检查的键作为参数。`entry` 函数的返回值是一个枚举，`Entry`，它代表了可能存在也可能不存在的值。比如说我们想要检查黄队的键是否关联了一个值。如果没有，就插入值 50，对于蓝队也是如此。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

### 第八章课后练习题

- 给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希函数会很有帮助）。

```rust
use std::collections::HashMap;
pub fn find_numbers(input: &mut Vec<i32>)-> Vec<i32>{
    let mut result = Vec::new();
    let mut mean: i32 = 0;
    let mut mid: i32 = 0;
    let mut mode: i32 = 0;
    input.sort();
    mid = input[input.len()/2];
    let input2 = input.clone();
    for i in &input2{
        mean += *i;
    }
    mean = mean / (input.len() as i32);
    let mut store_hash = HashMap::new();
    for i in &input2{
        let count = store_hash.entry(i).or_insert(0);
        *count += 1;
    }
    for (_key, value) in &store_hash{
        if value > &mode{
            mode = *value;
        }
    }
    result.push(mean);
    result.push(mid);
    result.push(mode);
    result
}
fn main(){
    let mut input: Vec<i32> = vec![1, 2, 3, 4, 5, 1];
    println!("{:?}", find_numbers(&mut input));
}
```



- 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！

```rust
pub fn pig_latin(input: &String)->String{
    let vow = ["a", "e", "i", "o", "u"];
    let head = &input[..1];
    let rest = &input[1..];

    match vow.contains(&head){
        true => format!("{}-{}", input, "hay"),
        false => format!("{}-{}{}", rest, head, "ay"),
    }
}
fn main(){
    println!("{}", pig_latin(&String::from("first")));
    println!("{}", pig_latin(&String::from("apple")));
}
```



## 10 泛型、trait与生命周期

`impl Trait` 很方便，适用于短小的例子。trait bound 则适用于更复杂的场景。例如，可以获取两个实现了 `Summary` 的参数。使用 `impl Trait` 的语法看起来像这样：

```rust
pub fn notify(item1: impl Summary, item2: impl Summary) {
```

这适用于 `item1` 和 `item2` 允许是不同类型的情况（只要它们都实现了 `Summary`）。不过如果你希望强制它们都是相同类型呢？这只有在使用 trait bound 时才有可能：

```rust
pub fn notify<T: Summary>(item1: T, item2: T) {
```

泛型 `T` 被指定为 `item1` 和 `item2` 的参数限制，如此传递给参数 `item1` 和 `item2` 值的具体类型必须一致。

#### [通过 `+` 指定多个 trait bound](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html#通过--指定多个-trait-bound)

如果 `notify` 需要显示 `item` 的格式化形式，同时也要使用 `summarize` 方法，那么 `item` 就需要同时实现两个不同的 trait：`Display` 和 `Summary`。这可以通过 `+` 语法实现：

```rust
pub fn notify(item: impl Summary + Display) {
```

`+` 语法也适用于泛型的 trait bound：

```rust
pub fn notify<T: Summary + Display>(item: T) {
```

通过指定这两个 trait bound，`notify` 的函数体可以调用 `summarize` 并使用 `{}` 来格式化 `item`。

#### [通过 `where` 简化 trait bound](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html#通过-where-简化-trait-bound)

然而，使用过多的 trait bound 也有缺点。每个泛型有其自己的 trait bound，所以有多个泛型参数的函数在名称和参数列表之间会有很长的 trait bound 信息，这使得函数签名难以阅读。为此，Rust 有另一个在函数签名之后的 `where` 从句中指定 trait bound 的语法。所以除了这么写：

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

还可以像这样使用 `where` 从句：

```rust
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

这个函数签名就显得不那么杂乱，函数名、参数列表和返回值类型都离得很近，看起来类似没有很多 trait bounds 的函数。

生命周期注解并不改变任何引用的生命周期的长短。与当函数签名中指定了泛型类型参数后就可以接受任何类型一样，当指定了泛型生命周期后函数也能接受任何生命周期的引用。生命周期注解描述了多个引用生命周期相互的关系，而不影响其生命周期。
