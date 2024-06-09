// formatter.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use lazy_static::lazy_static;
use spin::{mutex::Mutex};

lazy_static! {
    pub static ref INDENTLEVEL: Mutex<i8> = Mutex::new(1);
}

pub fn format_html(raw: String) -> String {
    let mut new = String::new();
    let mut indent_level: i8 = INDENTLEVEL.lock().clone();
    for _ in 0..indent_level {
        new.push_str("    ");
    }

    new.push_str(raw.as_str());

    new.push_str("\n");

    new
}

pub fn format_style(raw: String) -> String {
    return raw.replace('\n', "");
}

pub fn format_argument(key: String, value: String) -> String {
    let mut new = String::from(" ");
    new.push_str(&key);
    new.push_str("=\"");
    new.push_str(format!("{}\"", value).as_str());
    new
}