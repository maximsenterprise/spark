// program.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

use spin::Mutex;

pub struct Program {
    pub variables: Vec<Variable>,
}

pub struct Variable {
    pub symbol: String,
    pub value: String,
}

pub static mut PROGRAM: Option<Mutex<Program>> = None;

pub fn initialize_program() {
    unsafe {
        PROGRAM = Some(Mutex::new(Program {
            variables: Vec::new(),
        }));
    }
}

pub fn add_variable(symbol: String, value: String) {
    unsafe {
        if let Some(program) = &PROGRAM {
            let mut program = program.lock();
            program.variables.push(Variable {
                symbol,
                value,
            });
        }
    }
}

pub fn get_variable(symbol: String) -> String {
    unsafe {
        if let Some(program) = &PROGRAM {
            let program = program.lock();
            for variable in program.variables.iter() {
                if variable.symbol == symbol {
                    return variable.value.clone();
                }
            }
        }
    }
    String::new()
}