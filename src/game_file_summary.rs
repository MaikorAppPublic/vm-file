use crate::file_utils::read_string;
use crate::GameFileError::InvalidHeader;
use crate::{GameFileError, GameFileHeader, GameFileSummary};
use std::io::BufRead;

impl GameFileSummary {
    pub fn from_reader<R: BufRead>(mut reader: R) -> Result<GameFileSummary, GameFileError> {
        let header = GameFileHeader::from_reader(&mut reader)?;
        if let Err(text) = header.validate() {
            return Err(InvalidHeader(text));
        }

        let name = read_string(&mut reader, header.name_length as usize)?;
        let author = read_string(&mut reader, header.author_length as usize)?;
        let version = read_string(&mut reader, header.version_length as usize)?;

        Ok(GameFileSummary {
            header,
            version,
            name,
            author,
        })
    }
}

impl GameFileSummary {
    pub fn id(&self) -> u32 {
        self.header.id()
    }

    pub fn version_formatted(&self) -> String {
        format!("Ver {} (#{})", self.version, self.header.build())
    }
}
