use crate::key::KeyCounter;
use crate::math_expr::{MathDisc, MathExprParseResult};
use crate::node::Node;
use crate::outside::ParseError;
use crate::outside::MAX_INPUT_LENGTH;
use crate::result_map::ResultMap;
use crate::tex_char::TexChar;
use crate::tex_chars::TexChars;
use std::str::FromStr;

#[derive(Debug)]
pub(super) struct ParseOk {
    pub rmap: ResultMap,
    pub char_count: usize,
}

pub(super) fn parse_paragraphs(input: &str) -> Result<ParseOk, ParseError> {
    let input = correct_lines(input.to_string());

    let char_count = input.chars().count();
    if char_count > MAX_INPUT_LENGTH {
        return Err(ParseError::TooLongInput);
    }

    let mut kc = KeyCounter::new();
    let key = kc.count();

    let ps = parse_into_paragraphs(input);
    let ps: Vec<_> = ps
        .into_iter()
        .map(|cs| parse_paragraph(cs, &mut kc))
        .collect();

    let mut rmap = ResultMap::new(
        key,
        Node::ParagraphList(Some(ps.iter().map(|x| x.root()).collect())),
    );
    rmap.merge(ps);

    Ok(ParseOk { rmap, char_count })
}

const EOL: &str = "\n";

fn correct_lines(input: String) -> String {
    const COMMENT_DISC: &str = "%";

    input
        .lines()
        .map(
            |s| s.split(COMMENT_DISC).next().unwrap(), // 行末コメント除去
        )
        .map(|s| s.trim())
        .collect::<Vec<_>>()
        .join(EOL)
}

fn parse_into_paragraphs(input: String) -> Vec<TexChars> {
    input
        .split("\n\n") // todo EOL定数を使う
        .filter(|x| !x.is_empty())
        .map(|x| TexChars::from_str(x).unwrap())
        .collect()
}

fn parse_paragraph(mut cs: TexChars, kc: &mut KeyCounter) -> ResultMap {
    let key = kc.count();

    let mut maps = Vec::new();
    let mut buffer: Vec<TexChar> = Vec::new();

    macro_rules! push_raw_string {
        () => {
            let content = buffer_to_content_string(&mut buffer);
            if !content.is_empty() {
                let node = Node::RawString(content);
                maps.push(ResultMap::new(kc.count(), node));
            }
            buffer.clear();
        };
    }

    loop {
        if let Some(disc) = MathDisc::match_begin(&cs) {
            push_raw_string!();
            let map = parse_math_expr(&mut cs, kc, disc);
            maps.push(map);
            continue;
        }

        if cs.next_is(TexChar::Backslash) {
            push_raw_string!();
            let map = parse_inline_command(&mut cs, kc);
            maps.push(map);
            continue;
        }

        if let Some(c) = cs.next() {
            buffer.push(c);
        } else {
            push_raw_string!();
            break;
        }
    }

    let mut map = ResultMap::new(
        key,
        Node::Paragraph(Some(maps.iter().map(|x| x.root()).collect())),
    );
    map.merge(maps);

    map
}

fn parse_math_expr(cs: &mut TexChars, kc: &mut KeyCounter, disc: MathDisc) -> ResultMap {
    disc.consume_begin(cs);

    let mut buffer = Vec::new();
    let mut match_end = false;

    loop {
        if disc.match_end(cs) {
            match_end = true;
            disc.consume_end(cs);
            break;
        }

        if let Some(c) = cs.next() {
            buffer.push(c);
        } else {
            break;
        }
    }

    let content = buffer_to_content_string(&mut buffer);
    let node = if match_end {
        MathExprParseResult::ok(content, disc)
    } else {
        MathExprParseResult::err(content, disc)
    };

    ResultMap::new(kc.count(), Node::MathExpr(node))
}

fn parse_inline_command(cs: &mut TexChars, kc: &mut KeyCounter) -> ResultMap {
    let mut buffer = Vec::new();

    buffer.push(cs.next().unwrap());

    // todo
    loop {
        if let Some(TexChar::Char(c)) = cs.read_next() {
            if c.is_alphabetic() {
                buffer.push(TexChar::Char(c));
                cs.next();
                continue;
            }
        }
        break;
    }

    let content = buffer_to_content_string(&mut buffer);

    ResultMap::new(kc.count(), Node::InlineCommand(Some(content)))
}

fn buffer_to_content_string(cs: &mut Vec<TexChar>) -> String {
    let cs = std::mem::take(cs);
    TexChars::from_iter(cs).into_content_string()
}

//noinspection ALL
#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    mod parse_paragraphs {
        use super::*;

        #[test]
        fn 入力が文字数制限の最大値を超える場合はTooLongInput() {
            let input = "a".repeat(MAX_INPUT_LENGTH + 1);

            assert!(matches!(
                parse_paragraphs(&input),
                Err(ParseError::TooLongInput)
            ));
        }

        #[test]
        fn 除去される文字は文字数制限にカウントしない() {
            let input = "a".repeat(MAX_INPUT_LENGTH) + " %foo";

            assert!(!matches!(
                parse_paragraphs(&input),
                Err(ParseError::TooLongInput)
            ));
        }

        #[test]
        fn 動作確認() {
            let input = r"
            「イーハトーヴォ」という言葉は、
            私の知識の範囲では特定の意味を持つ言葉ではありません。
            もし「イーハトーヴォ」について具体的な情報や文脈を教えていただければ、
            お手伝いできるかもしれません。
            
            「イーハトーヴォ」という言葉は、
            私の知識の範囲では特定の意味を持つ言葉ではありません。
            もし「イーハトーヴォ」について具体的な情報や文脈を教えていただければ、
            お手伝いできるかもしれません。
            
            「イーハトーヴォ」という言葉は、
            私の知識の範囲では特定の意味を持つ言葉ではありません。
            もし「イーハトーヴォ」について具体的な情報や文脈を教えていただければ、
            お手伝いできるかもしれません。
            ";

            println!("{:#?}", parse_paragraphs(input));
        }

        #[test]
        fn 動作確認2() {
            let input = r"abc
             \( \mathscr{V} \defeq U_x^X \)は\( X \)の開被覆である.\foo
        例えば$Y$は$x \otimes y$である(\ref):
        \[Z \cong \left{A \oplus B\right. .\]
        例えば$Y2$は$$x_2 \otimes y_2$$である.";

            println!("{:#?}", parse_paragraphs(input));
        }
    }

    mod correct_lines {
        use super::*;
        macro_rules! test_correct_lines {
        ($($name:ident: $value:expr, )*) => {
                $(
                    #[test]
                    fn $name() {
                        let (input, expected) = $value;
                        assert_eq!(correct_lines(input.to_string()), expected.to_string());
                    }
                )*
            };
        }

        test_correct_lines! {
            空文字: ("", ""),
            コメントのみ: ("%", ""),
            前後に空白: (" a b c ", "a b c"),
            複数行_空行あり: (
                r"abc
                
                def",
                "abc\n\ndef"
            ),
            複数行_数式込み: (
                r"数式$x$や % コメント
                \(y\)など %",
                "数式$x$や\n\\(y\\)など"
            ),
        }
    }

    mod parse_into_paragraphs {
        use super::*;

        #[test]
        fn sample1() {
            assert_eq!(
                parse_into_paragraphs("abc\ndef\n\nefg".to_string()),
                vec![
                    TexChars::from_str("abc\ndef").unwrap(),
                    TexChars::from_str("efg").unwrap()
                ]
            );
        }

        #[test]
        fn 連続した空行() {
            assert_eq!(
                parse_into_paragraphs("abc\ndef\n\n\n\nefg".to_string()),
                vec![
                    TexChars::from_str("abc\ndef").unwrap(),
                    TexChars::from_str("efg").unwrap()
                ]
            );
        }
    }
}
