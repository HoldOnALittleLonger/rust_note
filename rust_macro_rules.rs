macro_rules! print_msg {
    ( $( $str:expr ) * ) => {
        $(
            println!("{}", $str);
        )*
    }
}

fn main() {
    print_msg!("print_msg macro" "another");
}
