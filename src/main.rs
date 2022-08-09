extern crate core;

mod config;
mod executor;

use config::parse_config;
use executor::{executor, WineProc};
use log::info;
use nix::unistd;
use std::process;

fn main() {
    let (wine_bin, program, conf, env, alias_data, mut verbose, back, alias, list, kill) =
        parse_config();
    alias.then(|| {
        alias_data.print_table();
        process::exit(0)
    });
    list.then(|| {
        WineProc::new().print_table();
        process::exit(0)
    });
    match kill {
        None => {}
        Some(name) => match name.as_str() {
            "all" => {
                WineProc::new().kill_all();
                process::exit(0)
            }
            _ => {
                WineProc::new().kill_by_name(name.as_str());
                process::exit(0)
            }
        },
    }
    info!("Wine path: {}", wine_bin);
    info!(
        "Wine bottle name: {}",
        conf.get("name").unwrap_or(&"noname".to_string())
    );
    info!("Program name: {}", program);
    let log_file = conf
        .get("log")
        .unwrap_or(&"out.log".to_string())
        .to_string();
    info!("old logs({}) will be overwritten", log_file.clone());
    match back {
        true => {
            verbose = false;
            unistd::daemon(true, false).unwrap()
        }
        false => (),
    }
    executor(
        wine_bin.clone(),
        vec![wine_bin, program],
        env,
        log_file,
        verbose,
    )
}
