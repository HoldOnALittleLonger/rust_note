/*
 * Get usage through
 *     execute "rs_grep" without any command line arguments.
 * I just used rust module to separate APIs need by this program,
 * no object-oriented design.
 */
use std::string;
use std::process;
use std::vec;
use rs_grep::*;

fn print_help_message() {
    println!("Usage: rs_grep <pattern> <filename>");
    println!("Execute this program with a wrong way will");
    println!("print this message.");
    println!("------------------------------------------");
    println!("Environment variable: ");
    println!("IGNORE_CASE => do insensitive searching");
    println!("RS_GREP_PRINT_DEBUG => print some debug info");
}

fn main() {
    let mut query: String;
    let f_path: String;

    match cmd_parse::parse_args() {
        Ok(pair) => {
            query = pair.0;
            f_path = pair.1;
        },
        Err(ec) => {
            eprintln!("error code is {},{}.",
                     cmd_parse::cmdec2i32(&ec),
                     cmd_parse::cmdec2str(&ec));
            print_help_message();
            process::exit(cmd_parse::cmdec2i32(&ec));
        },
    }

    if cmd_parse::getenv("RS_GREP_PRINT_DEBUG").is_ok() {
        println!("match: {query}");
        println!("file path: {f_path}");
    }

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

    let result: Vec<&str>;
    if cmd_parse::getenv("IGNORE_CASE").is_ok() {
        query = query.to_lowercase();
        result = pattern_match::match_it_case_insensitive(&query, &f_contents);
    } else {
        result = pattern_match::match_it(&query, &f_contents);
    }

    for e in result {
        println!("{e}");
    }
}
