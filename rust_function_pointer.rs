fn need_callable<T: Fn(u32)>(tf: T) {
    tf(2);
}

fn print_msg() {
    println!("print_msg");
}

fn need_callable_params(v: u32, tf: fn(u32)) {
    tf(v);
}

fn print_value(v: u32) {
    println!("value is {v}");
}

fn main() {
    let the_closure = |_| println!("the_closure");
//    need_callable(print_msg);
    need_callable(the_closure);
    need_callable_params(2, print_value);
}
