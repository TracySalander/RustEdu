# 函数
main 函数，它是很多程序的入口点。你也见过 fn 关键字，它用来声明新函数。

Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。这是一个包含函数定义示例的程序：
```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```
Rust 中的函数定义以 fn 开始并在函数名后跟一对圆括号。大括号告诉编译器哪里是函数体的开始和结尾。

可以使用函数名后跟圆括号来调用我们定义过的任意函数。因为程序中已定义 another_function 函数，所以可以在 main 函数中调用它。注意，源码中 another_function 定义在 main 函数 之后；也可以定义在之前。Rust 不关心函数定义于何处，只要定义了就行。

main 函数中的代码会按顺序执行。首先，打印 “Hello, world!” 信息，然后调用 another_function 函数并打印它的信息。
## 函数参数
函数也可以被定义为拥有 参数（parameters），参数是特殊变量，是函数签名的一部分。当函数拥有参数（形参）时，可以为这些参数提供具体的值（实参）。技术上讲，这些具体值被称为参数（arguments），但是在日常交流中，人们倾向于不区分使用 parameter 和 argument 来表示函数定义中的变量或调用函数时传入的具体值。

下面被重写的 another_function 版本展示了 Rust 中参数是什么样的：
```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```
another_function 的声明中有一个命名为 x 的参数。x 的类型被指定为 i32。当将 5 传给 another_function 时，println! 宏将 5 放入格式化字符串中大括号的位置。

在函数签名中，必须 声明每个参数的类型。这是 Rust 设计中一个经过慎重考虑的决定：要求在函数定义中提供类型注解，意味着编译器不需要你在代码的其他地方注明类型来指出你的意图。

当一个函数有多个参数时，使用逗号分隔，像这样：
```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
```
这个例子创建了一个名为 print_labeled_measurement 的函数，它有两个参数。第一个参数名为 value， 类型是 i32。第二个参数是 unit_label ，类型是 char。然后，该函数打印包含 value 和 unit_label 的文本。

## 包含语句和表达式的函数体
函数体由一系列的语句和一个可选的结尾表达式构成。目前为止，我们只介绍了没有结尾表达式的函数，不过你已经见过作为语句一部分的表达式。因为 Rust 是一门基于表达式（expression-based）的语言，这是一个需要理解的（不同于其他语言）重要区别。其他语言并没有这样的区别，所以让我们看看语句与表达式有什么区别以及这些区别是如何影响函数体的。

实际上，我们已经使用过语句和表达式。语句（Statements）是执行一些操作但不返回值的指令。表达式（Expressions）计算并产生一个值。让我们看一些例子：

使用 let 关键字创建变量并绑定一个值是一个语句。
```rust
fn main() {
    let y = 6;
}
```
函数定义也是语句，上面整个例子本身就是一个语句。

语句不返回值。因此，不能把 let 语句赋值给另一个变量，比如下面的例子尝试做的，会产生一个错误：
```rust
fn main() {
    let x = (let y = 6);
}
```
let y = 6 语句并不返回值，所以没有可以绑定到 x 上的值。这与其他语言不同，例如 C 和 Ruby，它们的赋值语句会返回所赋的值。在这些语言中，可以这么写 x = y = 6，这样 x 和 y 的值都是 6；Rust 中不能这样写。

表达式会计算出一个值，并且你将编写的大部分 Rust 代码是由表达式组成的。考虑一个数学运算，比如 5 + 6，这是一个表达式并计算出值 11。表达式可以是语句的一部分：在示例中，语句 let y = 6; 中的 6 是一个表达式，它计算出的值是 6。函数调用是一个表达式。宏调用是一个表达式。我们用来创建新作用域的大括号（代码块），{}，也是一个表达式，例如：
```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```
这个表达式：
```rust
{
    let x = 3;
    x + 1
}
```
是一个代码块，它的值是 4。这个值作为 let 语句的一部分被绑定到 y 上。注意结尾没有分号的那一行 x+1，与你见过的大部分代码行不同。表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。在接下来探索具有返回值的函数和表达式时要谨记这一点。

## 具有返回值的函数
函数可以向调用它的代码返回值。我们并不对返回值命名，但要在箭头（->）后声明它的类型。在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。这是一个有返回值的函数的例子：
```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```
five 函数的返回值是 5，所以返回值类型是 i32。让我们仔细检查一下这段代码。有两个重要的部分：首先，let x = five(); 这一行表明我们使用函数的返回值初始化一个变量。因为 five 函数返回 5，这一行与如下代码相同：
```rust
let x = 5;
```
其次，five 函数没有参数并定义了返回值类型，不过函数体只有单单一个 5 也没有分号，因为这是一个表达式，我们想要返回它的值。

让我们看看另一个例子：
```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```
运行代码会打印出 The value of x is: 6。但如果在包含 x + 1 的行尾加上一个分号，把它从表达式变成语句，我们将看到一个错误。
