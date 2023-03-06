use super::super::config::Env;
use log::info;
use nix::unistd;
use std::ffi::{CStr, CString};
use std::io::Write;
use std::os::unix::io::AsRawFd;

pub fn executor(program: String, args: Vec<String>, envs: Env, log: String, show_log: bool) {
    let env_str = envs
        .get_map()
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join(" ");
    match show_log {
        true => {
            info!("Environment: {}\n", env_str);
            info!("Command: {}\n", args.join(" "));
        }
        false => {
            let mut fp = std::fs::File::create(log).unwrap();

            fp.write_fmt(format_args!("Environment: {}\n", env_str)).unwrap();
            fp.write_fmt(format_args!("Command: {}\n", args.join(" ")))
                .unwrap();
            let raw_fd = fp.as_raw_fd();
            unistd::dup2(raw_fd, 1).unwrap();
            unistd::dup2(raw_fd, 2).unwrap();
        }
    };
    let args = args
        .iter()
        .map(|s| CString::new(s.as_bytes()).unwrap().into_boxed_c_str())
        .collect::<Vec<Box<CStr>>>();
    unistd::execvpe(
        CString::new(program).unwrap().as_c_str(),
        args.as_slice(),
        envs.cstr().as_slice(),
    )
    .unwrap();
}
