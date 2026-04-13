use std::string;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn multi(lhs: u64, rhs: u64) -> u64 {
    lhs * rhs
}

pub fn will_panic() {
    panic!("just panic");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_multi() {
        let r = multi(5, 5);
        let emsg = String::from("test message.");
        assert_eq!(r, 10, "Error Message - {}", emsg);
    }

    #[test]
    #[should_panic(expected = "just panic")]
    fn test_will_panic() {
        will_panic();
    }

    #[test]
    fn test_return_Result() -> Result<(), String> {
        Err(String::from("test_return_Result - demo"))
    }
}
