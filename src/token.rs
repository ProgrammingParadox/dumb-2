
#[derive(Debug, PartialEq)]
pub enum Token {
    Star,   // *
    Slash,  // /
    Plus,   // +
    Minus,  // -
    Period, // .

    Number(i32),

    EOF,
}
