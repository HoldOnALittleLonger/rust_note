struct TheStruct { }

trait GTrait<T> {
    fn do_something(&self) -> T;
}

impl GTrait<i32> for TheStruct {
    fn do_something(&self) -> i32 { return -2; }
}

impl GTrait<u32> for TheStruct {
    fn do_something(&self) -> u32 { return 2_u32; }
}

fn main() {
    let st = TheStruct { };
    println!("{}", <TheStruct as GTrait<u32>>::do_something(&st));
    println!("{}", <TheStruct as GTrait<i32>>::do_something(&st));
}
