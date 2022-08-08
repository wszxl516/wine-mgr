use crate::config::from_string;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
const DEFAULT_CONF: [(&str, &str); 2] = [("name", "windows7"), ("log", "wine-mgr.log")];

pub struct Config(HashMap<String, String>);
impl Config {
    pub fn from_string(c: &str) -> Result<Self, ()> {
        match from_string(c, "config") {
            Ok(c) => Ok(Self { 0: c }),
            Err(_) => Err(()),
        }
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
}
impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut fmt_str = String::new();
        fmt_str.push_str("Config { ");
        for item in &self.0 {
            fmt_str.push_str(&*format!("{}=\"{}\", ", item.0, item.1))
        }
        fmt_str.push_str(" }");
        write!(f, "{}", fmt_str)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            0: HashMap::from(DEFAULT_CONF.map(|(k, v)| (k.to_string(), v.to_string()))),
        }
    }
}
