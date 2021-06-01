

<https://exercism.io/tracks/rust/exercises>

# 1. Hello World

## `strings` `test driven development`

The classical introductory exercies. Just say "Hello, World!"

```rust
pub fn hello() -> &'static str {
    "Hello, World!"
}

#[test]
fn test_hello_world() {
    assert_eq!("Hello, World!", hello());
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

#[test]
fn test_year_not_divisible_by_4_common_year() {
    process_leapyear_case(2015, false);
}

#[test]
fn test_year_divisible_by_2_not_divisible_by_4_in_common_year() {
    process_leapyear_case(1970, false);
}

#[test]
fn test_year_divisible_by_4_not_divisible_by_100_leap_year() {
    process_leapyear_case(1996, true);
}

#[test]
fn test_year_divisible_by_4_and_5_is_still_a_leap_year() {
    process_leapyear_case(1960, true);
}

#[test]
fn test_year_divisible_by_100_not_divisible_by_400_common_year() {
    process_leapyear_case(2100, false);
}

#[test]
fn test_year_divisible_by_100_but_not_by_3_is_still_not_a_leap_year() {
    process_leapyear_case(1900, false);
}

#[test]
fn test_year_divisible_by_400_leap_year() {
    process_leapyear_case(2000, true);
}

#[test]
fn test_year_divisible_by_400_but_not_by_125_is_still_a_leap_year() {
    process_leapyear_case(2400, true);
}

#[test]
fn test_year_divisible_by_200_not_divisible_by_400_common_year() {
    process_leapyear_case(1800, false);
}

#[test]
fn test_any_old_year() {
    process_leapyear_case(1997, false);
}

#[test]
fn test_early_years() {
    process_leapyear_case(1, false);
    process_leapyear_case(4, true);
    process_leapyear_case(100, false);
    process_leapyear_case(400, true);
    process_leapyear_case(900, false);
}

#[test]
fn test_century() {
    process_leapyear_case(1700, false);
    process_leapyear_case(1800, false);
    process_leapyear_case(1900, false);
}

#[test]
fn test_exceptional_centuries() {
    process_leapyear_case(1600, true);
    process_leapyear_case(2000, true);
    process_leapyear_case(2400, true);
}

#[test]
fn test_years_1600_to_1699() {
    let incorrect_years = (1600..1700)
        .filter(|&year| is_leap_year(year) != (year % 4 == 0))
        .collect::<Vec<_>>();

    if !incorrect_years.is_empty() {
        panic!("incorrect result for years: {:?}", incorrect_years);
    }
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

