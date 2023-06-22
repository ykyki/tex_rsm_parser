use crate::tex_char::TexChar;
use crate::tex_chars::TexChars;

#[derive(Debug)]
pub(super) enum MathExprParseResult {
    Ok(MathExprInfo),
    Err(MathExprInfo),
}

impl MathExprParseResult {
    pub(crate) fn ok(content: String, disc: MathDisc) -> Self {
        Self::Ok(MathExprInfo { content, disc })
    }

    pub(crate) fn err(content: String, disc: MathDisc) -> Self {
        Self::Err(MathExprInfo { content, disc })
    }

    pub(crate) fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }
}

#[derive(Debug)]
pub(super) struct MathExprInfo {
    disc: MathDisc,
    content: String,
}

#[derive(Debug)]
pub(super) enum MathDisc {
    BsParen,
    BsBracket,
    DoubleDollar,
    SingleDollar,
}

impl MathDisc {
    pub(crate) fn is_inline(&self) -> bool {
        use MathDisc::*;
        matches!(self, SingleDollar | BsParen)
    }

    pub(crate) fn is_display(&self) -> bool {
        use MathDisc::*;
        matches!(self, DoubleDollar | BsBracket)
    }

    pub(crate) fn match_begin(cs: &TexChars) -> Option<Self> {
        use MathDisc::*;
        use TexChar::*;

        if cs.next_isis(Backslash, LParen) {
            return Some(BsParen);
        }

        if cs.next_isis(Backslash, LBracket) {
            return Some(BsBracket);
        }

        if cs.next_isis(Dollar, Dollar) {
            return Some(DoubleDollar);
        }

        if cs.next_is(Dollar) {
            return Some(SingleDollar);
        }

        None
    }

    pub(crate) fn match_end(&self, cs: &TexChars) -> bool {
        use MathDisc::*;
        use TexChar::*;

        match self {
            BsParen => cs.next_isis(Backslash, RParen),
            BsBracket => cs.next_isis(Backslash, RBracket),
            DoubleDollar => cs.next_isis(Dollar, Dollar),
            SingleDollar => cs.next_is(Dollar),
        }
    }

    pub(crate) fn consume_begin(&self, cs: &mut TexChars) {
        use MathDisc::*;

        match self {
            BsParen | BsBracket | DoubleDollar => {
                cs.next().unwrap();
                cs.next().unwrap();
            }
            SingleDollar => {
                cs.next().unwrap();
            }
        }
    }

    pub(crate) fn consume_end(&self, cs: &mut TexChars) {
        use MathDisc::*;

        match self {
            BsParen | BsBracket | DoubleDollar => {
                cs.next().unwrap();
                cs.next().unwrap();
            }
            SingleDollar => {
                cs.next().unwrap();
            }
        }
    }
}
