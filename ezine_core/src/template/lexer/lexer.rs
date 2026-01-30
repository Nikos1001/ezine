
use std::str::CharIndices;

use crate::SrcSpan;

const LOOKAHEAD_SIZE: usize = 2;

pub(super) struct Lexer<'src> {
    src: &'src str,
    iter: CharIndices<'src>,
    lookahead: [(usize, Option<char>); LOOKAHEAD_SIZE],
}

impl<'src> Lexer<'src> {

    fn char_iter_next(src: &str, iter: &mut CharIndices) -> (usize, Option<char>) {
        if let Some((idx, char)) = iter.next() {
            (idx, Some(char))
        } else {
            (src.len(), None)
        }
    }

    pub(super) fn new(src: &'src str) -> Self {
        let mut iter = src.char_indices();
        let lookahead = [
            Self::char_iter_next(src, &mut iter),
            Self::char_iter_next(src, &mut iter)
        ];
        Self {
            src,
            iter,
            lookahead,
        }
    }

    pub(super) fn peek_full(&self) -> (usize, Option<char>) {
        self.lookahead[0]
    }

    pub(super) fn peek(&self) -> Option<char> {
        self.peek_full().1
    }

    pub(super) fn advance_full(&mut self) -> (usize, Option<char>) {
        let res = self.lookahead[0];
        for i in 0..(LOOKAHEAD_SIZE - 1) {
            self.lookahead[i] = self.lookahead[i + 1];
        }
        self.lookahead[LOOKAHEAD_SIZE - 1] = Self::char_iter_next(self.src, &mut self.iter);
        res
    }

    pub(super) fn advance(&mut self) -> Option<char> {
        self.advance_full().1
    }

    pub(super) fn lookahead_matches(&self, str: &str) -> bool {
        for (i, char) in str.chars().enumerate() {
            if i >= LOOKAHEAD_SIZE {
                panic!("lookahead not long enough.");
            }
            if let Some(lookahead_char) = self.lookahead[i].1 {
                if char != lookahead_char {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub(super) fn lookahead_consume(&mut self, str: &str) -> Option<SrcSpan> {
        let start = self.curr_idx();
        if self.lookahead_matches(str) {
            for _ in str.chars() {
                self.advance();
            }
            Some(SrcSpan::new(start, self.curr_idx()))
        } else {
            None
        }
    }

    pub(super) fn curr_idx(&self) -> usize {
        self.peek_full().0
    }

}
