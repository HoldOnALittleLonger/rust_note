use std::string;

/*
 * add_one - compute x + 1
 * @x:       parameter
 * return:   x + 1               
 */
fn add_one(x: i32) -> i32 {
    x + 1 // return value is evaluated from the expression
}

/*
 * takes_ownership - takes ownership of an object passed by caller,
 *                   but without give ownership back
 * @s:               String object
 */
fn takes_ownership(s: String) {
    println!("have taken the ownership of String object {s} .");
}

/* takes ownership and give it back */
fn takes_ownership_giveback(s: String) -> String {
    s
}

/*
 * borrow_immutable_ownership - borrow immutable ownership of an object
 * @s:                          immutable reference to an String object                            
 */
fn borrow_immutable_ownership(s: &String) {
    println!("borrowed ownership,s is {s} .");
}

/*
 * borrow_mutable_ownership - borrow mutable ownership of an object
 * @s:                        mutable reference to an String object
 */
fn borrow_mutable_ownership(s: &mut String) {
    println!("original string is {s} .");
    s.push_str(" - push str");
    println!("now is {s} .");
}

/*
 * make_String_slice - makeup a slice from String and return it
 * @s:                 immutable reference
 * return:             slice refers to entire String
 */
fn make_String_slice(s: &String) -> &str {
    &s[..=3]
}

fn main() {
    let mut local_string = String::from("rust_function");
    let x = 5;
    
    println!("@local_string is {local_string} .");
    println!("@x is {x}.");

    let y = add_one(x);
    println!("@y = @x + 1 is {y}.");

    let tmp_s_drop_ownership = String::new();
    takes_ownership(tmp_s_drop_ownership);

    let tmp_s_prev_owner = String::from("takes ownership and give it back");
    println!("original owner is @tmp_s_prev_owner - {tmp_s_prev_owner}.");
    let tmp_s_current_owner = takes_ownership_giveback(tmp_s_prev_owner);
    println!("now owner is @tmp_s_current_owner - {tmp_s_current_owner}.");

    borrow_immutable_ownership(&local_string);
    
    println!("ready to borrow mutable ownership.");
    println!("@local_string is {local_string}.");
    borrow_mutable_ownership(&mut local_string);
    println!("@local_string is {local_string}.");

    println!("ready to make slice for @local_string.");
    let the_slice = make_String_slice(&local_string);
    println!("slice is {the_slice}.");

    println!("program end.");
}
