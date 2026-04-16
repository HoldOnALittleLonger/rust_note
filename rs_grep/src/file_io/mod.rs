use std::string;
use std::fs;

pub enum FSIO_EC {
    EREAD,
}

pub fn fsioec2i32(ec: &FSIO_EC) -> i32 {
    match ec {
        FSIO_EC::EREAD => -1,
        _ => -255,
    }
}

pub fn fsioec2str(ec: &FSIO_EC) -> &'static str {
    match ec {
        FSIO_EC::EREAD => "fs error: read file was failed",
        _ => "fs error: unknown error",
    }
}

pub fn read(fpath: String, mut sbuf: String) -> Result<String, FSIO_EC> {
    let io_ret = fs::read_to_string(fpath);
    let mut state = 0;

    match io_ret {
        Ok(contents) => sbuf = contents,
        Err(_) => state = -1,
    }

    if state < 0 {
        Err(FSIO_EC::EREAD)
    } else {
        Ok(sbuf)
    }
}
