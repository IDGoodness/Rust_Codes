pub fn run() {
    // Print to console
    println!("Hello, world!");

    // Basic Formatting
    println!("Number: {}", 1);

    println!("{} is from Nigeria and {} is {} years old.", "Goodness", "Goodness", "12");
    
    // The above line can now be written as;
    println!("{0} is from Nigeria and {0} is {1} years old.", "Goodness", "12");

    // Positional Argument
    println!("{0} is from {1} and {0} likes to {2}", "Goodness", "Mars", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity = "chess");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}