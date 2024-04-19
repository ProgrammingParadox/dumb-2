
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
use lexer::Lexer;
use crate::token::Token;

mod token;
//use token::Token;

fn main() {
    // program should print 1 + 1
    let mut lexer = Lexer::new(&"1 1 + . 2 * .");

    let mut tokens = vec![];
    while let cur = lexer.eat_token().unwrap() {
        if cur == Token::EOF {
            break;
        }

        tokens.push(cur);
    }

    println!("Tokens: {:?}", tokens);
}
