
#[derive(Debug)]
struct ColorNumbers(u32, u32, u32);

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
    color: ColorNumbers
}

impl Rectangle {
    fn length(&self) -> u32 {
        self.length
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn set_length(&mut self, v: u32) {
        self.length = v;
    }

    fn set_width(&mut self, v: u32) {
        self.width = v;
    }

    fn color(&self) -> &ColorNumbers {
        &self.color
    }

    fn set_color(&mut self, new_color: ColorNumbers) {
        self.color = new_color;
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn new(length: u32, width: u32, color: ColorNumbers) -> Self {
        Self {
            length,
            width,
            color,
        }
    }
}

fn main() {
    let mut rec: Rectangle = Rectangle {
        length: dbg!(8),
        width: dbg!(4),
        color: dbg!(ColorNumbers(176, 224, 230)),
    };
    let rec_color = rec.color();

    println!("Debug info: rec = {rec:?}");

    println!("@rec.length is {}", rec.length());
    println!("@rec.width is {}", rec.width());
    println!("@rec.color is {},{},{}", rec_color.0, rec_color.1, rec_color.2);
    println!("area of @rec is {}", rec.area());

    println!("reset @rec.");
    rec.set_length(20);
    rec.set_width(10);
    rec.set_color(ColorNumbers(230, 230, 250));
    let rec_color = rec.color();

    println!("Debug info: rec = {rec:?}");

    println!("@rec.length is {}", rec.length());
    println!("@rec.width is {}", rec.width());
    println!("@rec.color is {},{},{}", rec_color.0, rec_color.1, rec_color.2);
    println!("area of @rec is {}", rec.area());

    println!("create a new Rectangle object via new() method.");
    let rec1 = Rectangle::new(2, 2, ColorNumbers(100, 149, 237));
    let rec1_color = rec1.color();
                              

    println!("Debug info: rec1 = {rec1:?}");
    println!("@rec1.length is {}", rec1.length());
    println!("@rec1.width is {}", rec1.width());
    println!("@rec1.color is {},{},{}", rec1_color.0, rec1_color.1, rec1_color.2);
    println!("area of @rec1 is {}", rec1.area());

    println!("create a new Rectangle object is part updated from @rec.");
    let rec2 = Rectangle {
        color: ColorNumbers(240, 248, 255),
        ..rec
    };
    let rec2_color = rec2.color();

    println!("Debug info: rec = {rec:?}");

    println!("@rec.length is {}", rec.length());
    println!("@rec.width is {}", rec.width());
    println!("@rec.color is {},{},{}", rec_color.0, rec_color.1, rec_color.2);
    println!("area of @rec is {}", rec.area());

    println!("Debug info: rec2 = {rec2:?}");

    println!("@rec2.length is {}", rec2.length());
    println!("@rec2.width is {}", rec2.width());
    println!("@rec2.color is {},{},{}", rec2_color.0, rec2_color.1, rec2_color.2);
    println!("area of @rec2 is {}", rec2.area());
}
