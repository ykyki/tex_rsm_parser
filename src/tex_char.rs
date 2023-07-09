use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum TexChar {
    Backslash,
    Whitespace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Dollar,
    Return,
    Period,
    Comma,
    Char(char),
}

impl TexChar {}

impl From<char> for TexChar {
    fn from(c: char) -> Self {
        use self::TexChar::*;
        match c {
            ' ' => Whitespace,
            '.' => Period,
            ',' => Comma,
            '\\' => Backslash,
            '(' => LParen,
            ')' => RParen,
            '[' => LBracket,
            ']' => RBracket,
            '\n' => Return,
            '$' => Dollar,
            _ => Char(c),
        }
    }
}

impl Display for TexChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use self::TexChar::*;
        match self {
            Char(c) => write!(f, "{}", c),
            Backslash => write!(f, "\\"),
            Whitespace => write!(f, " "),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            LBracket => write!(f, "["),
            RBracket => write!(f, "]"),
            Return => writeln!(f),
            Dollar => write!(f, "$"),
            Period => write!(f, "."),
            Comma => write!(f, ","),
        }
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn TexChar_from_char() {
        use self::TexChar::*;
        assert_eq!(TexChar::from(' '), Whitespace);
        assert_eq!(TexChar::from('\\'), Backslash);
    }
}
