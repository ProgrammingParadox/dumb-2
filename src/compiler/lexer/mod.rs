
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

    fn eat_word(&mut self) -> Token {
        let mut word = String::new();

        while self.position < self.code.len() {
            if self.cur.is_ascii_whitespace() || !(self.cur.is_ascii_alphanumeric() || self.cur == '_') {
                break;
            }

            word.push(self.cur);

            self.advance();
        }

        Token::Word(word.into_boxed_str())
    }

    fn if_keyword_else_word(&mut self, word: Token) -> Option<Token> {
        if let Token::Word(boxed_name) = word {
            return Some(match boxed_name.as_ref() {
                "pop" => Token::Pop,
                "loop" => Token::Loop,
                _ => Token::Word(boxed_name)
            })
        }

        None
    }

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

        if self.cur.is_ascii_alphabetic() || self.cur == '_' {
            let word = self.eat_word();

            return Ok(self.if_keyword_else_word(word).unwrap());
        }

        // TODO! Check for, then eat a word, then match the word to a keyword (like pop) Maybe have Token::Keyword(Keyword::Pop)?

        return Ok(match self.cur {
            '+' => self.eat_plus(),
            '-' => self.eat_minus(),
            '*' => self.eat_star(),
            '/' => self.eat_slash(),
            '.' => self.eat_period(),
            '[' => self.eat_open_bracket(),
            ']' => self.eat_close_bracker(),
             _  => panic!("Unknown character {:?}", self.cur)
        });
    }

    fn eat_open_bracket(&mut self) -> Token {
        self.advance();

        Token::OpenBracket
    }
    fn eat_close_bracket(&mut self) -> Token {
        self.advance();

        Token::CloseBracket
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
