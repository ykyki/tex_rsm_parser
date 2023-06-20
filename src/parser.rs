use crate::key::Key;
use crate::node::Node;
use std::collections::BTreeMap;

use crate::ParseError;

pub(super) fn parse_paragraphs(input: &str) -> Result<ResultMap, ParseError> {
    let _ = correct_lines(input.to_string());
    todo!();
}

#[derive(Debug)]
pub(super) struct ResultMap {
    value: BTreeMap<Key, Node>, // todo
}

fn correct_lines(input: String) -> String {
    const COMMENT_DISC: &str = "%";

    input
        .lines()
        .map(
            |x| x.split(COMMENT_DISC).next().unwrap(), // 行末コメント除去
        )
        .map(|x| x.trim())
        .collect::<Vec<_>>()
        .join("\n")
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_lines() {
        macro_rules! assert_correct_lines {
            ($input:expr, $expected:expr) => {
                assert_eq!(correct_lines($input.to_string()), $expected.to_string());
            };
        }

        assert_correct_lines!("", "");
        assert_correct_lines!("%", "");
        assert_correct_lines!(" a b c ", "a b c");

        assert_correct_lines!(
            r"abc
            
            def",
            "abc\n\ndef"
        );

        assert_correct_lines!(
            r"数式$x$や % コメント
            \(y\)など %",
            "数式$x$や\n\\(y\\)など"
        );
    }
}
