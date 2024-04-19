
/*

 Introducing "Dumb", my painful, simple "programming language" inspired by Forth & Co.

* - multiply the last two numbers on the stack and replace them with the result
+ - add "
- - subtract "
/ - divide "
. - print the last number on the stack

<number> - pushes a number onto the stack

 */


mod lexer;
use crate::compiler::Compiler;

mod token;
mod compiler;

fn main() {
    // program should print (1 + 1), then result * 2
    Compiler::compile("1 1 + . 2 * .");

    Compiler::repl();
}
