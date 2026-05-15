use std::string;

fn main() {
    let mut result = String::new();
    let mut str1 = String::from("Hello ");
    let str2 = String::from("World!");
    /*
     * fn Add::add(lhs: String, rhs: &str) -> String;
     */
    result = str1.clone() + &str2;
    println!("{result}");

    let mut result = String::new();

    /*
     * fn String::push_str(&mut self, str: &str);
     */
    result.push_str(&str1);
    result.push_str(&str2);
    println!("{result}");

    /* provide operator+=(&mut self, rhs: &str) */
    str1 += &str2;
    println!("{str1}");
}
