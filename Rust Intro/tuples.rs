// tuples group together values of different types
// Max 12 elements

pub fn run_tuples(){
    let person: (&str, &str, i8) = ("Tracy", "Blainville", 28);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}