#![allow(non_snake_case)]
#![allow(dead_code)]
use crate::config::from_string;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt::{Display, Formatter};

const DEFAULT_ENV: [(&str, &str); 6] = [
    ("WINEPREFIX", "/opt/windows/windows7"),
    ("WINEARCH", "win64"),
    ("DXVK_HUD", "1"),
    //("WINEDEBUG", "warn+all"),
    ("DISPLAY", ":0.0"),
    (
        "Path",
        r#"c:\\windows;c:\\windows\\system;c:\\windows\\syswow64;c:\\windows\\system32"#,
    ),
    ("VK_ICD_FILENAMES", ""),
];

const ENV_KEY: &str = "env";

pub struct Env(HashMap<String, String>);
impl Env {
    pub fn get_map(&self) -> &HashMap<String, String> {
        &self.0
    }
    pub fn from_string(c: &str, with_sys_env: bool) -> Result<Env, ()> {
        match from_string(c, ENV_KEY) {
            Ok(mut e) => {
                with_sys_env
                    .then(|| e.extend(std::env::vars().collect::<HashMap<String, String>>()));
                Ok(Env { 0: e })
            }
            _ => Err(()),
        }
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
    pub fn cstr(&self) -> Vec<Box<CStr>> {
        self.0
            .iter()
            .map(|(k, v)| {
                CString::new(format!("{}={}", k, v))
                    .unwrap()
                    .into_boxed_c_str()
            })
            .collect::<Vec<Box<CStr>>>()
    }
}

impl Default for Env {
    fn default() -> Self {
        Env {
            0: HashMap::from(DEFAULT_ENV.map(|(k, v)| (k.to_string(), v.to_string()))),
        }
    }
}

impl Display for Env {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut fmt_str = String::new();
        fmt_str.push_str("Env { ");
        for item in &self.0 {
            fmt_str.push_str(&*format!("{}=\"{}\", ", item.0, item.1))
        }
        fmt_str.push_str(" }");
        write!(f, "{}", fmt_str)
    }
}
