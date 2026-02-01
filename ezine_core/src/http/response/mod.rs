
use crate::{Arena, ArenaVec};

pub struct HttpResponse<'arena, 's> {
    code: u16,
    status: &'s str,
    headers: ArenaVec<'arena, (&'s str, &'s str)>,
    body: &'s [u8]
}

impl<'arena, 's> HttpResponse<'arena, 's> {

    pub fn new(code: u16) -> Self {
        Self {
            code,
            status: match code {
                200 => "OK",
                400 => "Bad Request",
                401 => "Unauthorized",
                403 => "Forbidden",
                404 => "Not Found",
                405 => "Method Not Allowed",
                500 => "Internal Server Error",
                _ => "Unspecified"
            },
            headers: ArenaVec::new(),
            body: &[],
        }
    }

    pub fn set_status(&mut self, status: &'s str) {
        self.status = status;
    }

    pub fn with_status(mut self, status: &'s str) -> Self {
        self.set_status(status);
        self
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

    pub fn write<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        // Start line
        write!(writer, "HTTP/1.1 {} {}\r\n", self.code, self.status)?;

        // Headers
        if !self.body.is_empty() {
            write!(writer, "Content-Length: {}\r\n", self.body.len())?;
        }
        for (key, val) in &self.headers {
            if key.eq_ignore_ascii_case("Content-Length") {
                continue;
            }

            write!(writer, "{key}: {val}\r\n")?;
        }

        if !self.body.is_empty() {
            writer.write(b"\r\n")?;
            writer.write(self.body)?;
        } 

        Ok(())
    }

}
