// error.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use termion::{color, style};

#[macro_export]
macro_rules! error {
    () => (println!());
    ($($arg:tt)*) => ({
        println!("{}{}Error:",
            termion::color::Fg(termion::color::Red),
            termion::style::Bold);
        println!("{}{}{}{}",
            termion::color::Fg(termion::color::Red),
            termion::style::Italic,
            format!($($arg)*),
            termion::style::Reset);
        std::process::exit(1);
    })
}