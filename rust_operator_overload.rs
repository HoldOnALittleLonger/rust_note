use std::string;
use std::ops::Add;

struct TheStruct { x: usize, }

impl Add<String> for TheStruct {
    type Output = usize;
    fn add(self, rhs: String) -> Self::Output {
        self.x + rhs.len()
    }
}

fn main() {
    let ts = TheStruct { x: 20, };
    let str = String::from("hello world");
    println!("@ts + @str is {}", ts + str);
}
