use std::string;

fn borrow_it(v: &String) { }

fn borrow_it_mutable(v: &mut String) { }

fn main() {
    let mut x: Option<String> = Some(String::from("option"));
    let y = match x.as_ref() {
        None => String::from("None"),
        Some(v) => v.clone(),
    };
    println!("y is {y}");

    if let Some(v) = x.as_ref() {
        println!("x is {v}");
    }

    match x {
        None => panic!("none"),
        Some(ref v) => borrow_it(&v),
    }

    borrow_it(x.as_ref().unwrap());

    if let Some(v) = x.take() {
        borrow_it(&v);

        match x {
            None => println!("x become None"),
            Some(v) => println!("x still is {v}"),
        }
    }
}
