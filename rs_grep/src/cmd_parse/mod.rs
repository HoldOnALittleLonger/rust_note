use std::string;
use std::env;

pub enum CMD_EC {
    CMD_BADUSAGE,
}

pub fn cmdec2i32(ec: &CMD_EC) -> i32 {
    match ec {
        CMD_EC::CMD_BADUSAGE => -1,
        _ => -255,
    }
}

pub fn cmdec2str(ec: &CMD_EC) -> &'static str {
    match ec {
        CMD_EC::CMD_BADUSAGE => "cmd error: argument too less",
        _ => "cmd error: unknown error",
    }
}

pub fn parse_args() -> Result<(String, String), CMD_EC> {
    let cmd_args: Vec<String> = env::args().collect();
    if cmd_args.len() < 3 {
        return Err(CMD_EC::CMD_BADUSAGE);
    }
    let m = cmd_args[1].clone();
    let f = cmd_args[2].clone();
    Ok((m, f))
}
