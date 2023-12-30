use bytes::Buf;

pub trait RS2BufExt {
    fn get_string(&mut self) -> String;
}

impl<T> RS2BufExt for T
where
    T: Buf,
{
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
