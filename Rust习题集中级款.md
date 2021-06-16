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

# Binary Search

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

