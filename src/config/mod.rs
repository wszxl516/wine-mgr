mod alias;
mod config;
mod env;
mod log;

pub use alias::Alias;
pub use config::Config;
pub use env::Env;
use clap::Parser;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::{fs, path};
use ::log::{debug, error, warn};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    ///program name, alias or path(unix path support only).
    #[clap(short, long, value_parser, default_value = "winecfg")]
    pub program: String,
    ///enable verbose info print on console if not run process background.
    ///Use the log=debug environment variable to view debug logs.
    #[clap(short, long, value_parser, default_value_t = false)]
    pub verbose: bool,
    ///toml config file path. Built-in is used by default.
    #[clap(short, long, value_parser, default_value = "config.json")]
    pub config: String,
    ///list program alias from config.
    #[clap(short, long, value_parser, default_value_t = false)]
    pub alias: bool,
    ///list active Wine process.
    #[clap(short, long, value_parser, default_value_t = false)]
    pub list: bool,
    ///run process background.
    #[clap(short, long, value_parser, default_value_t = false)]
    pub back: bool,
    ///kill process by name, kill all process with 'all'.
    #[clap(short, long, value_parser)]
    pub kill: Option<String>,
}

fn from_string(config: &str, section: &str) -> Result<HashMap<String, String>, ()> {
    let c: serde_json::error::Result<Map<String, Value>> = serde_json::from_str(config);
    match c {
        Ok(cc) => match cc.get(section) {
            None => Err(()),
            Some(ccc) => match ccc.as_object() {
                None => Err(()),
                Some(cccc) => Ok(cccc
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string().replace("\"", "")))
                    .collect::<HashMap<String, String>>()),
            },
        },
        Err(_) => Err(()),
    }
}


pub fn parse_config() -> (String, String, Config, Env, Alias, bool, bool, bool, bool, Option<String>) {
    log::init_logger();
    let args = Args::parse();
    let mut env = Env::default();
    let mut conf = Config::default();
    let mut alias = Alias::default();
    match path::Path::new(&args.config).exists() {
        true => {
            debug!("Config file: {}", &args.config);
            let mut config_str = String::new();
            let mut f = fs::File::open(&args.config).unwrap();
            f.read_to_string(&mut config_str).unwrap();
            match Env::from_string(config_str.as_str(), true) {
                Ok(e) => env = e,
                Err(_) => error!("parse {} failed, use default env.", &args.config),
            }
            match Config::from_string(config_str.as_str()) {
                Ok(c) => conf = c,
                Err(_) => error!("parse {} failed, use default config.", &args.config),
            }
            match Alias::from_string(config_str.as_str()) {
                Ok(a) => alias = a,
                Err(_) => error!("parse {} failed, use default alias.", &args.config),
            }
        }
        false => warn!("can not found {}, use default config.", &args.config),
    };

    let wine_bin = match env.get("WINE") {
        None => "wine".to_string(),
        Some(w) => w.clone(),
    };
    debug!("{}", env);
    debug!("{}", conf);
    debug!("{}", alias);
    let program = match alias.get(&args.program) {
        None => &args.program,
        Some(p) => {
            debug!("program {} alias to {}", args.program, p);
            p.as_str()
        }
    };
    let program_path = PathBuf::from(program);
    match program_path.is_file() {
        true => match fs::canonicalize(&program_path) {
            Ok(fp) => debug!("Program full path is {}", fp.display()),
            Err(_) => (),
        },
        false => {}
    };
    (wine_bin, String::from(program), conf, env, alias, args.verbose, args.back, args.alias, args.list, args.kill)
}