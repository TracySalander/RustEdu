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

# 6 Sum Of Multiples

## `borrowing` `math` `algorithms`

Given a number, find the sum of all the unique multiples of particular numbers up to but not including that number.

If we list all the natural numbers below 20 that are multiples of 3 or 5, we get 3, 5, 6, 9, 10, 12, 15, and 18.

The sum of these multiples is 78.

```rust
pub fn sum_of_multiples(bound: u32, list: &[u32]) -> u32{
    let mut all = 0;
    for i in 1..bound{
        for j in list{
            if *j != 0 && i % j == 0{
                all += i;
                break;
            }
        }
    }
    all
}
```



# 7 Grains

`panic`

Calculate the number of grains of wheat on a chessboard given that the number on each square doubles.

There once was a wise servant who saved the life of a prince. The king promised to pay whatever the servant could dream up. Knowing that the king loved chess, the servant told the king he would like to have grains of wheat. One grain on the first square of a chess board, with the number of grains doubling on each successive square.

There are 64 squares on a chessboard (where square 1 has one grain, square 2 has two grains, and so on).

Write code that shows:

- how many grains were on a given square, and
- the total number of grains on the chessboard

```rust
pub fn square(num: u32) -> u64{
    if num <= 0 || num > 64{
        panic!("Square must be between 1 and 64")
    }
    if num == 1{
        return 1;
    }
    2 * square(num-1)
}

pub fn total() -> u64{
    let mut all = 0;
    for i in 1..65{
        all += square(i);
    }
    all
}
```

# 8 Prime Factors

## `math`

Compute the prime factors of a given natural number.

A prime number is only evenly divisible by itself and 1.

Note that 1 is not a prime number.

## Example

What are the prime factors of 60?

- Our first divisor is 2. 2 goes into 60, leaving 30.
- 2 goes into 30, leaving 15.
  - 2 doesn't go cleanly into 15. So let's move on to our next divisor, 3.
- 3 goes cleanly into 15, leaving 5.
  - 3 does not go cleanly into 5. The next possible factor is 4.
  - 4 does not go cleanly into 5. The next possible factor is 5.
- 5 does go cleanly into 5.
- We're left only with 1, so now, we're done.

Our successful divisors in that computation represent the list of prime factors of 60: 2, 2, 3, and 5.

You can check this yourself:

- 2 * 2 * 3 * 5
- = 4 * 15
- = 60
- Success!

```rust
pub fn factors(num: u64) -> Vec<u64>{
    let mut left= num;
    let mut result = Vec::new();
    let mut divisor = 2;
    while left > 1{
        while left % divisor == 0 {
            left /= divisor;
            result.push(divisor);
        }
        divisor += 1;
    }
    println!("{}", num);
    result
}
```

# 9 Armstrong Numbers

## `math`

An [Armstrong number](https://en.wikipedia.org/wiki/Narcissistic_number) is a number that is the sum of its own digits each raised to the power of the number of digits.

For example:

- 9 is an Armstrong number, because `9 = 9^1 = 9`
- 10 is *not* an Armstrong number, because `10 != 1^2 + 0^2 = 1`
- 153 is an Armstrong number, because: `153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153`
- 154 is *not* an Armstrong number, because: `154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190`

Write some code to determine whether a number is an Armstrong number.

```rust
pub fn is_armstrong_number(num: u32) -> bool{
    let size: u32 = num.to_string().len() as u32;
    let mut result = 0;
    let mut n = num;
    while n > 0{
        let temp = n % 10;
        result += temp.pow(size);
        n /= 10;
    }
    result == num
}
```

# 10 Reverse String

`str vs string` `strings` `iterators`

Reverse a string

For example: input: "cool" output: "looc"

```rust
pub fn reverse(input: &str) -> String{
    input.chars().rev().collect::<String>()
}
```



