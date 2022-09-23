pub fn run_print(){
    // Basic Formatting
    println!("This print is coming from the print file");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Tracy", "Blainville", "Play games"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Tracy",
        activity = "games"
    );

    // Placeholder Traits
    println!(
        "Binary: {:b} Hex: {:x} Octal {:o}",
        10, 10, 10
    );

    // Placeholder for debug Trait
    println!("{:?}", (1, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}