use crate::file_utils::{read_block, read_multiple_blocks, read_string};
use crate::GameFileError::InvalidHeader;
use crate::{GameFile, GameFileError, GameFileHeader, FILE_FORMAT_VER, ID_HEADER};
use maikor_language::mem::sizes;
use std::fmt::{Debug, Formatter};
use std::io::{BufRead, BufWriter, Write};
use std::{fmt, io};

impl GameFile {
    pub fn from_reader<R: BufRead>(mut reader: R) -> Result<GameFile, GameFileError> {
        let header = GameFileHeader::from_reader(&mut reader)?;
        if let Err(text) = header.validate() {
            return Err(InvalidHeader(text));
        }

        let name = read_string(&mut reader, header.name_length as usize)?;
        let author = read_string(&mut reader, header.author_length as usize)?;
        let version = read_string(&mut reader, header.version_length as usize)?;
        let atlas_lengths: Vec<u16> =
            read_multiple_blocks(&mut reader, 2, header.atlas_bank_count as usize)?
                .iter()
                .map(|arr| u16::from_be_bytes([arr[0], arr[1]]))
                .collect();
        let main_code = read_block(&mut reader, sizes::CODE_BANK as usize)?;
        let code_banks = read_multiple_blocks(
            &mut reader,
            sizes::CODE_BANK as usize,
            header.code_bank_count as usize,
        )?
        .iter()
        .map(|list| list.as_slice().try_into().unwrap())
        .collect();
        let mut atlas_banks = vec![];
        for len in atlas_lengths {
            atlas_banks.push(read_block(&mut reader, len as usize)?);
        }

        Ok(GameFile {
            id: header.id(),
            build: header.build(),
            compiled_for_maikor_version: header.compile_version(),
            min_maikor_version: header.min_version(),
            version,
            name,
            author,
            ram_bank_count: header.ram_bank_count as usize,
            main_code,
            code_banks,
            atlas_banks,
        })
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes = vec![];
        let mut writer = BufWriter::new(&mut bytes);
        writer.write_all(&ID_HEADER)?;
        writer.write_all(&[FILE_FORMAT_VER])?;
        writer.write_all(&self.min_maikor_version.to_be_bytes())?;
        writer.write_all(&self.compiled_for_maikor_version.to_be_bytes())?;
        writer.write_all(&self.id.to_be_bytes())?;
        writer.write_all(&self.build.to_be_bytes())?;
        writer.write_all(&[
            self.version.len() as u8,
            self.name.len() as u8,
            self.author.len() as u8,
            self.code_banks.len() as u8,
            self.ram_bank_count as u8,
            self.atlas_banks.len() as u8,
        ])?;
        writer.write_all(self.name.as_bytes())?;
        writer.write_all(self.author.as_bytes())?;
        writer.write_all(self.version.as_bytes())?;
        writer.write_all(
            &self
                .atlas_banks
                .iter()
                .flat_map(|arr| (arr.len() as u16).to_be_bytes())
                .collect::<Vec<u8>>(),
        )?;
        writer.write_all(&self.main_code)?;
        for bank in &self.code_banks {
            writer.write_all(bank)?;
        }
        for bank in &self.atlas_banks {
            writer.write_all(bank)?;
        }
        drop(writer);
        Ok(bytes)
    }
}

impl Debug for GameFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{: >10} - {} by {}, {} (#{})",
            self.id, self.name, self.author, self.version, self.build
        )
    }
}

impl PartialEq for GameFile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.build == other.build
    }
}
