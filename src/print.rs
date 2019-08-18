pub fn run() {
    // Print to console
    println!("Hello world from the print.rs file!");
    println!("Number: {}", 1);

    // Basic formatting
    println!("{} is from {}", "Marijus", "Jurbarkas");

    // Positional arguments
    println!("{0} is form {1} and {0} likes to {2}", "Marijus", "Jurbarkas", "code");

    // Name arguments
    println!("{name} likes to play {activity}", name = "Marijus", activity = "basketball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
