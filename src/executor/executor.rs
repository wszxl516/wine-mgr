use super::super::config::Env;
use nix::unistd;
use std::ffi::{CStr, CString};
use std::os::unix::io::AsRawFd;

pub fn executor(program: String, args: Vec<String>, envs: Env, log: String, show_log: bool) {
    match show_log {
        true => {}
        false => {
            let fp = std::fs::File::create(log).unwrap();
            let raw_fd = fp.as_raw_fd();
            unistd::dup2(raw_fd, 1).unwrap();
            unistd::dup2(raw_fd, 2).unwrap();
        }
    };
    let args = args
        .iter()
        .map(|s| CString::new(s.as_bytes()).unwrap().into_boxed_c_str())
        .collect::<Vec<Box<CStr>>>();
    unistd::execve(
        CString::new(program).unwrap().as_c_str(),
        args.as_slice(),
        envs.cstr().as_slice(),
    )
    .unwrap();
}
