use core::fmt;
use regex::Regex;

use crate::storage;
use crate::storage::Storage;
use logos::{Lexer, Logos};
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

    /// Contains all input passed into this program from the environment
    /// i.e. the command line.
    pub input_vec: &'a Vec<String>,

    /// A reference to a buffer to which output will be printed.
    pub output_buffer: &'a mut String,

    /// The maximum number of times a loop can run in this program.
    /// If a single loop exceeds this number, the program will crash.
    pub loop_limit: usize,
}

impl<'a> Program<'a> {
    /// Create a new unexecuted program from the contents of
    /// the given lexer. Requires a reference to a Storage struct.
    pub fn new(lex: Lexer<LBT>,
        starting_data: &'a mut Storage,
        inv: &'a Vec<String>,
        out: &'a mut String,
        loop_limit: usize,
    ) -> Result<Program<'a>, String> {
        let plist: Vec<LBT> = lex.collect();
        let prog = Program {
            program_list: plist,
            program_counter: 0,
            data: starting_data,
            finished: false,
            result: Ok(()),
            input_vec: inv,
            output_buffer: out,
            loop_limit
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
    pub fn step(&mut self) -> Result<(), String> {
        if self.finished {
            return Err(String::from("Program is already finished."));
        }

        // Get the instruction at the next position in the program.
        if let Some(token) = self.program_list.get(self.program_counter) {

            // Clone the token to prevent an immutable borrow
            let command = &token.clone();

            // Evaluate the instruction
            let step_result: Result<(), String> = self.evaluate(command);

            // Set the current result to the most recent instruction's result
            self.result = step_result;

            // If there is an error, don't execute any further.
            if let Err(msg) = &self.result {
                return Err(msg.to_string());
            }

            // Increment the program counter
            self.increment_counter();

            Ok(())
        }
        else {
            return Err(format!("No command found at counter index {}", self.program_counter));
        }
    }

    /// Runs an instruction and returns a result.
    /// This is the main location where parser tokens are mapped to
    /// execution implementations. Side effects abound as these implementations 
    /// can and will manipulate this program's data storage.
    fn evaluate(&mut self, command: &LBT) -> Result<(), String> {
        match command {

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
                let Val::Number(n_a) = self.data
                    .get_var(*a)
                    .expect(&format!("M: Could not get variable {a}"))
                    .to_owned() 
                else {
                    return Err(format!("M: Variable {a} is not a number"));
                };
                let Val::Number(n_b) = self.data
                    .get_var(*b)
                    .expect(&format!("M: Could not get variable {b}"))
                    .to_owned() 
                else {
                    return Err(format!("M: Variable {b} is not a number"));
                };

                // compute result
                let result = match op {
                    'A' => n_a + n_b,                               // add
                    'S' => n_a - n_b,                               // subtract
                    'M' => n_a * n_b,                               // multiply
                    'D' => n_a / n_b,                               // divide
                    'R' => n_a % n_b,                               // remainder
                    'E' => if n_a == n_b { 1.0 } else { 0.0 },      // equal to
                    'G' => if n_a > n_b { 1.0 } else { 0.0 },       // greater than
                    'L' => if n_a < n_b { 1.0 } else { 0.0 },       // less than
                    _ => {
                        return Err(format!("M: Invalid op {}", op));
                    },
                };
                // save result to storage
                self.data.set_var(*target, &Val::Number(result))
            },

            // BAcab
            BoolOp((op, target, a, b)) => {
                let b_a = self.data
                    .var_as_bool(*a)
                    .expect(&format!("B: Could not get variable {a}"))
                    .to_owned();
                let b_b = self.data
                    .var_as_bool(*b)
                    .expect(&format!("B: Could not get variable {b}"))
                    .to_owned();

                // compute result
                let result = match op {
                    'E' => if b_a == b_b { 1.0 } else { 0.0 },                       // equal to
                    'A' => if b_a && b_b { 1.0 } else { 0.0 },                       // and
                    'O' => if b_a || b_b { 1.0 } else { 0.0 },                       // or
                    'X' => if (b_a && !b_b) || (!b_a && b_b) { 1.0 } else { 0.0 }, // xor
                    _ => {
                        return Err(format!("B: Invalid op {}", op));
                    },
                };
                // save result to storage
                self.data.set_var(*target, &Val::Number(result))
            },

            // Ra
            ResetVar(var_name) => {
                self.data.reset_var(*var_name)
            },

            // Na
            Negate(var_name) => {
                let current = self.data
                    .var_as_bool(*var_name)
                    .expect(&format!("Could not get variable {var_name}"))
                    .to_owned();
                if current {
                    return self.data.reset_var(*var_name);
                }
                else {
                    return self.data.set_var(*var_name, &Val::Number(1.0));
                }
            },

            // RA
            ResetAll => {
                self.data.reset_all()
            },

            // LaX
            Loop((times, subcommand)) => {
                // get number of loops
                let Val::Number(t) = self.data
                    .get_var(*times)
                    .expect(&format!("L: Could not get variable {times}"))
                    .to_owned() 
                else {
                    return Err(format!("L: Variable {times} is not a number"));
                };

                let mut loops = t.floor() as i64;
                
                // execute subcommand that many times
                while loops > 0 {
                    if let Err(msg) = self.evaluate(subcommand) {
                        return Err(msg);
                    }
                    loops -= 1;
                }

                Ok(())
            },

            // IaX
            IfStatement((cond, subcommand)) => {
                // get condition as bool
                let c = self.data
                    .var_as_bool(*cond)
                    .expect(&format!("I: Could not get variable {cond}"))
                    .to_owned();
                
                // execute subcommand if condition is true
                if c {
                    return self.evaluate(subcommand);
                }

                Ok(())
            },

            // WaX
            WhileLoop((cond, subcommand)) => {
                // get condition as bool
                let mut c = self.data
                    .var_as_bool(*cond)
                    .expect(&format!("W: Could not get variable {cond}"))
                    .to_owned();
                
                // execute subcommand until condition evaluates false
                while c {
                    if let Err(msg) = self.evaluate(subcommand) {
                        return Err(msg);
                    }

                    c = self.data
                    .var_as_bool(*cond)
                    .expect(&format!("W: Could not get variable {cond}"))
                    .to_owned();
                }

                Ok(())
            },

            // GXa1
            GetInput((op, var, num)) => {
                let index = num.floor() as usize;
                let Some(input) = self.input_vec.get(index) else {
                    return Err(format!("G: no input at index {num}"))
                };
                let input_item = input.to_string();

                if !storage::is_var(var) {
                    return Err(format!("G: character {var} is not a variable name"));
                }
                match *op {
                    'N' => {
                        if let Some(val) = input_item.parse::<f64>().ok() {
                            self.data.set_var(*var, &Val::Number(val))
                        }
                        else {
                            Err(format!("G: Could not parse input into number: {input_item}"))
                        }
                    },
                    'S' => {
                        self.data.set_var(*var, &Val::Text(String::from(input_item)))
                    },
                    _ => Err(format!("G: invalid operation {op}")),
                }
            },

            // Xzacbd
            Execute((fn_var, argmap)) => {
                // validate argmap
                for c in argmap.chars() {
                    if !storage::is_var(&c) {
                        return Err(format!("X: Character {c} is not a variable name"));
                    }
                }

                // get string to execute
                let Val::Text(prog) = self.data
                    .get_var(*fn_var)
                    .expect(&format!("X: Could not get variable {fn_var}"))
                    .to_owned() 
                else {
                    return Err(format!("X: Variable {fn_var} is not a string"));
                };

                // substitute provided arguments
                let prog_with_params = Self::apply_argmap(prog, argmap.to_string());

                // create lexer to parse the string
                let sub_lex = LBT::lexer(&prog_with_params);
                // create new program using this program's params
                let sub_program = Program::new(
                    sub_lex,
                    self.data, 
                    self.input_vec, 
                    self.output_buffer, 
                    self.loop_limit.clone());

                match sub_program {
                    Ok(mut program) => program.run(),
                    Err(msg) => Err(msg),
                }
            },

            // F
            Finish => {
                self.finished = true;
                Ok(())
            },

            _ => Err(format!("Unrecognized instruction at counter index {}", self.program_counter)),
        }
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

    /// Used by Execute (`Xzacbd`).
    /// 
    /// Given a string of sequential argument mappings (i.e. "acbd"), and a String containing
    /// a Letterbox program, replaces each usage of a parameter name with its given variable.
    /// For the given example, all usages of 'a' will be replaced with 'c' and 'b' will be replaced
    /// with 'd'. This does not affect hardcoded strings being saved or printed in the program.
    fn apply_argmap(raw: String, argmap: String) -> String {

        // use this regex to match quotes
        let rx_quotes = Regex::new(r"'[^']*'").expect("Invalid regex");

        // remove all quoted strings from the text
        let quoted_strings = rx_quotes.find_iter(&raw);
        let text_no_quotes = rx_quotes.replace_all(&raw, "%%%");
        let mut replaceable_text = String::from(text_no_quotes);
        
        // replace each parameter with its given variable
        let argvec: Vec<char> = argmap.chars().collect();
        for i in (0..argmap.len()).step_by(2) {
            let param = argvec[i];
            let arg = argvec[i + 1];
            replaceable_text = replaceable_text.replace(&String::from(param), &String::from(arg));
        }
        
        // put the quotes back
        for quote in quoted_strings {
            replaceable_text = replaceable_text.replacen("%%%", quote.as_str(), 1);
        }

        return replaceable_text;
    }
}