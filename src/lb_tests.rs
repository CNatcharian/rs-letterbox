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
        let mut program = Program::new(lex, &mut data, &mut out).expect("Program init failed");
        let result = program.run();
        if let Err(msg) = result {
            panic!("Program failed: {}", msg);
        }
        assert_eq!(out, String::from($y));
    };
}

#[test]
fn print_store_copy() {
    assert_lb_out!("Sb3", "");
    assert_lb_out!("Sa4 Cab Pb", "4");
    assert_lb_out!("Sa5.5 Pa", "5.5");
    assert_lb_out!("P'Hello world'", "Hello world");
    assert_lb_out!("Sz'This is a test' Pz", "This is a test");
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
}