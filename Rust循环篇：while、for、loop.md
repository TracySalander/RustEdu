

while没啥特殊的，只不过rust没do-while

rust的for没有三元语句，只有for i in a.iter() 或 for i in 0..5或 for i in 3.. 或 for i in 3..=n/2（这个可是太秀了。。。下面介绍可以用这个的例子）

loop就是"while true", 为啥Rust没有while true是为了一致性。。。Rust说While conditon == true才是指定的。

# Nth Prime

## `loops` `primes` `math`

Given a number n, determine what the nth prime is.

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

```rust
pub fn nth(num: u32) -> u32{
    let mut counter = 0;
    let mut current_num: u32 = 1;
    while counter <= num {
        current_num += 1;
        if is_prime(current_num){
            counter += 1;
        }
    }
    current_num
}

pub fn is_prime(num: u32) -> bool{
    if num == 1 { return false; }
    if num == 2 {return true; }
    for i in 2..=num/2{
        if num % i == 0{
            return false;
        }
    }
    true
}
```

# Beer Song

## `case` `strings` `loops` `vectors`

Produce the lyrics to that beloved classic, that field-trip favorite: 99 Bottles of Beer on the Wall.

99 bottles of beer on the wall, 99 bottles of beer.
Take one down and pass it around, 98 bottles of beer on the wall.

98 bottles of beer on the wall, 98 bottles of beer.
Take one down and pass it around, 97 bottles of beer on the wall.

...

3 bottles of beer on the wall, 3 bottles of beer.
Take one down and pass it around, 2 bottles of beer on the wall.

2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.

1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.

No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.

```rust
pub fn verse(num:u32) -> String{
    match num {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        n => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1)
    }
}

pub fn sing(start: u32, end: u32)->String{
    let mut lyric = verse(start);
    let mut counter = 1;
    for n in end..start{
        lyric = format!("{}\n{}", lyric, verse(start - counter));
        counter += 1;
    }
    lyric
}
```

