
use crate::{Arena, ArenaVec, util};

use super::*;

pub struct HttpRequest<'arena, 'request> {
    pub method: HttpMethod,
    pub uri: &'request str,
    pub headers: ArenaVec<'arena, (&'request str, &'request str)>,
    pub body: &'request [u8]
}

impl<'arena, 'request> HttpRequest<'arena, 'request> {

    pub fn parse(arena: &'arena Arena, request: &'request [u8]) -> Option<Self> {
        let mut lines = util::Splitter::new(request, b"\r\n");

        // Start line
        let start_line = str::from_utf8(lines.next()?.trim_ascii()).ok()?;
        let mut start_line = start_line.split_ascii_whitespace();
        let method = match start_line.next()? {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => return None
        };
        let uri = start_line.next()?;
        let _protocol = start_line.next()?;
        if start_line.next().is_some() {
            return None;
        }

        // Headers
        let mut headers = ArenaVec::new();
        while let Some(header_line) = lines.next() {
            let header_line = header_line.trim_ascii();

            if header_line.is_empty() {
                break;
            }

            let mut header_line = util::Splitter::new(header_line, b":");
            let key = str::from_utf8(header_line.next()?).ok()?;
            let value = str::from_utf8(header_line.remainder().trim_ascii()).ok()?;

            headers.push(arena, (key, value));
        }

        // Body
        let body = lines.remainder();

        Some(Self {
            method,
            uri,
            headers,
            body,
        })
    }

}

#[cfg(test)]
mod tests;
