use std::string;

#[derive(Debug)]
enum Variants {
    INVALID,
    VSTRING(String),
    VSINT(i32),
    VFLOAT(f32),
    OTHER,
}

fn test_match(v: &Variants) {
    match v {
        Variants::INVALID =>
            println!("invalid state."),
        Variants::VSTRING(s) =>
            println!("string - {s}."),
        Variants::VSINT(i) =>
            println!("signed integer - {i}."),
        Variants::VFLOAT(f) =>
            println!("floating-point - {f}."),
        _ => println!("unknown type {v:?}."),
    }
}

fn main() {
    let mut vr = Variants::INVALID;
    test_match(&vr);
    vr = Variants::VSTRING(String::from("string"));
    
    // we borrow ownership of String object at there,
    // instead move semantics.
    if let Variants::VSTRING(s) = &vr {
        println!("@vr is string type - {s}.");
    }

    test_match(&vr);
    vr = Variants::VSINT(32);

    let Variants::VSINT(i) = vr else {
        println!("@vr is not signed integer.");
        return;
    };
    println!("let...else binding @i to {i}.");

    test_match(&vr);
    vr = Variants::VFLOAT(3.2);
    test_match(&vr);
    vr = Variants::OTHER;
    test_match(&vr);
}
