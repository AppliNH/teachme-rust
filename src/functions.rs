pub fn run() {

    greeting("Hi", "Thomas");
    let r  = add(1,2);
    println!("Sum is {}", r);

    // Closure function
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("Closure sum : {}", add_nums(3,4));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {} !", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 // This is what will be returned !
    // Don't put any semi-colon for the line you'll return
}