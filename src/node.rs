use crate::key::Key;
use crate::math_expr::MathExprParseResult;

#[derive(Debug)]
pub(super) enum Node {
    ParagraphList(Option<Vec<Key>>),
    Paragraph(Option<Vec<Key>>),
    RawString(String),
    InlineCommand(Option<String>),
    MathExpr(MathExprParseResult),
}

impl Node {
    // pub fn is_ok(&self) -> bool {
    //     use Node::*;
    //
    //     match self {
    //         ParagraphList(Some(_)) => true,
    //         Paragraph(Some(_)) => true,
    //         RawString(_) => true,
    //         InlineCommand(Some(_)) => true,
    //         MathExpr(x) => x.is_ok(),
    //         _ => false,
    //     }
    // }
}
