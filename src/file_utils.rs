use std::io;
use std::io::BufRead;

pub fn read_string<R: BufRead>(reader: &mut R, len: usize) -> Result<String, io::Error> {
    let mut bytes = vec![0; len];
    reader.read_exact(&mut bytes)?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

pub fn read_block<R: BufRead>(reader: &mut R, len: usize) -> Result<Vec<u8>, io::Error> {
    let mut bytes = vec![0; len];
    reader.read_exact(&mut bytes)?;
    Ok(bytes)
}

pub fn read_multiple_blocks<R: BufRead>(
    reader: &mut R,
    block_len: usize,
    block_count: usize,
) -> Result<Vec<Vec<u8>>, io::Error> {
    let mut output = vec![];
    for _i in 0..block_count {
        let mut bytes = vec![0; block_len];
        reader.read_exact(&mut bytes)?;
        output.push(bytes);
    }
    Ok(output)
}
