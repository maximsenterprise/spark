// compile.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use std::fs;
use crate::error;

pub fn compile(html: &String, file_name: String) {
    println!("Compiling HTML...");
    println!("HTML compiled successfully!");
    match fs::write("./build/index.html", html) {
        Ok(_) => println!("HTML written to ./build/index.html"),
        Err(error) => error!("Could not write to file: {}", error)
    };
}