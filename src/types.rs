/*Datatypes
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, (number of bits they take in memory.)

Floats: f32, f64.

Boolean: (bool)

Characters: (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.


pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 42535362526;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    // let is_active = true; can also by written as:
    let is_active: bool = true;

    println!("{:?}", (x, y, z, is_active));

    // Get boolean from expression
    let is_greater: bool = 10 < 5;
    println!("{:?}", (x, y, z, is_active, is_greater));

    // Character: char, has to be in single quotes. And char is not string, char is a single character like 'a'
    let a1 = 'a';
    let face1 = '\u{1F600}';
    let face2 = '\u{1F601}';
    let face3 = '\u{1F602}';
    let face4 = '\u{1F603}';
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face1, face2, face3, face4));
}