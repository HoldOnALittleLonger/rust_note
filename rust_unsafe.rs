unsafe fn unsafe_function() {
    println!("unsafe_function() is unsafe.");
}

static mut STATIC_MUT_VAR: u32 = 2;

unsafe trait UnsafeTrait {
    fn print_msg(&self) {
        println!("trait UnsafeTrait.");
    }
}

struct HasUnsafeStruct { }

unsafe impl UnsafeTrait for HasUnsafeStruct {
    fn print_msg(&self) {
        println!("type HasUnsafeTrait implemented UnsafeTrait.");
    }
}

union TheUnion {
    x: i32,
    y: u32,
    z: f32,
}

unsafe extern "C" {
    fn abs(i: i32) -> i32;
}

#[allow(static_mut_refs)]
fn main() {
    let u = TheUnion { x: -20 };
    unsafe {
        println!("TheUnion::x is {}", u.x);
        println!("TheUnion::y is {}", u.y);
        println!("TheUnion::z is {}", u.z);
    }

    unsafe {
        unsafe_function();
    }

    unsafe {
        println!("STATIC_MUT_VAR is {}", STATIC_MUT_VAR);
        STATIC_MUT_VAR += 2;
        println!("STATIC_MUT_VAR is {}", STATIC_MUT_VAR);
    }

    let unsafe_struct = HasUnsafeStruct { };
    unsafe_struct.print_msg();

    let mut v: i32 = 10;
    let rp_of_v = &raw mut v;
    unsafe {
        println!("raw pointer of v is {rp_of_v:?}");
        println!("v is {}", *rp_of_v);
        *rp_of_v += 2;
        println!("v is {}", *rp_of_v);
    }

    println!("invoke C function abs(@v)");
    unsafe {
        println!("abs -20 is {}", abs(-20));
    }

    // trigger SIGSEGV
//    let bad_addr = 0x07777bffe;
    let bad_rp = 0x07777bffe as *const i32;
    unsafe {
        println!("bad address is {bad_rp:?}");
        println!("trigger SIGSEGV");
        println!("{}", *bad_rp);
    }

}
