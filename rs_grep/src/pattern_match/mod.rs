use std::vec;
use std::string;

pub fn match_it<'a>(query: &String, contents: &'a String) -> Vec<&'a str> {
    let mut matched: Vec<&'a str> = Vec::new();

    for one_line in contents.lines() {
        if one_line.contains(query) {
            matched.push(&one_line[..]);
        }
    }

    matched
}
