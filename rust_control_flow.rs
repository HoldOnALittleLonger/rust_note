fn main() {
    loop {
        println!("this is a loop,and will be broken.");
        break;
    }

    'loop_1: loop {
        println!("this is a labeled loop 'loop_1.");

        'loop_2: loop {
            println!("this is a labeled loop 'loop_2,is nested in 'loop_1.");

            'loop_3: loop {
                println!("this is a labeld loop 'loop_3,is nested in 'loop_2.");
                println!("break to 'loop_2.");
                break 'loop_2;
            }

            println!("break 'loop_2.");
            break;
        }
        println!("break 'loop_1.");
        break;
    }

    let mut x = 0;
    println!("do while cycle,stop when @x >= 2.");
    while x < 2 {
        println!("x is {x}.");
        x += 1;
    }
              
    println!("do for cycle,traverse range [0, 5).");
    for v in 0..5 {
        println!("current number is {v}.");
    }

    println!("if expression.");
    let mut cond: bool = true;
    if cond {
        println!("@cond is true.");
    }

    cond = false;
    if !cond {
        println!("@cond is false.");
    }
}
