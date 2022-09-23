// Arrays -Fixed list where elements are the same data types
use std::mem;
pub fn run_array(){
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    // Re assign a value
    numbers[0] = -1;

    println!("Reassign a value {:?}", numbers);

    // Get single value
    println!("Get single value {}", numbers[1]);

    // Get array length
    println!("Get array length {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);
}
