use std::string;
use std::vec;

fn main() {
    let text = String::from("local text");
    let mut texts: Vec<String> = Vec::new();

    // immutable borrow
    println!("local @text is {text}");
    println!("local @texts is {texts:?}");

    // mutable borrow
    let mut FnMut_Closure = |the_str: String| {
        println!("FnMut_Closure");
        texts.push(the_str);
    };
    FnMut_Closure(text.clone());

    // immutable borrow
    println!("local @text is {text}");
    println!("local @texts is {texts:?}");

    // move semantics,transfer ownership
    let FnOnce_Closure = move || {
        println!("FnOnce_Closure");
        println!("local @text is {text}");
        println!("ready to drop @text");
    };
    FnOnce_Closure();

    // immutable borrow
    let Fn_Closure = || {
        println!("Fn_Closure");
        println!("local @texts is {texts:?}");
    };
    Fn_Closure();
}
