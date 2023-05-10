# Letterbox
## An esoteric programming language

This repository contains a rust crate defining the Letterbox programming language. Components:
- Lexer, made with Logos
- Parser
- Command-line interpreter

## Quick Reference
- A Letterbox program is a list of commands separated by whitespace.
- Commands begin with an uppercase letter and take in one or more arguments.
- Each program manipulates a database of 26 variables, one for each lowercase letter.
- Variables default to 0. They can hold integers, floats, or strings.
- A variable is considered false if it is exactly 0 and true otherwise.

### Commands
| Command | Description |
|---------|-------------|
| Pa | Print the value of variable a |
| P'hello' | Prints the string hello |
| Sa3 | Stores value 3 in variable a |
| Sa'hello' | Stores string hello in variable a |
| Cab | Copies var a into var b |
| Abc | Appends var c (as a string) to var b. c's value does not change. |
| MAbac | Math: Set b = a + c. See appendix for list of operations. |
| BEbac | Bool: Set b = a == c. See appendix for list of operations. |
| LaPc | Repeats the statement Pc a number of times equal to var a |
| IaPc | Executes statement Pc only if a is true |
| UaPc | Executes statement Pc UNLESS a is true |
| WaPc | Repeats statement Pc while a is true |
| Ra | Resets var a to 0. "RA" resets all variables. |
| GNa0 | Gets the 0th value passed to the program as input and stores it in var a |
| Na | Negate var a (if true set it to 0, otherwise set it to 1) |
| F | Finish the program |
| Xabdce... | Executes the string in var a as a Letterbox program. Replaces the given pairs of variables before executing (in this example, all usages of b will become d, and all usages of c will become e) |