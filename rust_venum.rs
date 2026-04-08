use std::{string, vec};

#[derive(Debug)]
enum ENUM {
    INT(i32),
    FLOAT(f32),
    TEXT(String),
}

fn print_enum(ropt: &Option<&ENUM>) {
    match ropt {
        None => return,
        Some(payload) => {
            match payload {
                ENUM::INT(e) => println!("{e}"),
                ENUM::FLOAT(e) => println!("{e}"),
                ENUM::TEXT(e) => println!("{e}"),
            }
        }
    }
}


fn main() {
    let mut venum: Vec<ENUM> = Vec::new();
    venum.push(ENUM::INT(2));
    venum.push(ENUM::FLOAT(3.2));
    venum.push(ENUM::TEXT(String::from("text")));
    println!("{venum:?}");

    print_enum(&venum.get(0));
    print_enum(&venum.get(1));
    print_enum(&venum.get(2));
}
