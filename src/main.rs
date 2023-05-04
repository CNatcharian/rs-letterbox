// LETTERBOX //
// An Assembly-style programming toy
// by Chris Natcharian

use std::fs;
use std::io;
use std::io::Write;
use std::process::exit;

mod program;
mod storage;
mod lexerbox;

#[cfg(test)]
mod lb_tests;

use crate::program::Program;
use crate::storage::Storage;
use crate::lexerbox::LBT;
use clap::Command;
use logos::{Logos, Lexer};
use clap::{Arg, ArgAction, arg};

fn main() {
    // parse command line args
    let matches = Command::new("Letterbox")
        .version("1.0")
        .author("ChairianWorks")
        .about("LETTERBOX - An Assembly-style coding toy")
        .arg(arg!(-l --looplimit <VALUE>)
            .required(false)
            .help("Set limit for loops")
            .long_help("Set limit for loops/recursion (default = 1000)"))
        .arg(arg!(-f --file <FILE>)
            .required(false)
            .help("Run program from file")
            .long_help("Run program from file"))
        .arg(Arg::new("PROGRAM-ARGS")
            .help("Pass arguments to your program")
            .long_help("Any arguments you provide will be available to your Letterbox program.")
            .action(ArgAction::Append))
        .get_matches();

    // extract program args
    let args = matches
        .get_many::<String>("PROGRAM-ARGS")
        .unwrap_or_default()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();

    let Ok(loop_limit) = matches
        .get_one::<String>("looplimit")
        .unwrap_or(&"1000".to_string())
        .parse::<usize>()
    else {
        println!("Invalid --looplimit {}", matches.get_one::<String>("looplimit").unwrap());
        exit(0);
    };

    // get filepath from args; if no filepath, open a command prompt
    match matches.get_one::<String>("file") {
        Some(file_path) => run_program_from_file(file_path.to_owned(), loop_limit, args),
        None => run_command_line(loop_limit, args),
    }
}

/// Reads the file at the given path. If it contains text, runs it as a Letterbox program.
fn run_program_from_file(file_path: String, loop_limit: usize, args: Vec<String>) {
    // read file at filepath
    let program_string = fs::read_to_string(file_path).expect("Problem reading file");
    
    // println!("File contents:\n{}", program_string);

    let lex: Lexer<LBT> = LBT::lexer(program_string.trim());
    let mut data = Storage::new();
    let input_vec = args.to_owned();
    let mut output_buffer = String::new();
    let mut program = Program::new(
        lex,
        &mut data,
        &input_vec,
        &mut output_buffer,
        loop_limit
    ).expect("Error initializing program");

    // println!("Program contents:\n{:?}", program.program_list);
    let program_result = program.run();
    if output_buffer.len() > 0 { println!("{}", output_buffer); }
    if let Err(msg) = program_result {
        println!("Error: {}", msg);
    }
}

/// Begins a loop in which the user can enter and execute Letterbox statements.
/// Lasts until Ctrl+C is pressed or `quit` is entered.
fn run_command_line(loop_limit: usize, args: Vec<String>) {

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

        // Define output buffers for the line.
        let mut line_output = String::new();

        // Lex and parse the line by creating a new Program instance referencing the Storage.
        let lex = LBT::lexer(line.trim());
        let mut program = Program::new(lex,
            &mut total_storage,
            &args,
            &mut line_output,
            loop_limit
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

