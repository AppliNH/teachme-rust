pub fn run() {
    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Count: {}", count);


        if count == 10 {
            // Break out of the loop
            break;
        }
    }

    count=0;

    // While loop
    while count <= 100 {
        if count % 15 == 0 {
            println!("Divisible by 15")
        }
        count +=1;
    }

    // For range
    for x in 0..100 {
        if x % 10 == 0 {
            println!("yay")
        }
    }
}