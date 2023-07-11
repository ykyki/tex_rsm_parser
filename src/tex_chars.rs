use crate::tex_char::TexChar;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(super) struct TexChars {
    queue: VecDeque<TexChar>,
}

impl TexChars {
    pub(crate) fn next_is(&self, c: TexChar) -> bool {
        self.queue.front() == Some(&c)
    }

    pub(crate) fn next_isis(&self, c1: TexChar, c2: TexChar) -> bool {
        let mut iter = self.queue.iter();

        iter.next() == Some(&c1) && iter.next() == Some(&c2)
    }

    pub(crate) fn read_next(&self) -> Option<TexChar> {
        self.queue.front().cloned()
    }

    pub(crate) fn into_content_string(self) -> String {
        use TexChar::*;

        fn drop_head_whitespaces(cs: impl IntoIterator<Item = TexChar>) -> Vec<TexChar> {
            cs.into_iter()
                .skip_while(|c| matches!(c, Whitespace))
                .collect()
        }

        let cs: Vec<_> = self.queue.into();

        // 先頭の空白は除去する
        let cs: Vec<_> = drop_head_whitespaces(cs);

        // 末尾の空白は除去する
        let cs: Vec<_> = drop_head_whitespaces(cs.into_iter().rev())
            .into_iter()
            .rev()
            .collect();

        // 改行前後の空白処理
        // Comma, Period, alphanumeric, ?, ! の後の Return は Whitespace に変換する
        // それ以外の Return は無視する
        // let mut cs = cs;
        let mut new_cs = Vec::new();
        for c in cs {
            if c == Return {
                if let Some(last) = new_cs.last() {
                    match last {
                        Comma | Period => {
                            new_cs.push(Whitespace);
                        }
                        Char(c) => {
                            if c.is_ascii_alphanumeric() || *c == '?' || *c == '!' {
                                new_cs.push(Whitespace);
                            }
                        }
                        _ => {}
                    }
                }
            } else {
                new_cs.push(c);
            }
        }
        let cs: Vec<_> = new_cs;

        // 連続する空白は1つに潰す
        let mut cs = cs;
        cs.dedup_by(|c1, c2| matches!(c1, Whitespace) && matches!(c2, Whitespace));
        let cs: Vec<_> = cs;

        // 文字列に変換
        cs.into_iter().map(|c| c.to_string()).collect()
    }
}

impl FromStr for TexChars {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.chars().map(|x| x.into()).collect::<VecDeque<_>>();

        Ok(Self { queue: v })
    }
}

impl Iterator for TexChars {
    type Item = TexChar;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

impl FromIterator<TexChar> for TexChars {
    fn from_iter<T: IntoIterator<Item = TexChar>>(iter: T) -> Self {
        Self {
            queue: iter.into_iter().collect(),
        }
    }
}

//noinspection ALL
#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        use self::TexChar::*;
        macro_rules! assert_tex_chars {
            ($input:expr, $expected_chars:expr) => {
                let cs: TexChars = $input.parse().unwrap();
                assert_eq!(cs.queue.into_iter().collect::<Vec<_>>(), $expected_chars);
            };
        }

        assert_tex_chars!("", vec![]);

        assert_tex_chars!("abc", vec![Char('a'), Char('b'), Char('c'),]);

        assert_tex_chars!(
            "いろはに",
            vec![Char('い'), Char('ろ'), Char('は'), Char('に'),]
        );

        assert_tex_chars!(
            r"数式\(x, y\)など.",
            vec![
                Char('数'),
                Char('式'),
                Backslash,
                LParen,
                Char('x'),
                Comma,
                Whitespace,
                Char('y'),
                Backslash,
                RParen,
                Char('な'),
                Char('ど'),
                Period,
            ]
        );

        assert_tex_chars!(
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
            ]
        );

        assert_tex_chars!(
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
            ]
        );
    }

    #[test]
    fn into_content_string() {
        macro_rules! assert_content_string {
            ($input:expr, $expected:expr) => {
                let cs: TexChars = $input.parse().unwrap();
                assert_eq!(cs.into_content_string(), $expected);
            };
        }

        assert_content_string!("foo ", "foo");
        assert_content_string!(" foo", "foo");
        assert_content_string!("  foo   ", "foo");

        assert_content_string!("", "");
        assert_content_string!(" ", "");

        assert_content_string!("foo\nbar", "foo bar");
        assert_content_string!("FOO\nbar", "FOO bar");
        assert_content_string!("123\n456", "123 456");
        assert_content_string!("あいう\nえお", "あいうえお");
        assert_content_string!("あいう\neo", "あいうeo");
        assert_content_string!("foo,\nbar", "foo, bar");
        assert_content_string!("foo.\nbar", "foo. bar");
        assert_content_string!("foo?\nbar", "foo? bar");
        assert_content_string!("foo!\nbar", "foo! bar");
        assert_content_string!("foo#\nbar", "foo#bar");

        assert_content_string!(" X,  Y,   Z, W  ", "X, Y, Z, W");

        assert_content_string!("foo, \nbar", "foo, bar");
    }
}
