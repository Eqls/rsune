use bytes::{Buf, BytesMut};

pub trait RS2BufExt {
    fn get_string(&mut self) -> String;
}

impl RS2BufExt for BytesMut {
    fn get_string(&mut self) -> String {
        let mut bytes = Vec::new();

        loop {
            match &self.get_u8() {
                10_u8 => break,
                n => bytes.push(n.clone()),
            }
        }

        return String::from_utf8(bytes).expect("invalid string");
    }
}
