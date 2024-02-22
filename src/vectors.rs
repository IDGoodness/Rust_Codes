// Vectors - Resizable arrays


// use core::num;
use std::mem;

pub fn run () {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // Can be made mutable too

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    // numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get Vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Array occupies {} bytes.", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);

    let slice1: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice1);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);

}