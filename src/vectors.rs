
use std::mem;

// Vectors are kind of arrays with dynamic length.

pub fn run() {
    // Array of int32, fixed length of 5
    let mut numbers: Vec<i32> = vec![1,2,3,4,5]; // Will throw an exception if there aren't 5 values

    // Re-assign a value
    numbers[0] = 20;

    // Push to vector (something you can't do with arrays)
    numbers.push(6);

    // remove value from vector
    numbers.remove(0);


    println!("{:?}", numbers);
    println!("Index 0 : {}", numbers[0]);

    println!("Vec occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2]; // index 2 not included
    println!("Slice: {:?}", slice);


    // Loop through vector
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop through and modify values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("New numbers: {:?}", numbers);


}