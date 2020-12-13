pub fn run() {

    // Primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("{:?}", (arr1, arr2)); // This will print two times the same array :)


    // With non-primitives, assigning another variable to a piece of data of
    // a first variable, it will cause the first variable to no longer hold the value.
    
    // You'll need to use a reference (&) to point to the resource

    // Vector
    let vec1 = vec![1,2,3];
    let vec2 = vec1;

    //println!("{:?}", (vec1, vec2)); // This will cause an error
    // ... because vec1 was moved to vec2 (let vec2 = vec1;)

    let vec3 = &vec2; // assigning by reference
    println!("{:?}", (&vec2, vec3)); // This won't cause an error

}   