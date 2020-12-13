pub fn run() {
    let me: (&str, &str, i8) = ("Thomas", "Strasbourg", 23);

    println!("{} is from {} and is {}", me.0, me.1, me.2);
}