# Rust习题集中级款

# 1 Clock

Implement a clock that handles times without dates.

You should be able to add and subtract minutes to it.

Two clocks that represent the same time should be equal to each other.

Rust Traits for `.to_string()`

Did you implement `.to_string()` for the `Clock` struct?

If so, try implementing the [Display trait](https://doc.rust-lang.org/std/fmt/trait.Display.html) for `Clock` instead.

Traits allow for a common way to implement functionality for various types.

For additional learning, consider how you might implement `String::from`for the `Clock` type. You don't have to actually implement this—it's redundant with `Display`, which is generally the better choice when the destination type is `String`—but it's useful to have a few type-conversion traits in your toolkit.

```rust
use chrono::{naive::NaiveTime, Duration, format::strftime::StrftimeItems};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock{
    pub time: NaiveTime,
}

impl Clock{
    pub fn new(hours: i32, minutes: i32)-> Self{
        let total_minutes = hours * 60 + minutes;
        let time = NaiveTime::from_hms(0, 0, 0) + Duration::minutes(total_minutes as i64);
        Self{
            time
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self{
        let time = self.time + Duration::minutes(minutes as i64);
        Self{
            time
        }
    }
}

impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let time_format = StrftimeItems::new("%H:%M");
        let time = self.time.format_with_items(time_format);

        write!(f, "{}", time)
    }
}
```

# 2 Binary Search

Implement a binary search algorithm.

Searching a sorted collection is a common task. A dictionary is a sorted list of word definitions. Given a word, one can find its definition. A telephone book is a sorted list of people's names, addresses, and telephone numbers. Knowing someone's name allows one to quickly find their telephone number and address.

If the list to be searched contains more than a few items (a dozen, say) a binary search will require far fewer comparisons than a linear search, but it imposes the requirement that the list be sorted.

In computer science, a binary search or half-interval search algorithm finds the position of a specified input value (the search "key") within an array sorted by key value.

In each step, the algorithm compares the search key value with the key value of the middle element of the array.

If the keys match, then a matching element has been found and its index, or position, is returned.

Otherwise, if the search key is less than the middle element's key, then the algorithm repeats its action on the sub-array to the left of the middle element or, if the search key is greater, on the sub-array to the right.

If the remaining array to be searched is empty, then the key cannot be found in the array and a special "not found" indication is returned.

A binary search halves the number of items to check with each iteration, so locating an item (or determining its absence) takes logarithmic time. A binary search is a dichotomic divide and conquer search algorithm.

## Restrictions

Rust provides in its standard library already a [binary search function](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search). For this exercise you should not use this function but just other basic tools instead.

## Hints

[Slices](https://doc.rust-lang.org/book/2018-edition/ch04-03-slices.html) have additionally to the normal element access via indexing (slice[index]) many useful functions like [split_at](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at) or [getting subslices](https://doc.rust-lang.org/std/primitive.slice.html#method.get)(slice[start..end]).

You can solve this exercise by just using boring old element access via indexing, but maybe the other provided functions can make your code cleaner and safer.

## For bonus points

Did you get the tests passing and the code clean? If you want to, there are some additional things you could try.

- Currently your find function will probably only work for slices of numbers, but the Rust type system is flexible enough to create a find function which works on all slices which contains elements which can be ordered.
- Additionally this find function can work not only on slices, but at the same time also on a Vec or an Array.

To run the bonus tests, remove the `#[ignore]` flag and execute the tests with the `generic` feature, like this:

```bash
$ cargo test --features generic
```

Then please share your thoughts in a comment on the submission. Did this experiment make the code better? Worse? Did you learn anything from it?

### Hints for Bonus Points

- To get your function working with all kind of elements which can be ordered, have a look at the [Ord Trait](https://doc.rust-lang.org/std/cmp/trait.Ord.html).
- To get your function working directly on Vec and Array, you can use the [AsRef Trait](https://doc.rust-lang.org/std/convert/trait.AsRef.html)

```rust
pub fn find(input: &[i32], key: i32)-> Option<usize>{
    if input.len() == 0{
        return None;
    }

    let mut left_index = 0;
    let mut right_index = input.len();
    let mut mid_index = 0;

    while (left_index <= right_index) && (key <= input[input.len()-1]) && (key >= input[0]){
        mid_index = (right_index - left_index) / 2 + left_index;
        if key == input[mid_index]{
            return Some(mid_index);
        } else if key < input[mid_index] {
            right_index = mid_index-1;
        } else{
            left_index = mid_index+1;
        }
    }
    None
}

```



```rust
pub fn find<T: Ord, V: AsRef<[T]>>(input: V, key: T)->Option<usize>{
    let input = input.as_ref();
    if input.len() == 0{
        return None;
    }

    let mut left = 0;
    let mut right = input.len() - 1;

    while left <= right && key >= input[0] && key <= input[input.len()-1] {
        let mid = (left+right)/2;

        if input[mid] == key{
            return Some(mid);
        } else if input[mid] < key{
            left = mid + 1;
        } else{
            right = mid - 1;
        }
    }
    None
}
```



# 3 Pascal's Triangle

Compute Pascal's triangle up to a given number of rows.

In Pascal's Triangle each number is computed by adding the numbers to the right and left of the current position in the previous row.

```text
    1
   1 1
  1 2 1
 1 3 3 1
1 4 6 4 1
# ... etc
```

```rust
pub struct PascalsTriangle{
    row_number: u32,
}

impl PascalsTriangle{
    pub fn new(root: u32)->Self{
        PascalsTriangle{ row_number: root }
    }

    pub fn rows(&self) -> Vec<Vec<u32>>{
        let mut result: Vec<Vec<u32>> = Vec::new();
        if self.row_number == 0{ return result;}
        else {
            result.push(vec![1]);
        }
        for i in 1..self.row_number as usize{
            let mut row = vec![];
            let prev_row = result.last().unwrap();
            for j in 0..=i{
                if j > 0 && j < prev_row.len(){
                    row.push(prev_row[j-1] + prev_row[j]);
                } else{
                    row.push(1);
                }
            }
            result.push(row);
        }
        result
    }
}
```



# 4 Acronym

Convert a phrase to its acronym.

Techies love their TLA (Three Letter Acronyms)!

Help generate some jargon by writing a program that converts a long name like Portable Network Graphics to its acronym (PNG).

```rust
pub fn abbreviate(input: &str)->String{
    input
        .split(|ch: char| ch.is_ascii_whitespace() || "-:_*".contains(ch))
        .filter(|word| !word.is_empty())
        .map(|word| {
            word.chars().take(1).chain(
                word.chars()
                        .skip_while(|c| c.is_ascii_uppercase())
                        .filter(|c| c.is_ascii_uppercase())
            )
            .collect::<String>()
            .to_ascii_uppercase()
        })
        .collect()
}
```



# 5 Grade School

Given students' names along with the grade that they are in, create a roster for the school.

In the end, you should be able to:

- Add a student's name to the roster for a grade
  - "Add Jim to grade 2."
  - "OK."
- Get a list of all students enrolled in a grade
  - "Which students are in grade 2?"
  - "We've only got Jim just now."
- Get a sorted list of all students in all grades. Grades should sort as 1, 2, 3, etc., and students within a grade should be sorted alphabetically by name.
  - "Who all is enrolled in school right now?"
  - "Let me think. We have Anna, Barb, and Charlie in grade 1, Alex, Peter, and Zoe in grade 2 and Jim in grade 5. So the answer is: Anna, Barb, Charlie, Alex, Peter, Zoe and Jim"

Note that all our students only have one name. (It's a small town, what do you want?)

```rust

#[allow(clippy::new_without_default)]
#[derive(Default)]
pub struct School{
    student_list: Vec<Student>,
}
pub struct Student{
    student_grade: u32,
    student_name: String,
}
impl School{
    pub fn new()->School{
        Default::default()
    }

    pub fn grades(&self)-> Vec<u32>{
        let mut result = self.student_list.iter().map(|s| s.student_grade).collect::<Vec<u32>>();
        result.sort();
        result.dedup();
        result
    }

    pub fn add(&mut self, grade: u32, name: &str){
        self.student_list.push(Student{
            student_grade: grade,
            student_name: String::from(name),
        })
    } 

    pub fn grade(&self, grade: u32)-> Vec<String>{
        let mut result = self.student_list.iter().filter_map(|s| match s.student_grade == grade{
            true => Some(s.student_name.clone()),
            _ => None,
        }).collect::<Vec<String>>();
        result.sort();
        result
    }
}
```



# 6 Queen Attack

Given the position of two queens on a chess board, indicate whether or not they are positioned so that they can attack each other.

In the game of chess, a queen can attack pieces which are on the same row, column, or diagonal.

A chessboard can be represented by an 8 by 8 array.

So if you're told the white queen is at (2, 3) and the black queen at (5, 6), then you'd know you've got a set-up like so:

```text
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ W _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ B _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
```

You'd also be able to answer whether the queens can attack each other. In this case, that answer would be yes, they can, because both pieces share a diagonal.

```rust
pub struct ChessPosition(i32, i32);

pub struct Queen{
    position: ChessPosition,
}

impl ChessPosition{
    pub fn new(row: i32, column: i32) -> Option<Self>{
        if row <= 7 && row >= 0 && column <= 7 && column >= 0{
            return Some(ChessPosition(row, column));
        }
        None
    }
}

impl Queen{
    pub fn new(position: ChessPosition)->Self{
        Queen{position}
    }

    pub fn can_attack(&self, other: &Queen) -> bool{
        if self.position.0 == other.position.0
            || self.position.1 == other.position.1
            || abs(self.position.0 - other.position.0) == abs(self.position.1 - other.position.1){
                return true;
            }
        false
    }
}

fn abs(x: i32) -> i32 {
    print!("The abs value of {} is ", x);

    if x > 0 {
        return x;
    } else {
        -x
    }
}
```



# 7 Isogram

Determine if a word or phrase is an isogram.

An isogram (also known as a "nonpattern word") is a word or phrase without a repeating letter, however spaces and hyphens are allowed to appear multiple times.

Examples of isograms:

- lumberjacks
- background
- downstream
- six-year-old

The word *isograms*, however, is not an isogram, because the s repeats.

```rust
use std::collections::HashSet;

pub fn check(input: &str)->bool{
    let mut hs = HashSet::new();
    input.to_lowercase().chars().filter(char::is_ascii_alphabetic).all(|x| hs.insert(x))
}
```



# 8 Perfect Numbers

Determine if a number is perfect, abundant, or deficient based on Nicomachus' (60 - 120 CE) classification scheme for positive integers.

The Greek mathematician [Nicomachus](https://en.wikipedia.org/wiki/Nicomachus) devised a classification scheme for positive integers, identifying each as belonging uniquely to the categories of **perfect**, **abundant**, or **deficient** based on their [aliquot sum](https://en.wikipedia.org/wiki/Aliquot_sum). The aliquot sum is defined as the sum of the factors of a number not including the number itself. For example, the aliquot sum of 15 is (1 + 3 + 5) = 9

- Perfect

  : aliquot sum = number

  - 6 is a perfect number because (1 + 2 + 3) = 6
  - 28 is a perfect number because (1 + 2 + 4 + 7 + 14) = 28

- Abundant

  : aliquot sum > number

  - 12 is an abundant number because (1 + 2 + 3 + 4 + 6) = 16
  - 24 is an abundant number because (1 + 2 + 3 + 4 + 6 + 8 + 12) = 36

- Deficient

  : aliquot sum < number

  - 8 is a deficient number because (1 + 2 + 4) = 7
  - Prime numbers are deficient

Implement a way to determine whether a given number is **perfect**. Depending on your language track, you may also need to implement a way to determine whether a given number is **abundant** or **deficient**.

```rust
#[derive(PartialEq, Debug)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient
}

pub fn classify(input: u64)->Option<Classification>{
    let aliquot: u64 = (1..=input/2).into_iter().filter(|&x| input % x == 0).sum();
    match aliquot{
        aliquot if aliquot < input => Some(Classification::Deficient),
        aliquot if aliquot == input => Some(Classification::Perfect),
        aliquot if aliquot > input => Some(Classification::Abundant),
        _ => None
    }
}
```



# 9 Pangram

Determine if a sentence is a pangram. A pangram (Greek: παν γράμμα, pan gramma, "every letter") is a sentence using every letter of the alphabet at least once. The best known English pangram is:

> The quick brown fox jumps over the lazy dog.

The alphabet used consists of ASCII letters `a` to `z`, inclusive, and is case insensitive. Input will not contain non-ASCII symbols.

```rust
use std::collections::HashSet;
pub fn is_pangram(input: &str)->bool{
    input
        .to_lowercase()
        .chars()
        .filter(|x| x.is_ascii_lowercase())
        .collect::<HashSet<char>>()
        .len() == 26
}
```



# 10 All Your Base

Convert a number, represented as a sequence of digits in one base, to any other base.

Implement general base conversion. Given a number in base **a**, represented as a sequence of digits, convert it to base **b**.

Note

- Try to implement the conversion yourself. Do not use something else to perform the conversion for you.

About [Positional Notation](https://en.wikipedia.org/wiki/Positional_notation)

In positional notation, a number in base **b** can be understood as a linear combination of powers of **b**.

The number 42, *in base 10*, means:

(4 * 10^1) + (2 * 10^0)

The number 101010, *in base 2*, means:

(1 * 2^5) + (0 * 2^4) + (1 * 2^3) + (0 * 2^2) + (1 * 2^1) + (0 * 2^0)

The number 1120, *in base 3*, means:

(1 * 3^3) + (1 * 3^2) + (2 * 3^1) + (0 * 3^0)

I think you got the idea!

*Yes. Those three numbers above are exactly the same. Congratulations!*

```rust
#[derive(Debug, PartialEq)]
pub enum Error{
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn to_digits(mut number: u32, base: u32)->Vec<u32>{
    if number == 0{
        return vec![0];
    }
    let mut result = vec![];
    while number > 0{
        result.insert(0, number % base);
        number /= base;
    }
    result
}

pub fn convert(input_digits: &[u32], input_base: u32, output_base: u32)->Result<Vec<u32>, Error>{
    if input_base <= 1{
        return Err(Error::InvalidInputBase);
    }
    if output_base <= 1{
        return Err(Error::InvalidOutputBase);
    }
    let mut x = 0;
    for &a in input_digits {
        if a >= input_base{
            return Err(Error::InvalidDigit(a));
        }
        x = x * input_base + a;
    }
    Ok(to_digits(x, output_base))
}
```

  

# 11 Scrabble Score

Given a word, compute the Scrabble score for that word.

## Letter Values

You'll need these:

```text
Letter                           Value
A, E, I, O, U, L, N, R, S, T       1
D, G                               2
B, C, M, P                         3
F, H, V, W, Y                      4
K                                  5
J, X                               8
Q, Z                               10
```

## Examples

"cabbage" should be scored as worth 14 points:

- 3 points for C
- 1 point for A, twice
- 3 points for B, twice
- 2 points for G
- 1 point for E

And to total:

- `3 + 2*1 + 2*3 + 2 + 1`
- = `3 + 2 + 6 + 3`
- = `5 + 9`
- = 14

## Extensions

- You can play a double or a triple letter.
- You can play a double or a triple word.

```rust
pub fn score(input: &str)->u64{
    input.chars()
        .map(|c| match c.to_ascii_lowercase(){
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'k' => 5,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0,
        }).sum()
}
```



# 12 Allergies

Given a person's allergy score, determine whether or not they're allergic to a given item, and their full list of allergies.

An allergy test produces a single numeric score which contains the information about all the allergies the person has (that they were tested for).

The list of items (and their value) that were tested are:

- eggs (1)
- peanuts (2)
- shellfish (4)
- strawberries (8)
- tomatoes (16)
- chocolate (32)
- pollen (64)
- cats (128)

So if Tom is allergic to peanuts and chocolate, he gets a score of 34.

Now, given just that score of 34, your program should be able to say:

- Whether Tom is allergic to any one of those allergens listed above.
- All the allergens Tom is allergic to.

Note: a given score may include allergens **not** listed above (i.e. allergens that score 256, 512, 1024, etc.). Your program should ignore those components of the score. For example, if the allergy score is 257, your program should only report the eggs (1) allergy.

```rust
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum Allergen{
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

pub struct Allergies{
    score: u32,
}

impl Allergies{
    pub fn new(score: u32)->Self{
        Allergies{score}
    }

    pub fn is_allergic_to(&self, allergic: &Allergen)-> bool{
        (*allergic as u32) & self.score != 0
    }

    pub fn allergies(&self)->Vec<Allergen>{
        [Allergen::Eggs, Allergen::Peanuts, Allergen::Shellfish, 
        Allergen::Strawberries, Allergen::Tomatoes, Allergen::Chocolate, 
        Allergen::Pollen, Allergen::Cats].iter()
        .filter_map(|&x| {
            if self.is_allergic_to(&x){
                Some(x)
            } else {
                None
            }
        })
        .collect()
    }
}

```



# 13 Say

Given a number from 0 to 999,999,999,999, spell out that number in English.

## Step 1

Handle the basic case of 0 through 99.

If the input to the program is `22`, then the output should be `'twenty-two'`.

Your program should complain loudly if given a number outside the blessed range.

Some good test cases for this program are:

- 0
- 14
- 50
- 98
- -1
- 100

### Extension

If you're on a Mac, shell out to Mac OS X's `say` program to talk out loud. If you're on Linux or Windows, eSpeakNG may be available with the command `espeak`.

## Step 2

Implement breaking a number up into chunks of thousands.

So `1234567890` should yield a list like 1, 234, 567, and 890, while the far simpler `1000` should yield just 1 and 0.

The program must also report any values that are out of range.

## Step 3

Now handle inserting the appropriate scale word between those chunks.

So `1234567890` should yield `'1 billion 234 million 567 thousand 890'`

The program must also report any values that are out of range. It's fine to stop at "trillion".

## Step 4

Put it all together to get nothing but plain English.

`12345` should give `twelve thousand three hundred forty-five`.

The program must also report any values that are out of range.

### Extensions

Use *and* (correctly) when spelling out the number in English:

- 14 becomes "fourteen".
- 100 becomes "one hundred".
- 120 becomes "one hundred and twenty".
- 1002 becomes "one thousand and two".
- 1323 becomes "one thousand three hundred and twenty-three".

## Rust Specific Exercise Notes

This is slightly changed in the Rust version, compared to other language versions of this exercise. Instead of requiring you to return errors for out of range, we are using Rust's strong type system to limit input. It is much easier to make a function deal with all valid inputs, rather than requiring the user of your module to handle errors.

There is a -1 version of a test case, but it is commented out. If your function is implemented properly, the -1 test case should not compile.

Adding 'and' into number text has not been implemented in test cases.

### Extension

Add capability of converting up to the max value for u64: 9,223,372,036,854,775,807.

For hints at the output this should have, look at the last test case

```rust
mod say{
    pub fn encode(n: u64)->String{
        match n {
            0 => String::from("zero"),
            1 => String::from("one"),
            2 => String::from("two"),
            3 => String::from("three"),
            4 => String::from("four"),
            5 => String::from("five"),
            6 => String::from("six"),
            7 => String::from("seven"),
            8 => String::from("eight"),
            9 => String::from("nine"),
            10 => String::from("ten"),
            11 => String::from("eleven"),
            12 => String::from("twelve"),
            13 => String::from("thirteen"),
            14 => String::from("fourteen"),
            15 => String::from("fifteen"),
            16 => String::from("sixteen"),
            17 => String::from("seventeen"),
            18 => String::from("eighteen"),
            19 => String::from("nineteen"),
            20 => String::from("twenty"),
            30 => String::from("thirty"),
            40 => String::from("forty"),
            50 => String::from("fifty"),
            60 => String::from("sixty"),
            70 => String::from("seventy"),
            80 => String::from("eighty"),
            90 => String::from("ninety"),
            20..=90 => format!("{}-{}", encode(10*(n/10)), encode(n%10)),
            100..=999 => format!("{} hundred {}", encode(n/100), encode(n%100)),
            1000..=999_999 => format!("{} thousand {}", encode(n/1000), encode(n%1000)),
            1_000_000..=999_999_999 => format!("{} million {}", encode(n/1_000_000), encode(n%1_000_000)),
            1_000_000_000..=999_999_999_999 => format!("{} billion {}", encode(n/1_000_000_000), encode(n%1_000_000_000)),
            1_000_000_000_000..=999_999_999_999_999 => format!("{} trillion {}", encode(n/1_000_000_000_000), encode(n%1_000_000_000_000)),
            1_000_000_000_000_000..=999_999_999_999_999_999 => format!("{} quadrillion {}", encode(n/1_000_000_000_000_000), encode(n%1_000_000_000_000_000)),
            1_000_000_000_000_000_000..=u64::MAX => format!("{} quintillion {}", encode(n/1_000_000_000_000_000_000), encode(n%1_000_000_000_000_000_000)),
            _=>format!("")
        }.replace(" zero", "")
    }
}
```

