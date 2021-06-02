<https://exercism.io/tracks/rust/exercises>

[TOC]

# 1. Hello World

## `strings` `test driven development`

The classical introductory exercies. Just say "Hello, World!"

  

```rust
pub fn hello() -> &'static str {
    "Hello, World!"
}
```

# 2. Leap 

## `booleans` `conditionals`

Give a year, report if it is a leap year.

The tricky thing here is that a leap year in the Gregorian calendar occurs:

```text
on every year that is evenly divisible by 4
  except every year that is evenly divisible by 100
    unless the year is also evenly divisible by 400
```

For example, 1997 is not a leap year, but 1996 is. 1900 is not a leap year, but 2000 is.

```Rust
fn is_leap_year(year: u64) -> bool{
    if ((year % 4 == 0) && (year % 100 != 0)) ||
        ((year % 4 == 0) && (year % 100 == 0) && (year % 400 == 0)){
        return true;
    }
    false
}


fn process_leapyear_case(year: u64, expected: bool) {
    assert_eq!(is_leap_year(year), expected);
}
```

# 3. Raindrops

## `conditionals` `strings` `type conversion` `variables`

Convert a number to a string, the content of which depends on the number's factors.

Your task is to convert a number into a string that contains raindrop sounds corresponding to certain potential factors. A factor is a number that evenly divides into another number, leaving no remainder. The simplest way to test if a one number is a factor of another is to use the [modulo operation](https://en.wikipedia.org/wiki/Modulo_operation).

The rules of `raindrops` are that if a given number:

- has 3 as a factor, add 'Pling' to the result.
- has 5 as a factor, add 'Plang' to the result.
- has 7 as a factor, add 'Plong' to the result.
- *does not* have any of 3, 5, or 7 as a factor, the result should be the digits of the number.

## Examples

- 28 has 7 as a factor, but not 3 or 5, so the result would be "Plong".
- 30 has both 3 and 5 as factors, but not 7, so the result would be "PlingPlang".
- 34 is not factored by 3, 5, or 7, so the result would be "34".

```rust
pub fn raindrops(num: u32) -> String{
    let mut result = "".to_string();
    let three = "Pling";
    let five = "Plang";
    let seven = "Plong";
    if num % 3 == 0{
        result = result + three;
    }
    if num % 5 == 0{
        result = result + five;
    }
    if num % 7 == 0{
        result = result + seven;
    }
    if result == "".to_string(){
        let r = num.to_string();
        return r;
    }
    result
}
```

# 4 Proverb

## `format`

For want of a horseshoe nail, a kingdom was lost, or so the saying goes.

Given a list of inputs, generate the relevant proverb. For example, given the list `["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"]`, you will output the full text of this proverbial rhyme:

```text
For want of a nail the shoe was lost.
For want of a shoe the horse was lost.
For want of a horse the rider was lost.
For want of a rider the message was lost.
For want of a message the battle was lost.
For want of a battle the kingdom was lost.
And all for the want of a nail.
```

Note that the list of inputs may vary; your solution should be able to handle lists of arbitrary length and content. No line of the output text should be a static, unchanging string; all should vary according to the input given.

```rust
pub fn build_proverb(list: &[&str]) -> String{
    let mut s = String::new();
    if list.is_empty(){
        return s
    }
    for i in 1..list.len(){
        s.push_str(&format!("For want of a {} the {} was lost.\n", list[i-1], list[i]))
    }
    s.push_str(&format!("And all for the want of a {}.", list[0]));
    s
}
```

# 5 Difference Of Squares

## `fold` `map` `math`

Find the difference between the square of the sum and the sum of the squares of the first N natural numbers.

```rust
pub fn square_of_sum(num: u32)->u32{
    let mut all = num;
    for i in 0..num{
        all = all + i;
    }
    all * all
}

pub fn sum_of_squares(num: u32)->u32{
    let mut all = num * num;
    for i in 0..num{
        all = all + i * i;
    }
    all
}

pub fn difference(num: u32) -> u32{
    square_of_sum(num) - sum_of_squares(num)
}

```

