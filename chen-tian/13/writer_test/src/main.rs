use std::{fmt::{Debug}, io::{Write}};

struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    fn new() -> Self {
        Self { 
            buf: Vec::with_capacity(1024),
        }
    }
}

impl Debug for BufBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

fn main() {
    let mut b = BufBuilder::new();
    b.write_all(b"hello world!").unwrap();
    println!("{:?}", b);
}
