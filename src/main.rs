mod config;
mod executor;

use crate::executor::executor_back;
use config::parse_config;
use executor::{executor, WineProc};
use log::info;
use std::process;
use std::sync::{Arc, Mutex};

fn main() {
    let (wine_bin, program, conf, env, alias_data, log, debug, back, alias, list, kill) =
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
        }
    }
    info!("Wine path: {}", wine_bin);
    info!(
        "Wine bottle name: {}",
        conf.get("name").unwrap_or(&"noname".to_string())
    );
    info!("Program name: {}", program);
    info!(
        "old logs({}) will be overwritten",
        conf.get("log").unwrap_or(&"out.log".to_string())
    );
    match back {
        true => executor_back(
            wine_bin.as_str(),
            vec![program.as_str()],
            env,
            Arc::new(Mutex::new(log)),
            debug,
        ),
        false => {
            executor(
                wine_bin.as_str(),
                vec![program.as_str()],
                env,
                Arc::new(Mutex::new(log)),
                debug,
            )
            .unwrap();
            ()
        }
    }
}
