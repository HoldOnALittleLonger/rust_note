use std::string;
use std::env;

pub enum CMD_EC {
    CMD_BADUSAGE,
    CMD_NOENV,
}

pub fn cmdec2i32(ec: &CMD_EC) -> i32 {
    match ec {
        CMD_EC::CMD_BADUSAGE => -1,
        CMD_EC::CMD_NOENV => -2,
        _ => -255,
    }
}

pub fn cmdec2str(ec: &CMD_EC) -> &'static str {
    match ec {
        CMD_EC::CMD_BADUSAGE => "cmd error: argument too less",
        CMD_EC::CMD_NOENV => "cmd error: no such environment variable",
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

pub fn getenv(env_name: &str) -> Result<String, CMD_EC> {
    let env_ret = env::var(env_name);

    if let Ok(val) = env_ret {
        Ok(val)
    } else {
        Err(CMD_EC::CMD_NOENV)
    }
}
