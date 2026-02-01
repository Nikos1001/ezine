
use crate::{Arena, ArenaVec, util};

use super::*;

pub struct HttpRequest<'arena, 'request> {
    pub method: HttpMethod,
    pub uri: &'request str,
    pub headers: ArenaVec<'arena, (&'request str, &'request str)>,
    pub body: &'request [u8]
}

impl<'arena, 'request> HttpRequest<'arena, 'request> where 'arena: 'request {

    pub fn read<R: std::io::Read>(arena: &'arena Arena, reader: R) -> Option<Self> {
        let mut reader = util::BufReader::new(arena, reader);

        // Start line
        let method = match reader.read_token(arena).ok()? {
            b"GET" => HttpMethod::GET,
            b"POST" => HttpMethod::POST,
            b"PUT" => HttpMethod::PUT,
            b"DELETE" => HttpMethod::DELETE,
            _ => return None
        };
        let uri = reader.read_token(arena).ok()?;
        let uri = str::from_utf8(uri).ok()?;
        if uri.is_empty() {
            return None;
        }
        reader.skip_after(b'\n').ok()?;

        // Headers
        let mut headers = ArenaVec::new();
        let mut content_length = 0;
        loop {
            let line = reader.read_until(arena, b'\n').ok()?.trim_ascii();
            reader.next().ok()?; // Skip newline
            if line.is_empty() {
                break;
            }

            let colon_pos = line.iter().position(|c| *c == b':')?;
            let key = str::from_utf8(line[..colon_pos].trim_ascii()).ok()?;
            let val = str::from_utf8(line[colon_pos + 1..].trim_ascii()).ok()?;

            headers.push(arena, (key, val));

            // Special headers
            if key.eq_ignore_ascii_case("Content-Length") {
                if let Ok(length) = usize::from_str_radix(val, 10) {
                    content_length = length;
                }
            }
        }

        // Body
        let body = reader.read_n(arena, content_length).ok()?;

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
