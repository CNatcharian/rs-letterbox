use logos::{Logos, Lexer};

/// A Logos-derived enum that can split a Letterbox program
/// into individual tokens AND parse out their arguments.
#[derive(Logos, Debug, PartialEq)]
pub enum LBT {
    /// Save a value into a variable
    #[regex(r"S[a-z][0-9]+(\.[0-9]+)?", save_number)]
    SaveNumber((char, f64)),

    /// Save a value into a variable
    #[regex(r"S[a-z]'[^']*'", save_str)]
    SaveStr((char, String)),

    /// Copy the value of a variable into another.
    #[regex(r"C[a-z][a-z]", copy)]
    Copy((char, char)),

    /// Print the value of the given variable
    /// `Pa`
    #[regex(r"P[a-z]", print_var)]
    PrintVar(char),

    /// Print the given string, replacing underscores with spaces
    /// `P:hello_world`
    #[regex(r"P'[^']*'", print_str)]
    PrintStr(String),

    /// Performs a mathematical operation.
    /// `MAabc`
    #[regex(r"M[A-Z][a-z][a-z][a-z]", math_op)]
    MathOp((char, char, char, char)),

    /// Unrecognized character(s)
    #[error]
    // skip comments
    #[regex(r"![^\n]*", logos::skip)]
    // skip whitespace
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

// Parser methods!

fn save_number(lex: &mut Lexer<LBT>) -> Option<(char, f64)> {
    let token = lex.slice();
    let var_name = token.chars().nth(1);
    let num = token[2..].parse::<f64>().ok();
    if let None = var_name {
        return None;
    }
    if let None = num {
        return None;
    }
    Some((var_name.unwrap(), num.unwrap()))
}

fn save_str(lex: &mut Lexer<LBT>) -> Option<(char, String)> {
    let token = lex.slice();
    let var_name = token.chars().nth(1);
    let my_str = String::from(token[2..].trim_matches('\''));
    if let None = var_name {
        return None;
    }
    Some((var_name.unwrap(), my_str))
}

fn copy(lex: &mut Lexer<LBT>) -> Option<(char, char)> {
    let token = lex.slice();
    let var_name_1 = token.chars().nth(1);
    let var_name_2 = token.chars().nth(2);
    if let None = var_name_1 {
        return None;
    }
    if let None = var_name_2 {
        return None;
    }
    Some((var_name_1.unwrap(), var_name_2.unwrap()))
}

fn print_var(lex: &mut Lexer<LBT>) -> Option<char> {
    let token = lex.slice();
    return token.chars().nth(1);
}

fn print_str(lex: &mut Lexer<LBT>) -> Option<String> {
    let token = lex.slice();
    let my_str = String::from(token[1..].trim_matches('\''));
    Some(my_str)
}

fn math_op(lex: &mut Lexer<LBT>) -> Option<(char, char, char, char)> {
    let token = lex.slice();
    let valid_ops = "ASMDEGL";
    let args: Vec<char> = token[1..].chars().collect();
    // must have exactly one op and three vars
    if args.len() != 4 {
        return None;
    }
    // op must be valid
    if !valid_ops.contains(args[0]) {
        return None;
    }
    Some((args[0], args[1], args[2], args[3]))
}

#[test]
fn tokens_parse_correctly() {
    let mut lex = LBT::lexer("Sa4.4 Cab P'hello world' Pa i ! This is a comment".trim());
    assert_eq!(lex.next(), Some(LBT::SaveNumber(('a', 4.4))));
    assert_eq!(lex.slice(), "Sa4.4");
    assert_eq!(lex.next(), Some(LBT::Copy(('a', 'b'))));
    assert_eq!(lex.slice(), "Cab");
    assert_eq!(lex.next(), Some(LBT::PrintStr(String::from("hello world"))));
    assert_eq!(lex.slice(), "P'hello world'");
    assert_eq!(lex.next(), Some(LBT::PrintVar('a')));
    assert_eq!(lex.slice(), "Pa");
    assert_eq!(lex.next(), Some(LBT::Error)); 
    assert_eq!(lex.slice(), "i");
    assert_eq!(lex.next(), None);
}

