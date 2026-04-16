pub mod cmd_parse;
pub mod file_io;
pub mod pattern_match;

#[cfg(test)]
mod tests {
    use super::*;
    use std::string;
    use std::vec;

    #[test]
    fn test_match_it() {
        let contents = String::from("\
Rust:
search and match it for one line,
print the result.
the usage is <program> pattern file-name
");
        println!("test contents is");
        println!("{contents}");

        let query = String::from("a");
        println!("test query is {query}");

        let ret = pattern_match::match_it(&query, &contents);
        assert_eq!(ret.len(), 2);
    }
}

