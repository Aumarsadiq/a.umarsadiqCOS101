// Rust program to read the height of a person
// and then print if person is tall, dwarf,
// or average height person

use std::io:

fn main()
{
    let mut input= String::new();

    println!("\nEnter Your Height (in centimetres):");
    io::stdin().read_line(&mut input).expect("Not a valid string")