// main.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use std::{env::{self, args}, fmt::Error, fs, io, process::Command};

use lazy_static::lazy_static;
use neo_rust::{process_file, NeoConfig};
use spark::{compile::compile, error, interpreter::interpreter::run, lexer::{lexer::tokenize, tokens::Token}, parser::parser::parse};
use spin::Mutex;

fn main() {
    let arguments = args().collect::<Vec<String>>();
    if arguments.len() == 2 {
        if arguments[1] == "init" {
            init("My Spark Project");
            println!("The project has been initialized")
        }
        else if arguments[1] == "run" {
            let data = read_file("./spark.neo".to_string()).unwrap();
            let config = process_file(data);
            if config.is_none() {
                error!("Error processing the spark.neo file");
            }
            else {
                for file in config.unwrap().files_to_compile {
                    let data = read_file(file.clone()).unwrap();
                    let mut tokens = tokenize(data);
                    let mut ast = parse(&mut tokens);
                    let html = run(&mut ast, true).unwrap();
                    let new_file = "build".to_string() + file.replace("src/", "").replace(".spark", ".html").to_owned().as_str();
                    compile(&html, new_file.clone());
                    tidy(new_file);
                }
            }
        }
    }
    else if arguments.len() == 3 {
        if arguments[1] == "init" {
            init(arguments[1].as_str());
            println!("The project has been initialized")
        }
        else if arguments[1] == "compile" {
            let file = arguments[2].clone();
            let data = read_file(file.clone()).unwrap();
            let mut tokens = tokenize(data);
            let mut ast = parse(&mut tokens);
            let html = run(&mut ast, true).unwrap();
            let new_file = "build".to_string() + file.replace("src/", "").replace(".spark", ".html").to_owned().as_str();
            compile(&html, new_file.clone());
            tidy(new_file);
        }
    } 
    else {
        error!("Invalid arguments");
    }
}

fn init(name: &str) {
    let neo_content = format!(r#"
name: {}
version: 1.0.0
language: spark
sources: src
"#, name);
    let spark_content = r#"
[name: "Index"]
[author: "Some author"]

@content
title("Welcome to Spark")<h1> [
    color: black;
]
    "#;

    fs::write("./spark.neo", neo_content).expect("Error creating spark.neo file");
    if !fs::metadata("./src").is_ok() {
        fs::create_dir("./src").expect("Error creating src directory");
    }
    fs::write("./src/index.spark", spark_content).expect("Error creating index.spark file");
}

fn tidy(file: String) {
    let tidy = Command::new("tidy")
        .args(&["-indent", "-modify", "-quiet", "--tidy-mark", "no", file.as_str()])
        .output()
        .expect("Error running tidy");
    if tidy.status.success() {
        println!("The file has been created")
    }
}

fn read_file(file_path: String) -> Result<String, &'static str> {
    let file_content = fs::read_to_string(file_path);
    match  file_content {
        Ok(content) => Ok(content),
        Err(_) => Err("The file inputed does not exist")
    }
}
