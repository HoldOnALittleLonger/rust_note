use std::string;
use std::fmt;

struct Text {
    text: String,
}

trait CopyAble {
    fn copy_able(&self) -> bool;
    fn copy_this(&self) -> Self;
}

impl CopyAble for Text {
    fn copy_able(&self) -> bool {
        true
    }
    
    fn copy_this(&self) -> Self {
        Text {
            text: self.text.clone(),
        }
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}


fn do_copy<T: CopyAble>(org: &T) -> Result<T, i32> {
    if org.copy_able() {
        Ok(org.copy_this())
    } else {
        Err(-1)
    }
}

fn copy_and_print<T>(p: &T)
where
    T: CopyAble + fmt::Display,
{
    let tmp = do_copy(p).unwrap();
    println!("copied temp object is {}", tmp);
}

struct Ref<'a, T> {
    r: &'a T,
}

impl<'a, T> Ref<'a, T> {
    fn get(&self) -> &T {
        self.r
    }
}

fn main() {
    let x = 10;
    let x_ref = Ref::<i32> {
        r: &x,
    };
    println!("x is {}", x_ref.get());


    let text1: Text = Text {
        text: String::from("text1"),
    };

    println!("text1 is {}", text1.text);
    copy_and_print(&text1);

    let ret = do_copy(&text1);
    let text2: Text;

    match ret {
        Ok(c) => {
            text2 = c;
            println!("text2 is {}", text2.text);
        },
        Err(e) => println!("failed to copy,error code {e}"),
    }
}
