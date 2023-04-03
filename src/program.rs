use core::fmt;

use crate::storage::Storage;
use logos::Lexer;
use crate::lexerbox::LBT;
use crate::lexerbox::LBT::*;

/// A value that can be stored in a Letterbox variable.
#[derive(Debug, Clone)]
pub enum Val {
    Text(String),
    Number(f64),
}

impl Val {
    /// The float 0.0.
    /// Used as the default value for a Letterbox variable.
    pub fn zero() -> Val {
        Val::Number(0.0)
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match self {
            Val::Text(text) => text.to_owned(),
            Val::Number(num) => format!("{}", num),
        };
        write!(f, "{}", x)
    }
}

/// A struct that represents a Letterbox program.
/// It combines a list of parsed instructions and a Storage struct,
/// executing each instruction in order.
pub struct Program<'a> {
    /// An ordered list of parsed instructions. See [LBT] for details.
    pub program_list: Vec<LBT>,

    /// An integer that indicates the number of the next 
    /// instruction to execute from the program list.
    program_counter: usize,

    /// A reference to a [Storage] struct which will be modified 
    /// by the execution of this program.
    data: &'a mut Storage,

    /// If true, this program has completed execution and can
    /// no longer be run or stepped through.
    pub finished: bool,

    /// The result of the last executed instruction.
    /// If this program is finished, this will be considered the
    /// result of the whole program.
    pub result: Result<(), String>,

    /// A reference to a buffer to which output will be printed.
    pub output_buffer: &'a mut String,
}

impl<'a> Program<'a> {
    /// Create a new unexecuted program from the contents of
    /// the given lexer. Requires a reference to a Storage struct.
    pub fn new(lex: Lexer<LBT>, starting_data: &'a mut Storage, out: &'a mut String) -> Result<Program<'a>, String> {
        let prog = Program {
            program_list: lex.collect(),
            program_counter: 0,
            data: starting_data,
            finished: false,
            result: Ok(()),
            output_buffer: out,
        };

        Ok(prog)
    }

    /// Run the program until it finishes.
    pub fn run(&mut self) -> Result<(), String> {
        while !self.finished {
            let step_result = self.step();
            if let Err(_) = step_result {
                self.finished = true;
                return self.result.clone();
            }
        }

        return self.result.clone();
    }

    /// Run the next instruction as indicated by the program counter.
    /// This is the main location where parser tokens are mapped to
    /// execution implementations.
    pub fn step(&mut self) -> Result<(), String> {
        if self.finished {
            return Err(String::from("Program is already finished."));
        }

        let find_command = self.program_list.get(self.program_counter);
        let command: &LBT;
        if let Some(token) = find_command {
            command = token;
        }
        else {
            return Err(format!("No command found at counter index {}", self.program_counter));
        }
        // println!("{:?}", command);

        // The execution begins!
        // Do something different for each command type.
        let step_result: Result<(), String> = match command {

            // Sa4
            SaveNumber((var_name, float_val)) => {
                self.data.set_var(*var_name, &Val::Number(*float_val))
            },

            // Sa'Hello'
            SaveStr((var_name, string_val)) => {
                self.data.set_var(*var_name, &Val::Text(string_val.clone()))
            },

            // Cab
            Copy((from_var, to_var)) => {
                self.data.copy(*from_var, *to_var)
            },

            // Pa
            PrintVar(var_name) => {
                let print_str = self.data.get_var(*var_name).expect("Could not get variable.");
                self.output_buffer.push_str(format!("{}", print_str).as_str());
                Ok(())
            },

            // P'Hello'
            PrintStr(string_val) => {
                self.output_buffer.push_str(format!("{}", string_val).as_str());
                Ok(())
            },

            // MAcab
            MathOp((op, target, a, b)) => {
                let Val::Number(n_a) = self.data.get_var(*a).expect(&format!("Could not get variable {a}")).to_owned() else {
                    return Err(format!("Variable {a} is not a number"));
                };
                let Val::Number(n_b) = self.data.get_var(*b).expect(&format!("Could not get variable {b}")).to_owned() else {
                    return Err(format!("Variable {b} is not a number"));
                };

                // compute result
                let result = match op {
                    'A' => n_a + n_b,
                    'S' => n_a - n_b,
                    'M' => n_a * n_b,
                    'D' => n_a / n_b,
                    'E' => if n_a == n_b { 1.0 } else { 0.0 },
                    'G' => if n_a > n_b { 1.0 } else { 0.0 },
                    'L' => if n_a < n_b { 1.0 } else { 0.0 },
                    _ => {
                        return Err(format!("Invalid op {}", op));
                    },
                };
                // save result to storage
                self.data.set_var(*target, &Val::Number(result))
            },
            _ => Err(format!("Unrecognized command {:?}", command)),
        };

        self.result = step_result;
        self.increment_counter();

        Ok(())
    }

    /// Increment the program counter, which determines which
    /// instruction to execute next.
    /// If it hits the end of the program list, we're finished.
    fn increment_counter(&mut self) {
        self.program_counter += 1;
        
        if self.program_counter >= self.program_list.len() {
            self.finished = true;
        }
    }
}