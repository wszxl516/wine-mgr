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
    let program = std::path::Path::new(&args.program);
    let wine_args = if !program.exists(){
        args.program.split(" ").map(|x|x.to_string()).collect::<Vec<String>>()
    }
    else {
        vec![args.program]
    };
    info!("Program: {:?}", &wine_args);
    info!("old log \"{}\" will be overwritten!", args.log_file);
    match args.background {
        true => {
            args.verbose = false;
            unistd::daemon(true, false).unwrap()
        }
        false => (),
    }
    let mut executor_args = vec![args.wine_bin.clone()];
    executor_args.extend(wine_args);
    executor(
        args.wine_bin.clone(),
        executor_args,
        args.env,
        args.log_file,
        args.verbose,
    )
}
