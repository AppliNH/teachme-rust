// Variables are immutable by default !
// use "mut" to make them mutable.
// Variables are block-scoped

pub fn run() {

    let activity = "guitar"; // activity can't be reassigned

    println!("I can play {}", activity);

    let mut languages = "french";

    println!("I can speak {}", languages);

    languages = "french and english";

    println!("Edit: I can speak {}", languages);

    // Constant: you *have* to explicitly define the type (you can also do it with let)
    const ID: i32 = 201;
    println!("ID: {}", ID);

    // Declare and assign multiple variables
    // Declaring a var as "_var" allows to disable the warnings about a declared varialbe not being used
    let (_one, _two) = ("one", 2);





}