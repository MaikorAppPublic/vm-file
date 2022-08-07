use crate::file_utils::{convert_vec, ReaderExt};
use crate::read_write_impl::{FileReadable, Readable, Writeable};
use crate::GameFileError;
use crate::GameFileError::{FileAccessError, InvalidPalette};
use std::path::Path;

const PALETTE_HEADER: [u8; 2] = [0xFD, 0xA2];
pub const PALETTE_EXT: &str = "mpal";

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Palette {
    pub filepath: Option<String>,
    pub colors: [Color; 16],
}

impl Palette {
    pub fn new(filepath: Option<String>, palette: [Color; 16]) -> Self {
        Self {
            filepath,
            colors: palette,
        }
    }
}

impl Palette {
    pub fn filename(&self) -> Option<String> {
        self.filepath.as_ref().and_then(|path| {
            Path::new(&path)
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_bytes(rgb: [u8; 3]) -> Color {
        Color::new(rgb[0], rgb[1], rgb[2])
    }
}

impl Color {
    pub fn as_bytes(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl Writeable for Palette {
    fn as_bytes(&self) -> Result<Vec<u8>, GameFileError> {
        let mut output = vec![];
        output.extend_from_slice(&PALETTE_HEADER);
        for color in self.colors {
            output.extend_from_slice(&color.as_bytes());
        }
        Ok(output)
    }
}

impl Readable for Palette {
    fn from_reader<R: ReaderExt>(reader: &mut R) -> Result<Self, GameFileError>
    where
        Self: Sized,
    {
        let mut header = [0; 2];
        reader
            .read_exact(&mut header)
            .map_err(|e| FileAccessError(e, "reading palette header data"))?;
        if header != PALETTE_HEADER {
            return Err(InvalidPalette(String::from("Not a palette file")));
        }
        let blocks = reader
            .read_multiple_blocks(3, 16)
            .map_err(|e| FileAccessError(e, "reading palette data"))?;
        let colours: Vec<Color> = blocks
            .into_iter()
            .map(|rgb| Color::from_bytes(convert_vec(rgb)))
            .collect();
        Ok(Palette::new(None, convert_vec(colours)))
    }
}

impl FileReadable for Palette {}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn header_check() {
        let data: Vec<u8> = vec![0, 0];
        let palette = Palette::from_reader(&mut BufReader::new(&*data));
        assert!(palette.is_err());
        assert_eq!(
            palette.err().unwrap().to_string(),
            String::from("Invalid Palette file: Not a palette file")
        )
    }

    #[test]
    fn read() {
        let data: Vec<u8> = vec![
            PALETTE_HEADER[0],
            PALETTE_HEADER[1],
            0,
            0,
            0,
            255,
            255,
            255,
            100,
            100,
            100,
            101,
            101,
            101,
            102,
            102,
            102,
            103,
            104,
            105,
            99,
            88,
            77,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            11,
            22,
            33,
            66,
            55,
            44,
            88,
            77,
            99,
            1,
            10,
            100,
            2,
            20,
            200,
            158,
            158,
            158,
        ];
        let palette = Palette::from_reader(&mut BufReader::new(&*data)).unwrap();
        assert_eq!(palette.filepath, None);
        assert_eq!(palette.colors[0], Color::new(0, 0, 0));
        assert_eq!(palette.colors[1], Color::new(255, 255, 255));
        assert_eq!(palette.colors[2], Color::new(100, 100, 100));
        assert_eq!(palette.colors[3], Color::new(101, 101, 101));
        assert_eq!(palette.colors[4], Color::new(102, 102, 102));
        assert_eq!(palette.colors[5], Color::new(103, 104, 105));
        assert_eq!(palette.colors[6], Color::new(99, 88, 77));
        assert_eq!(palette.colors[7], Color::new(1, 2, 3));
        assert_eq!(palette.colors[8], Color::new(4, 5, 6));
        assert_eq!(palette.colors[9], Color::new(7, 8, 9));
        assert_eq!(palette.colors[10], Color::new(11, 22, 33));
        assert_eq!(palette.colors[11], Color::new(66, 55, 44));
        assert_eq!(palette.colors[12], Color::new(88, 77, 99));
        assert_eq!(palette.colors[13], Color::new(1, 10, 100));
        assert_eq!(palette.colors[14], Color::new(2, 20, 200));
        assert_eq!(palette.colors[15], Color::new(158, 158, 158));
    }
}
