use crate::key::Key;

#[derive(Debug)]
pub(super) enum Node {
    ParagraphList(Option<Vec<Key>>),
    Paragraph(Option<Vec<Key>>),
    RawString(String),
    InlineCommand(Option<String>),
    InlineMath(Option<MathExpression>),
    DisplayMath(Option<MathExpression>),
}

impl Node {
    pub fn is_ok(&self) -> bool {
        use self::Node::*;
        matches!(
            self,
            ParagraphList(Some(_))
                | Paragraph(Some(_))
                | RawString(_)
                | InlineCommand(Some(_))
                | InlineMath(Some(_))
                | DisplayMath(Some(_))
        )
    }
}

#[derive(Debug)]
struct MathExpression {
    content: String,
}
