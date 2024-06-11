// main.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use std::{env::{self, args}, error, fmt::Error, fs, io, process::Command};

use lazy_static::lazy_static;
use neo_rust::{process_file, NeoConfig};
use spark::{compile::compile, error, interpreter::interpreter::run, lexer::{lexer::tokenize, tokens::Token}, parser::parser::parse};
use spin::Mutex;
use spark::success_final;

fn main() {
    let arguments = args().collect::<Vec<String>>();
    if arguments.len() == 2 {
        if arguments[1] == "init" {
            init("My Spark Project");
            success_final!("The project has been initialized");
        }
        else if arguments[1] == "run" {
            match read_file("./spark.neo".to_string()) {
                Ok(data) => {
                    let config = process_file(data);
                    if config.is_none() {
                        error!("Error processing the spark.neo file");
                    }
                    else {
                        process_config(config, "./build".to_string());  
                    } 
                }
                Err(_) => error!("No spark.neo file found in root directory")
            };
            
        }
        else {
            error!("Invalid arguments");
        }
    }
    else if arguments.len() == 3 {
        if arguments[1] == "init" {
            init(arguments[1].as_str());
            success_final!("The project has been initialized");
        }
        else if arguments[1] == "compile" {
            let file = arguments[2].clone();
            let data = read_file(file.clone()).unwrap();
            let mut tokens = tokenize(data);
            let mut ast = parse(&mut tokens);
            let html = run(&mut ast, true).unwrap();
            let new_file = "build".to_string() + file.replace("src/", "").replace(".spark", ".html").to_owned().as_str();
            compile(&html, new_file.clone(), true);
            tidy(new_file);
        }
        else {
            error!("Invalid arguments");
        }
    } 
    else if arguments.len() == 5 {
        if arguments[1] == "compile" {
            let file = arguments[2].clone();
            let data = read_file(file.clone()).unwrap();
            let mut tokens = tokenize(data);
            let mut ast = parse(&mut tokens);
            let html = run(&mut ast, true).unwrap();
            if arguments[3] == "-o" {
                let new_file = arguments[4].clone();
                compile(&html, new_file.clone(), false);
                tidy(new_file);
            }
            else {
                error!("Invalid arguments");
            }
        }
        else {
            error!("Invalid arguments");
        }
    }
    else {
        error!("Invalid arguments");
    }
}

fn process_config(config: Option<NeoConfig>, path: String) {
    for file in config.clone().unwrap().files_to_compile {
        if file.ends_with(".css") {
            let new_file = path.clone() + file.replace(&config.clone().unwrap().sources, "").to_owned().as_str();
            fs::copy(file.clone(), new_file.clone()).expect("Error copying css file");
        }
        else if file.ends_with(".html") {
            let new_file = path.clone() + file.replace(&config.clone().unwrap().sources, "").to_owned().as_str();
            fs::copy(file.clone(), new_file.clone()).expect("Error copying html file");
        }
        else if file.ends_with(".js") {
            let new_file = path.clone() + file.replace(&config.clone().unwrap().sources, "").to_owned().as_str();
            fs::copy(file.clone(), new_file.clone()).expect("Error copying js file");
        }
        else if file.ends_with(".spark") {
            let data = read_file(file.clone()).unwrap();
            let mut tokens = tokenize(data);
            let mut ast = parse(&mut tokens);
            let html = run(&mut ast, true).unwrap();
            let new_file = path.clone() + file.replace(&config.clone().unwrap().sources, "").replace(".spark", ".html").to_owned().as_str();
            compile(&html, new_file.clone(), true);
            tidy(new_file);
        }
        else if fs::metadata(file.clone()).unwrap().is_dir() {
            let new_folder = path.clone() + file.replace(&config.clone().unwrap().sources, "").to_owned().as_str();
            fs::create_dir(new_folder.clone()).expect("Error creating directory");
            process_config(
                Some(NeoConfig {
                    name: config.clone().unwrap().name.clone(),
                    language: config.clone().unwrap().language.clone(),
                    version: config.clone().unwrap().version.clone(),
                    files_to_compile: fs::read_dir(file.clone())
                        .unwrap()
                        .map(|file| file.unwrap().path().to_str().unwrap().to_string())
                        .collect(),
                    sources: file.clone(),
                }),
                new_folder.clone(),
            );
        }
        else {
            error!("Invalid file type");
        }
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
    match file_content {
        Ok(content) => Ok(content),
        Err(_) => Err("The file inputed does not exist")
    }
}
