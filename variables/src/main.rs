use std::string;

fn main() {
    let x = 5;
    let mut y = 6;
    y = 10;
    println!("x is {x}.");
    println!("y is {y}.");

    // shadowing
    let x = x + 5;
    println!("x is {x}.");
    let x = y + 2;
    println!("x is {x}.");
    let x = String::from("Hello world");
    println!("x is {x}.");

    const CONSTANT_OBJ: u32 = 5 + 20;
    println!("CONSTANT_OBJ IS {CONSTANT_OBJ}.");

//    y = String::from("Hello world");
//    println!("y is {y}.");

    let mut_def = 2;
    let mut mut_def = 0;
    println!("mut_def is {mut_def} .");
}
