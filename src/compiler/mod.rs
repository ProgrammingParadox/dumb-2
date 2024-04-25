
mod lexer;

use std::cmp::PartialEq;
use lexer::Lexer;
use crate::token::Token;

use std::io;
use std::io::Write;

#[derive(PartialEq, Debug)]
enum CompilerStatus {
    Done,
    NotDone
}

pub struct Compiler<'a> {
    tokens: &'a mut Vec<Token>,
    position: usize,

    output: String,

    stack: Vec<f32>
}

impl Compiler<'_> {
    pub fn new(tokens: &mut Vec<Token>) -> Compiler {
        Compiler {
            tokens,
            position: 0,

            output: String::new(),

            stack: vec![]
        }
    }

    fn push_tokens(&mut self, tokens: &mut Vec<Token>) {
        self.tokens.append(tokens);
    }
    fn clear_output(&mut self) {
        self.output.clear();
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn eat_expression(&mut self) -> CompilerStatus {
        if self.position >= self.tokens.len() {
            return CompilerStatus::Done;
        }

        let cur = &self.tokens[self.position];

        // do what you can or panic
        match cur {
            Token::Number(x) => self.push_stack(*x),
            Token::Plus => self.add(),
            Token::Star => self.multiply(),
            Token::Period => self.output(),
            Token::Pop => self.pop(),
            Token::Backtick => self.output_as_chars(),
            _ => panic!("Uncovered token {:?}", cur)
        }

        self.advance();

        CompilerStatus::NotDone
    }

    // basically the print command
    fn output(&mut self) {
        if self.output.len() > 0 {
            self.output.push('\n');
        }
        self.output += &*(self.stack
            .last()
            .or_else(|| panic!("Nothing to print!"))
            .unwrap()
            .to_string()
        );
    }
    fn output_as_chars(&mut self) {
        if self.output.len() > 0 {
            self.output.push('\n');
        }
        self.output += &*((self.stack
            .last()
            .or_else(|| panic!("Nothing to print!"))
            .unwrap().floor().abs() as u8 as char)
            .to_string()
        );
    }

    // other operations
    fn pop(&mut self) {
        self.stack.pop();
    }
    fn add(&mut self) {
        let a: f32 = self.stack.pop().unwrap();
        let b: f32 = self.stack.pop().unwrap();

        let result: f32 = a + b;

        self.stack.push(result);
    }
    fn multiply(&mut self) {
        let a: f32 = self.stack.pop().unwrap();
        let b: f32 = self.stack.pop().unwrap();

        let result: f32 = a * b;

        self.stack.push(result);
    }
    fn subtract(&mut self) {
        let a: f32 = self.stack.pop().unwrap();
        let b: f32 = self.stack.pop().unwrap();

        let result: f32 = a - b;

        self.stack.push(result);
    }
    fn divide(&mut self) {
        let a: f32 = self.stack.pop().unwrap();
        let b: f32 = self.stack.pop().unwrap();

        let result: f32 = a / b;

        self.stack.push(result);
    }

    fn push_stack(&mut self, x: f32) {
        self.stack.push(x);
    }

    // sets up and runs a repl
    pub fn repl() {
        println!("Welcome to the Dumb REPL! Have a look around. Type EXIT to exit.");

        let mut binding = Vec::new();
        let mut compiler = Self::new(&mut binding);
        loop {
            // we *could* just run self.compile for every line, but this way the stack
            // stays after each line

            let mut line = String::new();
            io::stdout().write_all(b"> ").unwrap();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut line).unwrap();

            if line.trim() == "EXIT" {
                break;
            } else if line.trim() == "" {
                continue;
            }

            line = line.trim().parse().unwrap();
            let mut lexer = Lexer::new(&line);

            let mut tokens = vec![];
            while let cur = lexer.eat_token().unwrap() {
                if cur == Token::EOF {
                    break;
                }

                tokens.push(cur);
            }

            compiler.push_tokens(&mut tokens);
            compiler.clear_output();

            while let cur = compiler.eat_expression() {
                if cur == CompilerStatus::Done {
                    break;
                }
            }

            println!("{}", compiler.output);
        }
    }

    // compile a set of code
    pub fn compile(code: &str) {
        let mut lexer = Lexer::new(code);

        let mut tokens = vec![];
        while let cur = lexer.eat_token().unwrap() {
            if cur == Token::EOF {
                break;
            }

            tokens.push(cur);
        }

        // println!("Tokens: {:?}", tokens);

        let mut compiler = Self::new(&mut tokens);
        while let cur = compiler.eat_expression() {
            if cur == CompilerStatus::Done {
                break;
            }
        }

        println!("{}", compiler.output);
    }
}
