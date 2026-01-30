
use crate::{Arena, ArenaVec};

pub struct HttpResponse<'arena, 's> {
    status: u16,
    headers: ArenaVec<'arena, (&'s str, &'s str)>,
    body: &'s [u8]
}

impl<'arena, 's> HttpResponse<'arena, 's> {

    pub fn new(status: u16) -> Self {
        Self {
            status,
            headers: ArenaVec::new(),
            body: &[],
        }
    }

    pub fn add_header(&mut self, arena: &'arena Arena, key: &'s str, val: &'s str) {
        self.headers.push(arena, (key, val));
    }

    pub fn with_header(mut self, arena: &'arena Arena, key: &'s str, val: &'s str) -> Self {
        self.add_header(arena, key, val);
        self
    }

    pub fn set_body(&mut self, body: &'s [u8]) {
        self.body = body;
    }

    pub fn with_body(mut self, body: &'s [u8]) -> Self {
        self.set_body(body);
        self
    }

    pub fn encode(&self) {
        todo!()
    }

}
