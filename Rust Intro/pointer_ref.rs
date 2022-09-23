// Reference pointers - Point to a resource in memory
pub fn run_pointer_ref(){
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("arrays' values: {:?}", (arr1, arr2));

    // with non-primitives, if you assign another variable to a piece of data, the first variable
    // will no longer hold that value, you'll need to use a reference (&) to point to the resource

    // Vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("vetors' values: {:?}", (&vec1, vec2));
}