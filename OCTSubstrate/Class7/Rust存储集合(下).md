# 使用结构体组织相关联的数据
struct，或者 structure，是一个自定义数据类型，允许你命名和包装多个相关的值，从而形成一个有意义的组合。如果你熟悉一门面向对象语言，struct 就像对象中的数据属性。
## 定义并实例化结构体
和元组一样，结构体的每一部分可以是不同类型。但不同于元组，结构体需要命名各部分数据以便能清楚的表明其值的意义。由于有了这些名字，结构体比元组更灵活：不需要依赖顺序来指定或访问实例中的值。

定义结构体，需要使用 struct 关键字并为整个结构体提供一个名字。结构体的名字需要描述它所组合的数据的意义。接着，在大括号中，定义每一部分数据的名字和类型，我们称为 字段（field）。例如，示例展示了一个存储用户账号信息的结构体：
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
一旦定义了结构体后，为了使用它，通过为每个字段指定具体值来创建这个结构体的 实例。创建一个实例需要以结构体的名字开头，接着在大括号中使用 key: value 键-值对的形式提供字段，其中 key 是字段的名字，value 是需要存储在字段中的数据值。实例中字段的顺序不需要和它们在结构体中声明的顺序一致。换句话说，结构体的定义就像一个类型的通用模板，而实例则会在这个模板中放入特定数据来创建这个类型的值。例如，可以像示例这样来声明一个特定的用户：
```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```
为了从结构体中获取某个特定的值，可以使用点号。如果我们只想要用户的邮箱地址，可以用 user1.email。要更改结构体中的值，如果结构体的实例是可变的，我们可以使用点号并为对应的字段赋值。示例展示了如何改变一个可变的 User 实例 email 字段的值：
```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

注意整个实例必须是可变的；Rust 并不允许只将某个字段标记为可变。另外需要注意同其他任何表达式一样，我们可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式地返回这个实例。

示例显示了一个 build_user 函数，它返回一个带有给定的 email 和用户名的 User 结构体实例。active 字段的值为 true，并且 sign_in_count 的值为 1。

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```
为函数参数起与结构体字段相同的名字是可以理解的，但是不得不重复 email 和 username 字段名称与变量有些啰嗦。如果结构体有更多字段，重复每个名称就更加烦人了。幸运的是，有一个方便的简写语法！

### 变量与字段同名时的字段初始化简写语法
因为示例中的参数名与字段名都完全相同，我们可以使用 __字段初始化简写语法__（field init shorthand）来重写 build_user，这样其行为与之前完全相同，不过无需重复 email 和 username 了，如示例所示。
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```
这里我们创建了一个新的 User 结构体实例，它有一个叫做 email 的字段。我们想要将 email 字段的值设置为 build_user 函数 email 参数的值。因为 email 字段与 email 参数有着相同的名称，则只需编写 email 而不是 email: email。

### 使用结构体更新语法从其他实例创建实例
使用旧实例的大部分值但改变其部分值来创建一个新的结构体实例通常是很有帮助的。这可以通过 __结构体更新语法__（struct update syntax）实现。
```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
}
```
使用结构体更新语法，我们可以通过更少的代码来达到相同的效果，如示例所示。.. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。
```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

### 使用没有命名字段的元组结构体来创建不同的类型
也可以定义与元组类似的结构体，称为 __元组结构体__（tuple structs）。元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的，这时像常规结构体那样为每个字段命名就显得多余和形式化了。

要定义元组结构体，以 struct 关键字和结构体名开头并后跟元组中的类型。例如，下面是两个分别叫做 Color 和 Point 元组结构体的定义和用法：
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

### 没有任何字段的类单元结构体
我们也可以定义一个没有任何字段的结构体！它们被称为 类单元结构体（unit-like structs）因为它们类似于 ()，即 unit 类型。
