use std::io::Cursor;
use std::{str, string::String};

#[derive(Debug, Clone)]
pub struct ReadBuffer {
    inner: Vec<u8>,
    cursor: usize,
}

impl ReadBuffer {
    #[must_use]
    pub fn new(inner: Vec<u8>) -> ReadBuffer {
        ReadBuffer { inner, cursor: 0 }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }

    pub fn read_byte(&mut self) -> Result<&u8, &str> {
        if (&self.inner.len() - self.cursor) >= 1 {
            let slice = &self.inner[self.cursor..self.cursor + 1];

            self.cursor += 1;

            return Ok(&slice[0]);
        }

        return Err("Limit is out of bounds");
    }

    pub fn read_bytes(&mut self, limit: usize) -> Result<&[u8], &str> {
        if (&self.inner.len() - self.cursor) >= limit {
            let slice = &self.inner[self.cursor..self.cursor + limit];

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

    pub fn read_int(&mut self) -> u32 {
        return u32::from_be_bytes(self.inner[self.cursor..4 + self.cursor].try_into().unwrap());
    }
}
