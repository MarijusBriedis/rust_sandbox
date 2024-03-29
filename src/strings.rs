// Primitive str = immutable fixed-lenght string somewhere in memory
// string = growable, heap-allocated data structure - use when You need to modify or own string data

pub fn run () {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push for one character only
    hello.push('W'); 

    // Push string
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through strings by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    //println!("{}", hello)
}