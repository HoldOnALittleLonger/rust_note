use std::vec;

trait CommonTrait {
    type RetType;
    fn common(&mut self) -> Self::RetType;
}

struct CustomStruct { }

impl CustomStruct {
    fn new() -> Self { Self { } }
}

impl CommonTrait for CustomStruct {
    type RetType = i32;
    fn common(&mut self) -> Self::RetType {
        2
    }
}

struct CustomStruct1 { }

impl CustomStruct1 {
    fn new() -> Self { Self { } }
}

impl CommonTrait for CustomStruct1 {
    type RetType = bool;
    fn common(&mut self) -> Self::RetType {
        true
    }
}

fn main() {
    let mut obj = CustomStruct { };
    let ret = obj.common();
    println!("@ret is {ret}");

    let mut collection: Vec<Box<dyn CommonTrait<RetType = bool>>> =
        vec![Box::new(CustomStruct1::new())];
    println!("value is {}", collection[0].common());

    let sp: Box<dyn CommonTrait<RetType = bool>> =
        Box::new(CustomStruct1::new());
}
