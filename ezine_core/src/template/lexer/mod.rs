
use crate::{Arena, ArenaVec};
use super::*;

mod token;
pub use token::*;

mod lexer;
use lexer::*;

fn lex_code<'arena>(arena: &'arena Arena, lexer: &mut Lexer, errors: &mut Errors<'arena>, opening_span: SrcSpan, closing_char: char) -> ArenaVec<'arena, Token<'arena>> {

    let mut tokens = ArenaVec::new();

    while let (start, Some(c)) = lexer.advance_full() {
        if c.is_whitespace() {
            continue; 
        }

        if c == '(' {
            let subtokens = lex_code(arena, lexer, errors, SrcSpan::new(start, lexer.curr_idx()), ')');
            tokens.push(arena, Token {
                ty: TokenType::Paren(subtokens),
                span: SrcSpan::new(start, lexer.curr_idx())
            });
            continue;
        }
        if c == '[' {
            let subtokens = lex_code(arena, lexer, errors, SrcSpan::new(start, lexer.curr_idx()), ']');
            tokens.push(arena, Token {
                ty: TokenType::Brack(subtokens),
                span: SrcSpan::new(start, lexer.curr_idx())
            });
            continue;
        }
        if c == '{' {
            let subtokens = lex_code(arena, lexer, errors, SrcSpan::new(start, lexer.curr_idx()), '}');
            tokens.push(arena, Token {
                ty: TokenType::Brace(subtokens),
                span: SrcSpan::new(start, lexer.curr_idx())
            });
            continue;
        }

        // Closing
        if c == closing_char {
            return tokens;
        }

        errors.report(arena, SrcSpan::new(start, lexer.curr_idx()), "unexpected character.");
    }

    errors.report(arena, opening_span, "unmatched opening bracket.");

    tokens
}

pub fn lex<'arena>(arena: &'arena Arena, src: &str, errors: &mut Errors<'arena>) -> ArenaVec<'arena, Segment<'arena>> {
    let mut segments = ArenaVec::new();

    let mut lexer = Lexer::new(src);
    let mut text_start = 0;

    fn push_text<'arena>(arena: &'arena Arena, segments: &mut ArenaVec<'arena, Segment>, span: SrcSpan) {
        if !span.is_empty() {
            segments.push(arena, Segment {
                ty: SegmentType::Text,
                span
            });
        }
    }

    while let Some(_) = lexer.peek() {
        if let Some(span) = lexer.lookahead_consume("#{") {
            push_text(arena, &mut segments, SrcSpan::new(text_start, span.start()));
            let tokens = lex_code(arena, &mut lexer, errors, span, '}');
            segments.push(arena, Segment {
                ty: SegmentType::Expr(tokens),
                span: SrcSpan::new(span.start(), lexer.curr_idx())
            });
            text_start = lexer.curr_idx(); 
        }
        if let Some(span) = lexer.lookahead_consume("!{") {
            push_text(arena, &mut segments, SrcSpan::new(text_start, span.start()));
            let tokens = lex_code(arena, &mut lexer, errors, span, '}');
            segments.push(arena, Segment {
                ty: SegmentType::Stmt(tokens),
                span: SrcSpan::new(span.start(), lexer.curr_idx())
            });
            text_start = lexer.curr_idx(); 
        }

        lexer.advance();
    }
    push_text(arena, &mut segments, SrcSpan::new(text_start, lexer.curr_idx()));

    segments
}

#[cfg(test)]
mod tests;
