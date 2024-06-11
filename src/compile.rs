// compile.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use std::fs;
use crate::{error, success, success_final, utils::working, working};

pub fn compile(html: &String, file_name: String, make_path: bool) {
    working!("Compiling Spark into HTML...");
    if file_name.ends_with(".html") {
        if make_path == true {
            if !fs::metadata("./build").is_ok() {
                match fs::create_dir("./build") {
                    Ok(_) => {
                        match fs::write(file_name.clone(), html.clone()) {
                            Ok(_) => success_final!("Successfully compiled Spark into HTML"),
                            Err(_) => error!("SC-001: Error writing to file")
                        }
                    }
                    Err(_) => error!("SC-002: Error creating build directory")
                }
            }
            else {
                match fs::write(file_name.clone(), html.clone()) {
                    Ok(_) => success_final!("Successfully compiled Spark into HTML"),
                    Err(_) => error!("SC-003: Error writing to file")
                }
            }
        }
        else if make_path == false {
            if !fs::metadata(file_name.clone()).is_ok() {
                match fs::create_dir_all(file_name.clone()) {
                    Ok(_) => {
                        match fs::write(file_name.clone(), html.clone()) {
                            Ok(_) => success_final!("Successfully compiled Spark into HTML"),
                            Err(_) => error!("SC-004: Error writing to file")
                        }
                    }
                    Err(_) => error!("SC-005: Error creating build directory")
                }
            }
            else {
                match fs::write(file_name.clone(), html.clone()) {
                    Ok(_) => success_final!("Successfully compiled Spark into HTML"),
                    Err(_) => error!("SC-006: Error writing to file")
                }
            }
        }
        println!("");
    }
}