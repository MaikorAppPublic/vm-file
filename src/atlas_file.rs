use crate::file_utils::ReaderExt;
use crate::read_write_impl::{Readable, Writeable};
use crate::GameFileError;
use crate::GameFileError::InvalidAtlas;
use maikor_platform::constants::{ATLAS_TILE_HEIGHT, ATLAS_TILE_WIDTH};

const ATLAS_SPRITE_SIZE: usize = ATLAS_TILE_HEIGHT * ATLAS_TILE_WIDTH;

pub struct AtlasFile {
    images: Vec<[u8; ATLAS_SPRITE_SIZE]>,
}

impl AtlasFile {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut error = vec![];
        if self.images.len() > ATLAS_SPRITE_SIZE {
            error.push(String::from("Atlas has too many images"));
        }
        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
    }
}

impl Writeable for AtlasFile {
    fn as_bytes(&self) -> Result<Vec<u8>, GameFileError> {
        let mut output = vec![];
        for image in &self.images {
            output.extend_from_slice(image);
        }
        Ok(output)
    }
}

impl Readable for AtlasFile {
    fn from_reader<R: ReaderExt>(reader: &mut R) -> Result<Self, GameFileError>
    where
        Self: Sized,
    {
        let mut bytes = vec![];
        let read_count = reader
            .read_to_end(&mut bytes)
            .map_err(|e| InvalidAtlas(e.to_string()))?;
        if read_count as f64 % ATLAS_SPRITE_SIZE as f64 != 0.0 {
            return Err(InvalidAtlas(format!(
                "Content must be multiple of {}",
                ATLAS_SPRITE_SIZE
            )));
        }
        let mut images = vec![];
        for chunk in bytes.chunks_exact_mut(ATLAS_SPRITE_SIZE) {
            let mut image = [0; ATLAS_SPRITE_SIZE];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    chunk.as_mut_ptr(),
                    image.as_mut_ptr(),
                    ATLAS_SPRITE_SIZE,
                );
            }
            images.push(image);
        }
        Ok(AtlasFile { images })
    }
}

#[cfg(test)]
mod test {
    use crate::atlas_file::AtlasFile;
    use crate::read_write_impl::{Readable, Writeable};
    use std::io::BufReader;

    #[test]
    fn basic_read_write() {
        let image = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ];
        let atlas = AtlasFile {
            images: vec![image],
        };
        let output = atlas.as_bytes().unwrap();
        assert_eq!(image.to_vec(), output);
        let parsed_atlas = AtlasFile::from_reader(&mut BufReader::new(&*image.to_vec())).unwrap();
        assert_eq!(parsed_atlas.images.len(), 1);
        let mut bytes = image.to_vec();
        bytes.extend_from_slice(&image);
        let parsed_atlas = AtlasFile::from_reader(&mut BufReader::new(&*bytes)).unwrap();
        assert_eq!(parsed_atlas.images.len(), 2);
        assert_eq!(parsed_atlas.images[1], image);
    }
}
