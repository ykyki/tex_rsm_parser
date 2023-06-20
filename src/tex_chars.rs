use crate::tex_char::TexChar;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
struct TexChars {
    total: u32,
    queue: VecDeque<TexChar>,
}

impl TexChars {}

impl FromStr for TexChars {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.chars().map(|x| x.into()).collect::<VecDeque<_>>();

        Ok(Self {
            total: v.len() as u32,
            queue: v,
        })
    }
}

impl Iterator for TexChars {
    type Item = TexChar;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn TexChars_from_str() {
        use self::TexChar::*;
        macro_rules! test_tex_chars {
            ($input:expr, $expected_chars:expr, $expected_len:expr) => {
                let cs: TexChars = $input.parse().unwrap();
                assert_eq!(cs.queue.into_iter().collect::<Vec<_>>(), $expected_chars);
                assert_eq!(cs.total, $expected_len);
            };
        }

        test_tex_chars!("", vec![], 0);

        test_tex_chars!("abc", vec![Char('a'), Char('b'), Char('c'),], 3);

        test_tex_chars!(
            "いろはに",
            vec![Char('い'), Char('ろ'), Char('は'), Char('に'),],
            4
        );

        test_tex_chars!(
            r"数式\(x, y\)など.",
            vec![
                Char('数'),
                Char('式'),
                Backslash,
                LParen,
                Char('x'),
                Char(','),
                Whitespace,
                Char('y'),
                Backslash,
                RParen,
                Char('な'),
                Char('ど'),
                Char('.'),
            ],
            13
        );

        test_tex_chars!(
            "改行\n\
            \\[\\xi\\]\n\
            など",
            vec![
                Char('改'),
                Char('行'),
                Return,
                Backslash,
                LBracket,
                Backslash,
                Char('x'),
                Char('i'),
                Backslash,
                RBracket,
                Return,
                Char('な'),
                Char('ど'),
            ],
            13
        );

        test_tex_chars!(
            "空行\n\
            \n\
            あり",
            vec![
                Char('空'),
                Char('行'),
                Return,
                Return,
                Char('あ'),
                Char('り'),
            ],
            6
        );
    }
}
