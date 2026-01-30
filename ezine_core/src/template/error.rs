
use crate::{Arena, ArenaVec};

use super::*;

pub struct Error<'arena> {
    msg: &'arena str,
    span: SrcSpan 
}

pub struct Errors<'arena> {
    errors: ArenaVec<'arena, Error<'arena>>
}

impl<'arena> Errors<'arena> {

    pub fn new() -> Self {
        Self {
            errors: ArenaVec::new(),
        }
    }

    pub fn report(&mut self, arena: &'arena Arena, span: SrcSpan, msg: &'arena str) {
        self.errors.push(arena, Error { msg, span });
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

}
