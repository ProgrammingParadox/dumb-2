
#[derive(Debug, PartialEq)]
pub enum Token {
    Star,   // *
    Slash,  // /
    Plus,   // +
    Minus,  // -
    Period, // .
    Pop,    // pop

    Word(str),

    Number(f32),

    EOF,
}
