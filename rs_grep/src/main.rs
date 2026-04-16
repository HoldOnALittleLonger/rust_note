use std::string;
use std::process;
use std::vec;
use rs_grep::*;

fn main() {
    let query: String;
    let f_path: String;
    match cmd_parse::parse_args() {
        Ok(pair) => {
            query = pair.0;
            f_path = pair.1;
        },
        Err(ec) => {
            println!("error code is {},{}.",
                     cmd_parse::cmdec2i32(&ec),
                     cmd_parse::cmdec2str(&ec));
            process::exit(cmd_parse::cmdec2i32(&ec));
        },
    }
    println!("match: {query}");
    println!("file path: {f_path}");

    let mut f_contents: String = String::new();
    match file_io::read(f_path, f_contents) {
        Ok(buf) => f_contents = buf,
        Err(ec) => {
            println!("error code is {},{}.",
                     file_io::fsioec2i32(&ec),
                     file_io::fsioec2str(&ec));
            process::exit(file_io::fsioec2i32(&ec));
        },
    }

//    println!("file contents: {f_contents}");

    let result = pattern_match::match_it(&query, &f_contents);
    for e in result {
        println!("{e}");
    }
}
