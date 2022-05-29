use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub trait ReaderExt: Read {
    fn read_len_string(&mut self) -> Result<String, io::Error> {
        let len = self.read_u8()? as usize;
        self.read_string(len)
    }

    fn read_string(&mut self, len: usize) -> Result<String, io::Error> {
        let mut bytes = vec![0; len];
        self.read_exact(&mut bytes)?;
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }

    fn read_block(&mut self, len: usize) -> Result<Vec<u8>, io::Error> {
        let mut bytes = vec![0; len];
        self.read_exact(&mut bytes)?;
        Ok(bytes)
    }

    fn read_multiple_blocks(
        &mut self,
        block_len: usize,
        block_count: usize,
    ) -> Result<Vec<Vec<u8>>, io::Error> {
        let mut output = vec![];
        for _i in 0..block_count {
            let mut bytes = vec![0; block_len];
            self.read_exact(&mut bytes)?;
            output.push(bytes);
        }
        Ok(output)
    }

    fn read_u8(&mut self) -> Result<u8, io::Error> {
        let mut bytes = vec![0; 1];
        self.read_exact(&mut bytes)?;
        Ok(bytes[0])
    }

    fn read_u16(&mut self) -> Result<u16, io::Error> {
        let mut bytes = vec![0; 2];
        self.read_exact(&mut bytes)?;
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }

    fn read_u32(&mut self) -> Result<u32, io::Error> {
        let mut bytes = vec![0; 4];
        self.read_exact(&mut bytes)?;
        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl ReaderExt for BufReader<File> {}
impl ReaderExt for BufReader<&[u8]> {}
