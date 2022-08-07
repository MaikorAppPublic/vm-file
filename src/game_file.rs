use crate::constants::mem::*;
use crate::file_utils::{convert_vec, read_sized_blocks, ReaderExt};
use crate::read_write_impl::{Readable, Writeable};
use crate::GameFileError::FileAccessError;
use crate::{GameFile, GameFileError, GameFileHeader};

impl GameFile {
    pub fn new(
        header: GameFileHeader,
        main_code: [u8; MAIN_CODE],
        code_banks: Vec<[u8; CODE_BANK]>,
        atlases: Vec<[u8; ATLAS_BANK]>,
    ) -> Self {
        Self {
            header,
            main_code,
            code_banks,
            atlases,
        }
    }
}

impl Readable for GameFile {
    fn from_reader<R: ReaderExt>(reader: &mut R) -> Result<GameFile, GameFileError> {
        let header = GameFileHeader::from_reader(reader)?;
        let main_code = reader
            .read_block(CODE_BANK)
            .map_err(|e| FileAccessError(e, "reading main code"))?;
        let code_banks = read_sized_blocks(reader, header.code_bank_count as usize)?;
        let atlas_banks = read_sized_blocks(reader, header.atlas_bank_count as usize)?;
        Ok(GameFile::new(
            header,
            convert_vec(main_code),
            code_banks,
            atlas_banks,
        ))
    }
}

impl Writeable for GameFile {
    fn as_bytes(&self) -> Result<Vec<u8>, GameFileError> {
        let mut output = self.header.as_bytes()?;
        output.extend_from_slice(&self.main_code);
        for bank in &self.code_banks {
            output.extend_from_slice(bank);
        }
        for bank in &self.atlases {
            output.extend_from_slice(bank);
        }

        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use crate::read_write_impl::Writeable;
    use crate::{GameFile, GameFileHeader, ATLAS_BANK, CODE_BANK, MAIN_CODE};

    #[test]
    #[rustfmt::skip]
    fn test_write() {
        let header = GameFileHeader::new(String::from("1"), 1, 1, 1,0, String::from("a"), String::from("b"), String::from("c"), 1, 1);
        let file = GameFile::new(header, [1; MAIN_CODE], vec![[2; CODE_BANK]], vec![[3; ATLAS_BANK]]);
        
        let bytes  = file.as_bytes().unwrap();
        assert_eq!(bytes.len(), MAIN_CODE + CODE_BANK + ATLAS_BANK + 22);
    }
}
