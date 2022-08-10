extern crate core;

mod config;
mod executor;

use config::ArgsConfig;
use executor::{executor, WineProc};
use log::info;
use nix::unistd;
use std::process;

fn main() {
    let mut args = ArgsConfig::new();
    args.show_alias.then(|| {
        args.alias.print_table();
        process::exit(0)
    });
    args.list_process.then(|| {
        WineProc::new().print_table();
        process::exit(0)
    });
    match args.kill {
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
    info!(
        "Wine bottle name: {}",
        args.conf.get("name").unwrap_or(&"noname".to_string())
    );
    info!("Wine prefix: {}", args.env.get("WINEPREFIX").unwrap());
    info!("Wine path: {}", args.wine_bin);
    info!("Program name: {}", args.program);
    info!("old log \"{}\" will be overwritten!", args.log_file);
    match args.background {
        true => {
            args.verbose = false;
            unistd::daemon(true, false).unwrap()
        }
        false => (),
    }
    executor(
        args.wine_bin.clone(),
        vec![args.wine_bin, args.program],
        args.env,
        args.log_file,
        args.verbose,
    )
}
