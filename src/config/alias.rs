use crate::config::from_string;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct Alias(HashMap<String, String>);
impl Alias {
    pub fn from_string(c: &str) -> Result<Self, ()> {
        match from_string(c, "alias") {
            Ok(c) => Ok(Self { 0: c }),
            Err(_) => Err(()),
        }
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
    pub fn print_table(&self) {
        let mut table = vec![];
        for (key, value) in self.0.iter() {
            table.push(vec![key.cell(), value.cell().justify(Justify::Right)])
        }
        let table = table
            .table()
            .title(vec!["Name".cell().bold(true), "Alias".cell().bold(true)])
            .bold(true);
        print_stdout(table).unwrap();
    }
}
impl Display for Alias {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut fmt_str = String::new();
        fmt_str.push_str("Alias { ");
        for item in &self.0 {
            fmt_str.push_str(&*format!("{}=\"{}\", ", item.0, item.1))
        }
        fmt_str.push_str(" }");
        write!(f, "{}", fmt_str)
    }
}

impl Default for Alias {
    fn default() -> Self {
        Alias {
            0: HashMap::<String, String>::new(),
        }
    }
}
