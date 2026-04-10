struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }

    fn set_x(&mut self, v: T) {
        self.x = v;
    }

    fn set_y(&mut self, v: T) {
        self.y = v;
    }
}

impl Point<f32> {
    fn do_not_call(&self) {
        panic!("EINVAL");
    }
}


fn find_max<T: std::cmp::PartialOrd>(ary: &[T]) -> &T {
    let mut tmp: &T = &ary[0];
    for iter in ary {
        if tmp < iter {
            tmp = iter;
        }
    }
    tmp
}

fn main() {
    let num_set: [i32; 5] = [2, 5, 1, 6, 10];
    let ret = find_max(&num_set);
    println!("maximum value is {ret}");

    let mut p = Point::new(20, 30);
    println!("@p.x is {}", p.x());
    println!("@p.y is {}", p.y());
    p.set_x(30);
    p.set_y(40);
    println!("@p.x is {}", p.x());
    println!("@p.y is {}", p.y());

    let mut float_p = Point { x: 2.0, y: 3.0, };
    float_p.set_x(5.0);
    println!("@float_p.x is {}", float_p.x());
    float_p.do_not_call();
}
