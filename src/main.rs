// LETTERBOX //
// An Assembly-style programming toy
// by Chris Natcharian

use std::env;
use std::fs;
use std::io;
use std::io::Write;

mod program;
mod storage;
mod lexerbox;

#[cfg(test)]
mod lb_tests;

use crate::program::Program;
use crate::storage::Storage;
use crate::lexerbox::LBT;
use logos::{Logos, Lexer};

fn main() {
    let args: Vec<String> = env::args().collect();

    // get filepath from args; if no filepath, open a command prompt
    match args.get(1) {
        Some(file_path) => run_program_from_file(file_path.to_owned()),
        None => run_command_line(),
    }
}

/// Reads the file at the given path. If it contains text, runs it as a Letterbox program.
fn run_program_from_file(file_path: String) {
    // read file at filepath
    let program_string = fs::read_to_string(file_path).expect("Problem reading file");
    
    // println!("File contents:\n{}", program_string);

    let lex: Lexer<LBT> = LBT::lexer(program_string.trim());
    let mut data = Storage::new();
    let mut output_buffer = String::new();
    let mut program = Program::new(lex, &mut data, &mut output_buffer).expect("Error initializing program");

    // println!("Program contents:\n{:?}", program.program_list);
    let program_result = program.run();
    if output_buffer.len() > 0 { println!("{}", output_buffer); }
    if let Err(msg) = program_result {
        println!("Error: {}", msg);
    }
}

/// Begins a loop in which the user can enter and execute Letterbox statements.
/// Lasts until Ctrl+C is pressed or `quit` is entered.
fn run_command_line() {
    println!("//// LETTERBOX ////");
    println!("// An Assembly-style coding toy");
    println!("// (c) ChairianWorks 2023");

    // Establish a single data storage.
    let mut total_storage = Storage::new();

    loop {
        // Collect line of program from input.
        let mut line = String::new();
        print!("> ");
        io::stdout().flush().expect("Failed to flush to stdout."); // makes sure '>' is printed before pausing for input
        io::stdin().read_line(&mut line).expect("Failed to read from stdin.");

        // if special command "quit" has been typed, exit the loop.
        if line.trim().to_lowercase() == "quit" { break; }

        // Define an output buffer for the line.
        let mut line_output = String::new();

        // Lex and parse the line by creating a new Program instance referencing the Storage.
        let lex = LBT::lexer(line.trim());
        let mut program = Program::new(lex,
            &mut total_storage,
            &mut line_output
        ).expect("Error parsing line.");

        // Execute line until it finishes.
        let line_result = program.run();

        // Print buffered output.
        if line_output.len() > 0 { println!("{}", line_output); }

        // Print any errors.
        if let Err(msg) = line_result {
            println!("Error: {}", msg);
        }
    }
}

