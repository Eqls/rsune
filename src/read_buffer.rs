use std::{str, string::String};

#[derive(Debug, Clone)]
pub struct ReadBuffer {
    buf: Vec<u8>,
    cursor: usize,
}

impl ReadBuffer {
    pub fn new(buf: Vec<u8>) -> ReadBuffer {
        ReadBuffer {
            buf,
            cursor: 0
        }
    }

    pub fn read_byte(&mut self) -> Result<&u8, &str> {
        if (&self.buf.len() - self.cursor) >= 1 {
            let slice = &self.buf[self.cursor..self.cursor + 1];

            self.cursor += 1;

            return Ok(&slice[0]);
        }

        return Err("Limit is out of bounds");
    }

    pub fn read_bytes(&mut self, limit: usize) -> Result<&[u8], &str> {
        if (&self.buf.len() - self.cursor) >= limit {
            let slice = &self.buf[self.cursor..self.cursor + limit];

            self.cursor += limit;

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
