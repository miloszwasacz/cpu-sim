use std::io::{self, Read, Write};

#[derive(Clone, Default)]
pub struct StdStream(String);

impl StdStream {
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl Read for StdStream {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        //TODO Proper stand-in for stdin
        Ok(0)
    }
}

impl Write for StdStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.extend(buf.iter().copied().map(char::from));
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
