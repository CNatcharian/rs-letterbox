use crate::storage::*;
use crate::program::*;
use crate::lexerbox::LBT;
use logos::Logos;

/// Made for testing Letterbox programs.
/// Asserts that string A, when run as a Letterbox program,
/// produces string B as output.
#[macro_export]
macro_rules! assert_lb_out {
    ( $x:expr, $y:expr ) => {
        let mut data = Storage::new();
        let mut out = String::new();
        let lex = LBT::lexer($x);
        let inv = Vec::<String>::new();
        let mut program = Program::new(lex, &mut data, &inv, &mut out, 1000).expect("Program init failed");
        let result = program.run();
        if let Err(msg) = result {
            panic!("Program failed: {}", msg);
        }
        assert_eq!(out, String::from($y));
    };
}

/// Made for testing Letterbox programs.
/// Asserts that string A, when run as a Letterbox program,
/// using Vec B as inputs,
/// produces string C as output.
#[macro_export]
macro_rules! assert_lb_from_input {
    ( $x:expr, $y:expr, $z:expr ) => {
        let mut data = Storage::new();
        let mut out = String::new();
        let lex = LBT::lexer($x);
        let inv = $y;
        let mut program = Program::new(lex, &mut data, &inv, &mut out, 1000).expect("Program init failed");
        let result = program.run();
        if let Err(msg) = result {
            panic!("Program failed: {}", msg);
        }
        assert_eq!(out, String::from($z));
    };
}

#[test]
fn print_store_copy() {
    assert_lb_out!("Sb3", "");
    assert_lb_out!("Sa4 Cab Pb", "4");
    assert_lb_out!("Sa5.5 Pa", "5.5");
    assert_lb_out!("Sa-6 Pa", "-6");
    assert_lb_out!("Sa-6.5 Pa", "-6.5");
    assert_lb_out!("P'Hello world'", "Hello world");
    assert_lb_out!("Sz'This is a test' Pz", "This is a test");
}

#[test]
fn append() {
    assert_lb_out!("Arc Pr", "00");
    assert_lb_out!("Sr'' Arc Pr", "0");
    assert_lb_out!("Sr'' Sc12.5 Arc Pr", "12.5");
    assert_lb_out!("Sr'The secret is ' Sc12.5 Arc Pr", "The secret is 12.5");
    assert_lb_out!("Sr23 Sc'chicken' Arc Pr", "23chicken");
    assert_lb_out!("Sr'fizz' Sc'buzz' Arc Arc Pr", "fizzbuzzbuzz");
    assert_lb_out!("Sr'fizz' Sc'buzz' Arr Arr Pr", "fizzfizzfizzfizz");
}

#[test]
fn reset_var() {
    assert_lb_out!("Ra", "");
    assert_lb_out!("Pb P' ' Sb3 Pb Rb P' ' Pb", "0 3 0");
    assert_lb_out!("Rb Pb", "0");
}

#[test]
fn reset_all() {
    assert_lb_out!("RA", "");
    assert_lb_out!("Sa1 Sb2 Sc'3' RA Pa Pb Pc", "000");
}

#[test]
fn discrete_loop() {
    assert_lb_out!("Sa3 Sb4 LaPb", "444");
    assert_lb_out!("Sa2 Sd11 LdMAbab Pb", "22");
}

#[test]
fn while_loop() {
    assert_lb_out!("Sa10 Sb1 WaMSaab Pa", "0");
}

#[test]
fn if_statement() {
    assert_lb_out!("IaPb", "");
    assert_lb_out!("Sa0.0 Sb1 IaPb", "");
    assert_lb_out!("Sa10 Sb2 IaPb", "2");
    assert_lb_out!("Sa10 Sb2 MGcab IcPb", "2");
    assert_lb_out!("Sa10 Sb2 MLcab IcPb", "");
}

#[test]
fn unless() {
    assert_lb_out!("Sb5 UaPb", "5");
    assert_lb_out!("Sa0.0 Sb1 UaPb", "1");
    assert_lb_out!("Sa10 Sb2 UaPb", "");
    assert_lb_out!("Sa10 Sb2 MGcab UcPb", "");
    assert_lb_out!("Sa10 Sb2 MLcab UcPb", "2");
}

#[test]
fn negate() {
    assert_lb_out!("Na", "");
    assert_lb_out!("Na Pa", "1");
    assert_lb_out!("Pb P' ' Sb3 Pb Nb P' ' Pb", "0 3 0");
    assert_lb_out!("Sa1 Sb2 MGcab Nc Pc", "1");
}

#[test]
fn execute_basic() {
    assert_lb_out!("Sc'' Xc", "");
    assert_lb_out!("Sa2 Sb'a' Sc'Pb' LaXc", "aa");
    assert_lb_out!(
        "Sa4 Sb'loop ' Sc1 Sd'Pb MAaac MLlaf' Sf12 MLlaf WlXd Pa",
        "loop loop loop loop loop loop loop loop 12");
}

#[test]
fn execute_with_params() {
    assert_lb_out!("Sa1 Sb2 Sx'Pa' Xxab", "2");
    assert_lb_out!("Sf'MAcab' Se2 Sg4 Xfaebgcz Pz", "6");
}

#[test]
fn input() {
    assert_lb_from_input!("Sa4 Pa", vec!["1".to_string(), "2".to_string()], "4");
    assert_lb_from_input!("GNa0 GNb1 MAcab Pa Pb Pc", vec!["1".to_string(), "2".to_string()], "123");
    assert_lb_from_input!("GSa0 Pa", vec!["Pizza".to_string()], "Pizza");
}

#[test]
fn finish() {
    assert_lb_out!("F", "");
    assert_lb_out!("Sa4 Pa F Sa3 Pa", "4");
    assert_lb_out!("Sa4 Pa IbF Sa3 Pa", "43");
}

#[cfg(test)]
mod math_ops {
    use crate::storage::*;
    use crate::program::*;
    use crate::lexerbox::LBT;
    use logos::Logos;

    #[test]
    fn add() {
        assert_lb_out!("Sa3 Sb2 MAcab Pc", "5");
        assert_lb_out!("Sa3.5 Sb1 MAcab Pc", "4.5");
    }

    #[test]
    fn subtract() {
        assert_lb_out!("Sa3 Sb2 MScab Pc", "1");
        assert_lb_out!("Sa3.5 Sb5 MScab Pc", "-1.5");
    }

    #[test]
    fn multiply() {
        assert_lb_out!("Sa3 Sb2 MMcab Pc", "6");
        assert_lb_out!("Sa0.5 Sb5 MMcab Pc", "2.5");
    }

    #[test]
    fn divide() {
        assert_lb_out!("Sa3 Sb2 MDcab Pc", "1.5");
        assert_lb_out!("Sa10 Sb5 MDcab Pc", "2");
    }

    #[test]
    fn equal_to() {
        assert_lb_out!("Sa3 Sb2 MEcab Pc", "0");
        assert_lb_out!("Sa10 Sb10 MEcab Pc", "1");
    }

    #[test]
    fn greater_than() {
        assert_lb_out!("Sa3 Sb2 MGcab Pc", "1");
        assert_lb_out!("Sa3 Sb2 MGcba Pc", "0");
    }

    #[test]
    fn less_than() {
        assert_lb_out!("Sa3 Sb2 MLcba Pc", "1");
        assert_lb_out!("Sa3 Sb2 MLcab Pc", "0");
    }

    #[test]
    fn remainder() {
        assert_lb_out!("Sa3 Sb2 MRcab Pc", "1");
        assert_lb_out!("Sa10 Sb10 MRcab Pc", "0");
        assert_lb_out!("Sa5 Sb10 MRcab Pc", "5");
    }
}

#[cfg(test)]
mod bool_ops {
    use crate::storage::*;
    use crate::program::*;
    use crate::lexerbox::LBT;
    use logos::Logos;

    #[test]
    fn equal() {
        assert_lb_out!("Sa1 Sb'x' BEcab Pc",  "1"); // t t
        assert_lb_out!("Sa0 Sb'' BEcab Pc",   "0"); // f t
        assert_lb_out!("Sa'cz' Sb0 BEcab Pc", "0"); // t f
        assert_lb_out!("Sa0 Sb0.0 BEcab Pc",  "1"); // f f
    }

    #[test]
    fn and() {
        assert_lb_out!("Sa1 Sb'x' BAcab Pc",  "1"); // t t
        assert_lb_out!("Sa0 Sb'' BAcab Pc",   "0"); // f t
        assert_lb_out!("Sa'cz' Sb0 BAcab Pc", "0"); // t f
        assert_lb_out!("Sa0 Sb0.0 BAcab Pc",  "0"); // f f
    }

    #[test]
    fn or() {
        assert_lb_out!("Sa1 Sb'x' BOcab Pc",  "1"); // t t
        assert_lb_out!("Sa0 Sb'' BOcab Pc",   "1"); // f t
        assert_lb_out!("Sa'cz' Sb0 BOcab Pc", "1"); // t f
        assert_lb_out!("Sa0 Sb0.0 BOcab Pc",  "0"); // f f
    }

    #[test]
    fn xor() {
        assert_lb_out!("Sa1 Sb'x' BXcab Pc",  "0"); // t t
        assert_lb_out!("Sa0 Sb'' BXcab Pc",   "1"); // f t
        assert_lb_out!("Sa'cz' Sb0 BXcab Pc", "1"); // t f
        assert_lb_out!("Sa0 Sb0.0 BXcab Pc",  "0"); // f f
    }
}