// working.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use termion::{color, style};

#[macro_export]
macro_rules! working {
    () => (println!());
    ($($arg:tt)*) => ({
        println!("{}{}{}{}",
            termion::color::Fg(termion::color::Yellow),
            termion::style::Italic,
            format!($($arg)*),
            termion::style::Reset);
    })
}