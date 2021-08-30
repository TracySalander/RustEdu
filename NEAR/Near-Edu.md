# 第一次题目对应：Raindrops, Binary Search and Bonus of Binary Search

<https://github.com/near-apps/nft-market/blob/main/contracts/market-simple/src/ft_callbacks.rs>

line 17

```rust
let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
```

<https://github.com/near-apps/nft-market/blob/main/contracts/nft-simple/src/enumerable.rs>

Line 21 - line 30

```rust
	pub fn nft_tokens_batch(
        &self,
        token_ids: Vec<String>,
    ) -> Vec<JsonToken> {
        let mut tmp = vec![];
        for i in 0..token_ids.len() {
            tmp.push(self.nft_token(token_ids[i].clone()).unwrap());
        }
        tmp
    }
```





二者各有各的用途: **if 的条件是 true 或 false, 而 match 是模式匹配.**

**而且 Rust 不支持对表达式的模式匹配:**

```rust
match true {
    /* expression is not allowed */ => expression
}
```

**所以, 该用 if 的地方还是得用 if,** 这点和 Kotlin 的 when 表达式完全不一样.

> if let 和 if 有什么区别

**if 的条件是 true 或 false** (支持表达式)**, 而 if let 是模式匹配** (等号左边不支持表达式)**.**

该用啥用啥.表达式有值，语句没值

```rust
	pub fn nft_supply_for_type(
        &self,
        token_type: &String,
    ) -> U64 {
        let tokens_per_type = self.tokens_per_type.get(&token_type);
        if let Some(tokens_per_type) = tokens_per_type {
            U64(tokens_per_type.len())
        } else {
            U64(0)
        }
    }
```

fn function(x：&str）｛

｝

调用的时候可以传一个&&str类型的参数

但是反过来就不行 函数参数定义是&&str，传入&str，编译器就会报错

这个Rust的一个特性，叫自动解引用。

Rust的引用其实就是指针，给一个引用，一定能解引用出一个对象。

一个函数如果需要一个`i32`的参数，那给它`&i32`也应当是可行的，只需要解引用就行了。

另外，Rust的「解引用」操作通过`std::ops::Deref`的`deref`方法指定，解引用的`*x`其实就是`x.deref()`。 

Rust规定，在给函数传参数时，如果给的参数类型不符合函数需要的类型，而且实参的类型实现了`Deref` trait， 那就对实参调用一次`deref`方法，再重复这个步骤，直到下面两种情况之一发生：

- 经过若干次`deref`后，实参符合形参类型，调用成功；
- 经过若干次`deref`后，实参类型没有实现`Deref` trait，无法再解引用，调用失败。

这个特性主要是为智能指针和各种字符串类型设计的。比如一个函数要用`i32`类型的参数，你给它一个`Box<i32>`、`Rc<i32>`都是可以的，因为它们都实现了`Deref`
