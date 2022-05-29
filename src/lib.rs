mod constants;
mod file_utils;
pub mod game_file;
pub mod game_header;
pub mod read_write_impl;

use crate::constants::mem::*;
use crate::GameFileError::{FileFormatInvalid, InvalidFileVersion};
use std::fmt::Debug;
use std::io;
use thiserror::Error;

const ID_HEADER: [u8; 2] = [0xFD, 0xA1];
const MAIKOR_HEADER_LENGTH: usize = 16;
const FILE_FORMAT_VER: u8 = 1;
const MAX_STRING_LEN: usize = 255;
const MIN_FILE_SIZE: u64 = MAIKOR_HEADER_LENGTH as u64 + CODE_BANK as u64 + 3;
const MAX_FILE_SIZE: u64 = ATLAS_BANK as u64 * 255
    + CODE_BANK as u64 * 255
    + RAM_BANK as u64 * 255
    + CONTROLLER_GRAPHICS_BANK as u64 * 9
    + MIN_FILE_SIZE;

#[derive(Error, Debug)]
pub enum GameFileError {
    #[error("Maikor file not found")]
    FileNotFound(),
    #[error("Maikor file read error, not a file/can't access")]
    NotAFile(),
    #[error("Maikor read access error, for field {1}: {0}")]
    FileAccessError(#[source] io::Error, &'static str),
    #[error("Maikor file too large. File was {0}, max is {MAX_FILE_SIZE}")]
    FileTooLarge(u64),
    #[error("Maikor file too small. This may be not be a valid Maikor file.")]
    FileTooSmall(),
    #[error("Not a Maikor game file")]
    FileFormatInvalid(),
    #[error("Unsupported Maikor game file version, was {0} and must be {FILE_FORMAT_VER}")]
    InvalidFileVersion(u8),
    #[error("Invalid/corrupt Maikor file")]
    InvalidMaikorFile(),
    #[error("Invalid atlas banks")]
    InvalidAtlasBanks(),
    #[error("Header validation failed:\n{0}")]
    InvalidHeader(&'static str),
    #[error("{0} field too long, max is {1} and was {2}")]
    FieldTooLong(&'static str, usize, usize),
}

#[derive(Debug, Eq, PartialEq)]
pub struct GameFileHeader {
    ///Unique ID for app
    pub id: String,
    ///Build number of app (must be equal or higher than installed version)
    pub build: u32,
    ///Target Maikor version (used for compatibility)
    pub compiled_for_maikor_version: u16,
    ///Minimum supported Maikor Version (used for compatibility)
    pub min_maikor_version: u16,
    ///Number of RAM banks needed by game
    pub ram_bank_count: u8,
    ///Game name
    pub name: String,
    ///Game version
    pub version: String,
    ///Game author
    pub author: String,
    pub code_bank_count: u8,
    pub atlas_bank_count: u8,
    pub controller_graphics_bank_count: u8,
}

/// Full game file
pub struct GameFile {
    pub header: GameFileHeader,
    ///Main code data
    pub main_code: [u8; CODE_BANK],
    ///Code bank data
    pub code_banks: Vec<[u8; CODE_BANK]>,
    ///Atlas bank data
    pub atlases: Vec<[u8; ATLAS_BANK]>,
    ///Controller graphics data
    pub controller_graphics: Vec<[u8; CONTROLLER_GRAPHICS_BANK]>,
}
