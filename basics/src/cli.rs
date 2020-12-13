use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    
    println!("Args: {:?}", args);


    if args.len() > 1 {
        // .clone() is required since the args var is a non-primitive.
        // Value could be moved, but in practice you can't move out of cli arguments
        // Cloning basically allows to copy the value, it's an expensive operation.
        let command = args[1].clone();
    
        println!("Command is {}", command);

        if command == "hello" {
            println!("Hi you, how are u doing ?");
        }
    }
}