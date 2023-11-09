use std::{str, string::String};

#[derive(Debug, Clone)]
pub struct Buffer {
    buf: Vec<u8>,
    read_cursor: usize,
    write_cursor: usize,
}

impl Buffer {
    pub fn new(buf: Vec<u8>) -> Buffer {
        Buffer {
            buf,
            read_cursor: 0,
            write_cursor: 0,
        }
    }

    pub fn read_byte(&mut self) -> Result<&u8, &str> {
        if (&self.buf.len() - self.read_cursor) >= 1 {
            let slice = &self.buf[self.read_cursor..self.read_cursor + 1];

            self.read_cursor += 1;

            return Ok(&slice[0]);
        }

        return Err("Limit is out of bounds");
    }

    pub fn read_bytes(&mut self, limit: usize) -> Result<&[u8], &str> {
        if (&self.buf.len() - self.read_cursor) >= limit {
            let slice = &self.buf[self.read_cursor..self.read_cursor + limit];

            self.read_cursor += limit;

            return Ok(slice);
        }

        return Err("Limit is out of bounds");
    }

    pub fn read_string(&mut self) -> Result<String, &str> {
        let mut bytes = Vec::new();

        loop {
            match &self.read_byte().clone() {
                Ok(10_u8) => break,
                Ok(&n) => bytes.push(n.clone()),
                Err(_) => break,
            }
        }

        return match String::from_utf8(bytes) {
            Ok(v) => Ok(v),
            Err(e) => Err("Cannot convert to string"),
        };
    }
}
