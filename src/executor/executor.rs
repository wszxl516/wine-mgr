use super::super::config::Env;
use anyhow::Error;
use nix::unistd;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

pub fn executor<F>(
    program: &str,
    args: Vec<&str>,
    envs: Env,
    log: Arc<Mutex<F>>,
    show_log: bool,
) -> Result<i32, Error>
where
    F: Write + Send + 'static,
    Stdio: From<F>
{
    let mut child = Command::new(program)
        .args(args)
        .envs(envs.get_map())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdout = child.stdout.take().expect("Failed to open stdout");
    let mut stderr = child.stderr.take().expect("Failed to open stderr");
    let stdout_log = Arc::clone(&log);
    let stdout_thread = std::thread::spawn(move || {
        let mut out_buffer = [0u8; 256];
        loop {
            let out_len = stdout.read(&mut out_buffer).unwrap();
            if out_len != 0 {
                let str_buffer = String::from_utf8_lossy(&out_buffer[..out_len]);
                stdout_log
                    .lock()
                    .unwrap()
                    .write(str_buffer.as_bytes())
                    .unwrap();
                show_log.then(|| println!("{}", &str_buffer));
            }
            if out_len == 0 {
                break;
            }
        }
    });
    let stderr_log = Arc::clone(&log);
    let stderr_thread = std::thread::spawn(move || {
        let mut err_buffer = [0u8; 256];
        loop {
            let err_len = stderr.read(&mut err_buffer).unwrap();
            if err_len != 0 {
                let str_buffer = String::from_utf8_lossy(&err_buffer[..err_len]);
                stderr_log
                    .lock()
                    .unwrap()
                    .write_all(str_buffer.as_bytes())
                    .unwrap();
                show_log.then(|| println!("{}", str_buffer));
            }
            if err_len == 0 {
                break;
            }
        }
    });
    let code = child.wait()?;
    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();
    Ok(code.code().unwrap())
}

pub fn executor_back<F>(
    program: &str,
    args: Vec<&str>,
    envs: Env,
    log: Arc<Mutex<F>>,
    show_log: bool,
)
where
    F: Write + Send + 'static,
    Stdio: From<F>
{
    unistd::daemon(true, false).unwrap();
    executor(program, args, envs, log, show_log).unwrap();
}
