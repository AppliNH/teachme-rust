// Arrays are fixed list where elements are the SAME data types

pub fn run() {
    // Array of int32, fixed length of 5
    let mut numbers: [i32; 5] = [1,2,3,4,5]; // Will throw an exception if there aren't 5 values

    // Re-assign a value
    numbers[0] = 20;


    println!("{:?}", numbers);
    println!("Index 0 : {}", numbers[0]);


    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    // 20 bytes => 4 bytes per value
    // if you add "use std::meme" at the top of the file, then you can simply use "mem::" here

    // Get slice
    let slice: &[i32] = &numbers[0..2]; // index 2 not included
    println!("Slice: {:?}", slice);

}