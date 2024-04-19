
#[derive(Debug, PartialEq)]
pub enum Token {
    Star,   // *
    Slash,  // /
    Plus,   // +
    Minus,  // -
    Period, // .

    Number(f32),

    EOF,
}
