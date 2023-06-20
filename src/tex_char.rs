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
    Char(char),
}

impl TexChar {}

impl From<char> for TexChar {
    fn from(c: char) -> Self {
        use self::TexChar::*;
        match c {
            ' ' => Whitespace,
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
