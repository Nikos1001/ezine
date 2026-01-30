
use crate::ArenaVec;
use super::*;

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType<'arena> {
    Paren(ArenaVec<'arena, Token<'arena>>),
    Brack(ArenaVec<'arena, Token<'arena>>),
    Brace(ArenaVec<'arena, Token<'arena>>)
}

#[derive(PartialEq, Eq, Debug)]
pub struct Token<'arena> {
    pub ty: TokenType<'arena>,
    pub span: SrcSpan,
}

#[derive(PartialEq, Eq, Debug)]
pub enum SegmentType<'arena> {
    Text,
    Expr(ArenaVec<'arena, Token<'arena>>),
    Stmt(ArenaVec<'arena, Token<'arena>>)
}

#[derive(PartialEq, Eq, Debug)]
pub struct Segment<'arena> {
    pub ty: SegmentType<'arena>,
    pub span: SrcSpan,
}
