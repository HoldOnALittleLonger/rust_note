use std::vec;
use std::string;

fn search_it<'a>(q: &String, contents: &'a String, lowercase: bool)
-> Vec<&'a str> {
    let mut matched: Vec<&'a str> = Vec::new();
    for one_line in contents.lines() {
        if lowercase {
            if one_line.to_lowercase().contains(q) {
                matched.push(&one_line[..]);
            }
        } else {
            if one_line.contains(q) {
                matched.push(&one_line[..]);
            }
        }
    }

    matched
}


pub fn match_it<'a>(query: &String, contents: &'a String) -> Vec<&'a str> {
    search_it(query, contents, false)
}

pub fn match_it_case_insensitive<'a>(query: &String, contents: &'a String)
-> Vec<&'a str> {
    search_it(query, contents, true)
}
