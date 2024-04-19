
#[derive(Debug, PartialEq)]
pub enum Token {
    Star,   // *
    Slash,  // /
    Plus,   // +
    Minus,  // -
    Period, // .

    // may need to implement a parsing abstraction layer to this
    OpenBracket, // [
    CloseBracket, // ]

    Pop,    // pop
    Loop,

    Word(Box<str>),

    Number(f32),

    EOF,
}
