use crate::{
    FileFormatInvalid, GameFileError, GameFileHeader, InvalidFileVersion, FILE_FORMAT_VER,
    FILE_HEADER_LENGTH, ID_HEADER, MAIKOR_HEADER_LENGTH,
};
use std::io::BufRead;

impl GameFileHeader {
    pub fn id(&self) -> u32 {
        u32::from_be_bytes(self.id)
    }

    pub fn build(&self) -> u16 {
        u16::from_be_bytes(self.build)
    }

    pub fn min_version(&self) -> u16 {
        u16::from_be_bytes(self.min_maikor_version)
    }

    pub fn compile_version(&self) -> u16 {
        u16::from_be_bytes(self.compiled_for_maikor_version)
    }
}

impl GameFileHeader {
    pub fn from_reader<R: BufRead>(reader: &mut R) -> Result<GameFileHeader, GameFileError> {
        let mut file_header = [0; FILE_HEADER_LENGTH];
        reader.read_exact(&mut file_header)?;
        if file_header[0..=1] != ID_HEADER {
            return Err(FileFormatInvalid());
        }
        if file_header[2] != FILE_FORMAT_VER {
            return Err(InvalidFileVersion(file_header[2]));
        }

        let mut header_bytes = [0; MAIKOR_HEADER_LENGTH];
        reader.read_exact(&mut header_bytes)?;

        Ok(GameFileHeader {
            min_maikor_version: [header_bytes[0], header_bytes[1]],
            compiled_for_maikor_version: [header_bytes[2], header_bytes[3]],
            id: [
                header_bytes[4],
                header_bytes[5],
                header_bytes[6],
                header_bytes[7],
            ],
            build: [header_bytes[8], header_bytes[9]],
            version_length: header_bytes[10],
            name_length: header_bytes[11],
            author_length: header_bytes[12],
            code_bank_count: header_bytes[13],
            ram_bank_count: header_bytes[14],
            atlas_bank_count: header_bytes[15],
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        let mut error = String::new();

        if self.build() == 0 {
            error.push_str("Build ver must be at least 1\n");
        }
        if self.compiled_for_maikor_version < self.min_maikor_version {
            error.push_str("Minimum maikor version must <= compile version\n");
        }
        if self.author_length == 0 {
            error.push_str("Author length must have at least one character\n");
        }
        if self.name_length == 0 {
            error.push_str("Name length must have at least one character\n");
        }
        if self.version_length == 0 {
            error.push_str("Version length must have at least one character\n");
        }

        //TODO check length using code_banks and atlas_banks

        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
    }
}
