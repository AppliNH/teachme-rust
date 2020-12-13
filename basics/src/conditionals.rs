pub fn run() {
    let age = 23;
    
    // If / Else
    if age > 18 {
        println!("You're allowed to drink, in Europe, cause you're {}", age);
    } else {
        println!("No beer for you boi");
    }


    // Short if
    let is_major = if age >= 18 { true } else {false};
    println!("Is major : {}", is_major);



}