// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run_var(){
    let name = "Tracy";
    let mut age = 28;
    println!("My name is {} and I am {}", name, age);
    age = 18;
    println!("My name is {}", name);
    println!("My name is {} and I am {}", name, age);

    // Define constants
    const ID: i32 = 001;
    println!("ID {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Tracy", 28);
    println!("{} is {}", my_name, my_age);
}