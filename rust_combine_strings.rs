use std::string;

fn main() {
    let s1 = String::from("Hello ");
    println!("{s1}");
    let s2 = String::from("World");
    println!("{s2}");
    let s3 = s1 + &s2;
    println!("{s3}");

    let years = String::from("YYYY");
    println!("{years}");
    let monthes = String::from("MM");
    println!("{monthes}");
    let days = String::from("DD");
    println!("{days}");

    let ymd = format!("{years}-{monthes}-{days}");
    println!("{ymd}");
}
