// success.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use termion::{color, style};

#[macro_export]
macro_rules! success {
    () => (println!());
    ($($arg:tt)*) => ({
        println!("{}{}{}{}",
            termion::color::Fg(termion::color::Green),
            termion::style::Italic,
            format!($($arg)*),
            termion::style::Reset);
    })
}

#[macro_export]
macro_rules! success_final {
    () => (println!());
    ($($arg:tt)*) => ({
        println!("{}{}{}{}{}",
            termion::color::Fg(termion::color::Green),
            termion::style::Italic,
            termion::style::Bold,
            format!($($arg)*),
            termion::style::Reset);
    })
}