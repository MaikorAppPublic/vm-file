use crate::GameFileError::{FileNotFound, FileTooLarge, FileTooSmall, NotAFile};
use crate::{
    FileFormatInvalid, GameFile, GameFileError, GameFileSummary, InvalidFileVersion,
    FILE_FORMAT_VER, FILE_HEADER_LENGTH, ID_HEADER, MAX_FILE_SIZE, MIN_FILE_SIZE,
};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn create_reader(path: &Path) -> Result<BufReader<File>, GameFileError> {
    validate_file(path)?;
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

fn create_writer(path: &Path) -> Result<BufWriter<File>, GameFileError> {
    let file = File::open(path)?;
    let writer = BufWriter::new(file);
    Ok(writer)
}

pub fn get_file_size(path: &Path) -> u64 {
    if let Ok(data) = path.metadata() {
        data.len()
    } else {
        0
    }
}

fn validate_header<R: BufRead>(reader: &mut R) -> Result<(), GameFileError> {
    let mut header_bytes: [u8; FILE_HEADER_LENGTH] = [0; FILE_HEADER_LENGTH];
    reader.read_exact(&mut header_bytes)?;
    if header_bytes[0..=1] != ID_HEADER {
        return Err(FileFormatInvalid());
    }
    if header_bytes[2] != FILE_FORMAT_VER {
        return Err(InvalidFileVersion(header_bytes[2]));
    }
    Ok(())
}

fn validate_file(path: &Path) -> Result<(), GameFileError> {
    if !path.exists() {
        return Err(FileNotFound());
    }
    if !path.is_file() {
        return Err(NotAFile());
    }
    let size = get_file_size(path);
    if size > MAX_FILE_SIZE {
        return Err(FileTooLarge(size));
    }
    if size < MIN_FILE_SIZE {
        return Err(FileTooSmall());
    }
    Ok(())
}

pub trait Readable {
    fn read(path: &Path) -> Result<Self, GameFileError>
    where
        Self: Sized;
}

impl Readable for GameFileSummary {
    fn read(path: &Path) -> Result<Self, GameFileError> {
        validate_file(path)?;
        let reader = create_reader(path)?;
        let header = GameFileSummary::from_reader(reader)?;
        Ok(header)
    }
}

impl Readable for GameFile {
    fn read(path: &Path) -> Result<Self, GameFileError> {
        validate_file(path)?;
        let mut reader = create_reader(path)?;
        validate_header(&mut reader)?;
        let header = GameFile::from_reader(reader)?;
        Ok(header)
    }
}

impl GameFile {
    pub fn write(&self, path: &Path) -> Result<(), GameFileError> {
        let mut writer = create_writer(path)?;
        writer.write_all(&self.as_bytes()?)?;
        Ok(())
    }
}
