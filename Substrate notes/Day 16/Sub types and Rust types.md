How do you convert between Substrate specific types and Rust primitive types?

```
// Working from u32 to Substrate specific types should be easy:
pub fn u32_to_balance(input: u32) -> T::Balance {
    input.into()
}

// For larger types, you need to handle the case where the Balance type for a runtime is smaller than what is available:
pub fn u64_to_balance_option(input: u64) -> Option<T::Balance> {
    input.try_into().ok()
}

// Forcely do. Only do it by making sure there won't leak
pub fn u64_to_balance_saturated(input: u64) -> T::Balance {
    input.saturated_into()
}

// When converting from T::Balance to a rust primitive, you need to also handle conversion between incompatible types:
pub fn balance_to_u64(input: T::Balance) -> Option<u64> {
    TryInto::<u64>::try_into(input).ok()
}

// Forcely do. Only do it by making sure there won't leak
pub fn balance_to_u64_saturated(input: T::Balance) -> u64 {
    input.saturated_into::<u64>()
}
```
