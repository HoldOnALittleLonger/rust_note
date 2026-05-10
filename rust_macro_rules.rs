macro_rules! print_msg {
    ( $( $str:expr ) * ) => {
        $(
            println!("{}", $str);
        )*
    }
}

macro_rules! local_add {
    ( $lhs:expr, $rhs:expr ) => {
        $lhs + $rhs
    }
}

fn main() {
    print_msg!("print_msg macro" "another");
    println!("local_add(2, 4) = {}", local_add!(2, 4));
}
