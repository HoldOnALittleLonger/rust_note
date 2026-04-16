use std::vec;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    dbg!(args);
    let tmp = env::args();
    let _: () = tmp;
}
