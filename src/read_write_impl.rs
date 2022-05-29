use crate::file_utils::ReaderExt;
use crate::GameFileError::{
    FieldTooLong, FileAccessError, FileNotFound, FileTooLarge, FileTooSmall, NotAFile,
};
use crate::{
    GameFile, GameFileError, GameFileHeader, MAX_FILE_SIZE, MAX_STRING_LEN, MIN_FILE_SIZE,
};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

fn create_reader(path: &Path) -> Result<BufReader<File>, GameFileError> {
    validate_file(path)?;
    let file = File::open(path).map_err(|e| FileAccessError(e, "reading file"))?;
    let reader = BufReader::new(file);
    Ok(reader)
}

fn create_writer(path: &Path) -> Result<BufWriter<File>, GameFileError> {
    let file = File::open(path).map_err(|e| FileAccessError(e, "writing file"))?;
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

pub trait FileReadable {
    fn read(path: &Path) -> Result<Self, GameFileError>
    where
        Self: Sized + Readable,
    {
        validate_file(path)?;
        let mut reader = create_reader(path)?;
        let header = Self::from_reader(&mut reader)?;
        Ok(header)
    }
}

pub trait Readable {
    fn from_reader<R: ReaderExt>(reader: &mut R) -> Result<Self, GameFileError>
    where
        Self: Sized;
}

pub trait Writeable {
    fn as_bytes(&self) -> Result<Vec<u8>, GameFileError>;
}

impl FileReadable for GameFileHeader {}

impl FileReadable for GameFile {}

impl GameFile {
    pub fn write(&self, path: &Path) -> Result<(), GameFileError> {
        let mut writer = create_writer(path)?;
        writer
            .write_all(&self.as_bytes()?)
            .map_err(|e| FileAccessError(e, "writing file"))?;
        Ok(())
    }
}

pub fn convert_string(field_name: &'static str, str: &str) -> Result<Vec<u8>, GameFileError> {
    let len = str.trim().len();
    if len > MAX_STRING_LEN {
        return Err(FieldTooLong(field_name, MAX_STRING_LEN, len));
    }
    let mut bytes = str.as_bytes().to_vec();
    bytes.insert(0, len as u8);
    Ok(bytes)
}
