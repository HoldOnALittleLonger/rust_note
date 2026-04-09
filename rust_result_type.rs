use std::fs::File;

fn open_file_propagate_err(filename: &str) -> Result<File, i32> {
    let state = File::open(filename);
    match state {
        Ok(handle) => Ok(handle),
        Err(_) => Err(-1),
    }
}

fn open_file_propagate_err_qm(filename: &str) -> Result<File, std::io::Error> {
    let handle = File::open(filename)?;
    Ok(handle)
}

fn main() {
    println!("ret is {ret:?}");

    let fpath: &str = "./zero";
/*
    let ret = match open_file_propagate_err(fpath) {
        Ok(f) => f,
        Err(e) => {
            println!("error code is {e}");
            panic!("no such file");
        },
    };
    println!("ret is {ret:?}");
*/

    let ret_qm = open_file_propagate_err_qm(fpath).unwrap();
}
