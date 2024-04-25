
#[derive(Debug, PartialEq)]
pub enum Token {
    Star,   // *
    Slash,  // /
    Plus,   // +
    Minus,  // -
    Period, // .

    Backtick, // `

    Pop,    // pop

    Word(Box<str>),

    Number(f32),

    EOF,
}
