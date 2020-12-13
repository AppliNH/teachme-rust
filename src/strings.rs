pub fn run() {
    // Primitive str => Immutable fixed-length string in memory
    let hello = "Hello";

    // String => Growable, heap-allocated data structure 
    // (use when you need to modify or own string data)
    // (mut has nothing to do with that, it's String::from)
    let mut hello2 = String::from("Hello");

    // Get length
    println!("{} is {} characters long", hello, hello.len());

    // Concat a character or a str into an existing String
    // Only working if the initial variable is a String
    hello2.push('W');
    hello2.push_str("orld!");


    // Capacity in bytes (String)
    println!("Capacity is {}", hello2.capacity());

    // Check if empty
    println!("Is empty: {}", hello2.is_empty());

    // Contains substring
    println!("Contains 'World' : {}", hello2.contains("World"));

    // Replace
    println!("Replace 'World' by 'You' : {}", hello2.replace("World", "You"));


    // Loop through String by whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word)
    }

    // Create String with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len()); // Nothing happens here, since the test passes

    println!("{} and length is: {}", s, s.len());



}