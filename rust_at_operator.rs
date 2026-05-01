fn main() {
    let array_obj: [u32; 4] = [0, 1, 2, 3];
    match array_obj {
        [0, .., v] => println!("last one is {v}"),
        _ => println!("nothing"),
    }

    let some_obj = Some(5);
    match some_obj {
        Some(v @ 0..=6) => println!("value {v} in range [0, 6]"),
        _ => println!("none"),
    }
    
    match some_obj {
        Some(0..=8) => match some_obj {
                           Some(v) => println!("v is {v}"),
                           None => println!("none"),
                       },
        _ => println!("none"),
    }

}
