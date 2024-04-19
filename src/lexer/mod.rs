
use crate::token::Token;

pub struct Lexer<'a> {
    code: &'a str,
    cur: char,
    position: usize
}

impl Lexer<'_> {
    pub fn new(code: &str) -> Lexer {
        Lexer {
            code,
            cur: code.as_bytes()[0] as char,
            position: 0
        }
    }

    fn advance(&mut self) {
        self.position += 1;

        if self.position < self.code.len() {
            self.cur = self.code.as_bytes()[self.position] as char;
        }
    }


    fn eat_whitespace(&mut self) {
        while self.position < self.code.len() && self.cur == ' ' {
            self.advance();
        }
    }

    fn eat_number(&mut self) -> Result<Token, u32> {
        let mut number = String::new();

        if self.cur == '-' {
            number.push('-');
            self.advance();
        }

        while self.position < self.code.len() {
            if !(self.cur.is_ascii_digit() || (self.cur == '.' && !number.contains('.'))) {
                break;
            }

            number.push(self.cur);

            self.advance();
        }

        Ok(Token::Number(number.parse::<f32>().unwrap()))
    }

    // fn eat_word(&mut self) -> Token {
    //     // TODO!
    //
    //     Token::word("")
    // }

    fn peek(&mut self) -> Option<&u8> {
        self.code.as_bytes().get(self.position + 1)
    }

    // TODO? Real errors?
    pub fn eat_token(&mut self) -> Result<Token, u32> {
        self.eat_whitespace();

        if self.position >= self.code.len() {
            return Ok(Token::EOF)
        }

        if self.cur == '-' && self.peek().is_some_and(|x| x.is_ascii_digit()) {
            return self.eat_number();
        }

        if self.cur.is_ascii_digit() {
            return self.eat_number();
        }

        // TODO! Check for, then eat a word, then match the word to a keyword (like pop) Maybe have Token::Keyword(Keyword::Pop)?

        return Ok(match self.cur {
            '+' => self.eat_plus(),
            '-' => self.eat_minus(),
            '*' => self.eat_star(),
            '/' => self.eat_slash(),
            '.' => self.eat_period(),
             _  => panic!("Unknown character {:?}", self.cur)
        });
    }

    fn eat_plus(&mut self) -> Token {
        self.advance();

        Token::Plus
    }
    fn eat_minus(&mut self) -> Token {
        self.advance();

        Token::Minus
    }
    fn eat_star(&mut self) -> Token {
        self.advance();

        Token::Star
    }
    fn eat_slash(&mut self) -> Token {
        self.advance();

        Token::Slash
    }
    fn eat_period(&mut self) -> Token {
        self.advance();

        Token::Period
    }
}
