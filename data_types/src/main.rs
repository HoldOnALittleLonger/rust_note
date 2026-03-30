use std::io;
use std::string;

fn main() {
    let a: i32 = 2;
    let b: u64 = 4;

    println!("a is {a}.");
    println!("b is {b}.");

    let f1: f32 = 32.22;
    let f2: f64 = 128.22;
    println!("f1 is {f1}."); 
    println!("f2 is {f2}.");

    let c: char = 'C';
    println!("c is {c}.");

    let right: bool = true;
    let wrong: bool = false;
    println!("right is {right}.");
    println!("wrong is {wrong}.");

    let tup: (i32, bool, char, f32) = (16, false, 'X', 1.0);
    println!("tup.0 is {0}.", tup.0);
    println!("tup.1 is {0}.", tup.1);
    println!("tup.2 is {0}.", tup.2);
    println!("tup.3 is {0}.", tup.3);

    let the_array: [u8; 4] = [0, 1, 2, 3];
    println!("the_array[0] is {0}.", the_array[0]);
    println!("the_array[1] is {0}.", the_array[1]);
    println!("the_array[2] is {0}.", the_array[2]);
    println!("the_array[3] is {0}.", the_array[3]);

    let the_array1 = ['f'; 5];
    println!("the_array1[0] is {0}.", the_array1[0]);
    println!("the_array1[1] is {0}.", the_array1[1]);
    println!("the_array1[2] is {0}.", the_array1[2]);
    println!("the_array1[3] is {0}.", the_array1[3]);
    println!("the_array1[4] is {0}.", the_array1[4]);

    let prompt = String::from("Type index,the maximum index is 4: ");
    println!("{prompt}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read index.");

    let mut array_index: usize = input
        .trim()
        .parse()
        .expect("Index is invalid.");

//    let e = the_array1[10]; compile-time error,exceeded array boundary.

    println!("element at {0} is {1} .", array_index, the_array1[array_index]);


    let v = 2 * (3 + 4) / 8 + 4 - 2;
    println!("v is {v}.");
}
