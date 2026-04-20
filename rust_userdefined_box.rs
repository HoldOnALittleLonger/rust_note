use std::string;
use std::ops::Deref;
use std::ops::DerefMut;

struct CustomSP<T>(T);

impl<T> CustomSP<T> {
    fn new(v: T) -> Self {
        Self(v)
    }
}

impl<T> Deref for CustomSP<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CustomSP<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn print_str(s: &str) {
    println!("{s}");
}

fn main() {
    let str_pointer = CustomSP::new(String::from("Custom smart pointer"));

    //             *(str_pointer.deref())
    println!("{}", *str_pointer);

    // Deref coercion.
    print_str(&str_pointer);

    let mut int_pointer = CustomSP::new(10);
    println!("inner is {}", *int_pointer);
    *int_pointer = 20;
    println!("inner is {}", *int_pointer);
}
