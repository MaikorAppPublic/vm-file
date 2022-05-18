mod file_utils;
mod game_file;
mod game_file_summary;
mod game_header;
mod read_write_impl;

use crate::GameFileError::{FileFormatInvalid, InvalidFileVersion};
use maikor_platform::mem::sizes;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io;
use thiserror::Error;

const ID_HEADER: [u8; 2] = [0xFD, 0xA1];
const FILE_HEADER_LENGTH: usize = 3;
const MAIKOR_HEADER_LENGTH: usize = 16;
const FILE_FORMAT_VER: u8 = 1;
const MIN_FILE_SIZE: u64 = MAIKOR_HEADER_LENGTH as u64 + sizes::CODE_BANK as u64 + 3;
const MAX_FILE_SIZE: u64 = sizes::ATLAS as u64 * 255
    + sizes::CODE_BANK as u64 * 255
    + sizes::RAM_BANK as u64 * 255
    + MIN_FILE_SIZE;

#[derive(Error, Debug)]
pub enum GameFileError {
    #[error("Maikor file not found")]
    FileNotFound(),
    #[error("Maikor file read error, not a file/can't access")]
    NotAFile(),
    #[error("Maikor file access error: {0}")]
    FileAccessError(#[from] io::Error),
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
    InvalidHeader(String),
}

#[derive(Debug, Eq, PartialEq)]
struct GameFileHeader {
    id: [u8; 4],
    build: [u8; 2],
    compiled_for_maikor_version: [u8; 2],
    min_maikor_version: [u8; 2],
    code_bank_count: u8,
    ram_bank_count: u8,
    atlas_bank_count: u8,
    name_length: u8,
    version_length: u8,
    author_length: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct GameFileSummary {
    header: GameFileHeader,
    pub version: String,
    pub name: String,
    pub author: String,
}

#[derive(Eq)]
pub struct GameFile {
    pub id: u32,
    pub build: u16,
    pub compiled_for_maikor_version: u16,
    pub min_maikor_version: u16,
    pub version: String,
    pub name: String,
    pub author: String,
    pub ram_bank_count: usize,
    pub main_code: Vec<u8>,
    pub code_banks: Vec<[u8; sizes::CODE_BANK as usize]>,
    pub atlas_banks: Vec<Vec<u8>>,
    pub button_graphics: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct GameBundle {
    pub name: String,
    pub id: u32,
    pub version: String,
    pub description: String,
    pub image: Vec<u8>,
    pub screenshots: Vec<u8>,
    pub age_rating: String,
    pub special_notes: String,
    pub file: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use maikor_language::input::controller_type;
    use maikor_language::mem::sizes;
    use std::io::BufReader;

    fn rand_u8() -> u8 {
        fastrand::u8(0..=255)
    }

    fn maikor_from_bytes(bytes: Vec<u8>) -> GameFile {
        GameFile::from_reader(BufReader::new(&*bytes)).unwrap()
    }

    fn summary_from_bytes(bytes: Vec<u8>) -> GameFileSummary {
        assert!(bytes.len() > FILE_HEADER_LENGTH + MAIKOR_HEADER_LENGTH);
        GameFileSummary::from_reader(&mut BufReader::new(&*bytes)).unwrap()
    }

    #[test]
    fn basic_write() {
        let maikor_game = GameFile {
            id: 1,
            build: 10,
            compiled_for_maikor_version: 3,
            min_maikor_version: 1,
            version: "1".to_string(),
            name: "Test".to_string(),
            author: "Tester".to_string(),
            ram_bank_count: 0,
            main_code: vec![3; sizes::CODE_BANK as usize],
            code_banks: vec![],
            atlas_banks: vec![],
            button_graphics: vec![
                vec![1; sizes::CONTROLLER_GRAPHICS as usize];
                controller_type::COUNT
            ],
        };
        let bytes = maikor_game.as_bytes().unwrap();
        assert_eq!(
            bytes.len(),
            2 + 1 + 2 + 2 + 4 + 2 + 6 + 4 + 6 + 1 + 8700 + 792
        );
        #[rustfmt::skip]
        assert_eq!(
            &bytes[0..=18],
            &[
                ID_HEADER[0],
                ID_HEADER[1],
                FILE_FORMAT_VER,
                0, 1, 0, 3, //min, compile
                0, 0, 0, 1, //id
                0, 10, //build
                1, 4, 6, //ver len, name len, author len
                0, 0, 0, //bank counts
            ]
        );
        assert_eq!(&bytes[19..=22], "Test".as_bytes());
        assert_eq!(&bytes[23..=28], "Tester".as_bytes());
        assert_eq!(&bytes[29..=29], "1".as_bytes());
        for i in 0..controller_type::COUNT {
            let start = 30 + i * sizes::CONTROLLER_GRAPHICS as usize;
            assert_eq!(
                &bytes[start..start + sizes::CONTROLLER_GRAPHICS as usize],
                &[1; sizes::CONTROLLER_GRAPHICS as usize]
            );
        }
        assert_eq!(&bytes[822..], [3; 8700]);
    }

    #[test]
    fn basic_read() {
        #[rustfmt::skip]
        let mut bytes = vec![
            ID_HEADER[0],
            ID_HEADER[1],
            FILE_FORMAT_VER,
            0, 56, 1, 0, //min, compile
            0, 2, 45, 10, //id
            0, 32, //build
            2, 3, 1, //ver len, name len, author len
            0, 1, 0, //code, ram, atlas bank counts
        ];
        bytes.extend_from_slice("btr".as_bytes());
        bytes.extend_from_slice("t".as_bytes());
        bytes.extend_from_slice("01".as_bytes());
        let summary = summary_from_bytes(bytes.clone());
        assert_eq!(summary.header.id, [0, 2, 45, 10]);
        assert_eq!(summary.header.id(), 142602);
        assert_eq!(summary.header.build, [0, 32]);
        assert_eq!(summary.header.build(), 32);
        assert_eq!(summary.header.version_length, 2);
        assert_eq!(summary.header.name_length, 3);
        assert_eq!(summary.header.author_length, 1);
        assert_eq!(summary.header.code_bank_count, 0);
        assert_eq!(summary.header.ram_bank_count, 1);
        assert_eq!(summary.header.atlas_bank_count, 0);
        assert_eq!(summary.header.min_maikor_version, [0, 56]);
        assert_eq!(summary.header.min_version(), 56);
        assert_eq!(summary.header.compiled_for_maikor_version, [1, 0]);
        assert_eq!(summary.header.compile_version(), 256);
        assert_eq!(summary.name, String::from("btr"));
        assert_eq!(summary.author, String::from("t"));
        assert_eq!(summary.version, String::from("01"));
        bytes.extend_from_slice(&[1; sizes::CONTROLLER_GRAPHICS as usize * controller_type::COUNT]);
        bytes.extend_from_slice(&[0; sizes::CODE_BANK as usize]);
        let game = maikor_from_bytes(bytes);
        assert_eq!(game.id, 142602);
        assert_eq!(game.min_maikor_version, 56);
        assert_eq!(game.compiled_for_maikor_version, 256);
        assert_eq!(game.build, 32);
        assert_eq!(game.ram_bank_count, 1);
        assert!(game.code_banks.is_empty());
        assert!(game.atlas_banks.is_empty());
        assert_eq!(game.version, String::from("01"));
        assert_eq!(game.name, String::from("btr"));
        assert_eq!(game.author, String::from("t"));
        assert_eq!(
            game.button_graphics,
            &[[1; sizes::CONTROLLER_GRAPHICS as usize]; controller_type::COUNT]
        );
        assert_eq!(game.main_code, &[0; sizes::CODE_BANK as usize]);
    }

    #[test]
    fn write_and_read() {
        let maikor_game = GameFile {
            id: 125563563,
            build: 1842,
            compiled_for_maikor_version: 288,
            min_maikor_version: 1,
            version: "3".to_string(),
            name: "WaR Test".to_string(),
            author: "WaR Tester".to_string(),
            ram_bank_count: 2,
            main_code: vec![rand_u8(); sizes::CODE_BANK as usize],
            code_banks: vec![],
            atlas_banks: vec![vec![rand_u8(); sizes::ATLAS as usize], vec![rand_u8(); 500]],
            button_graphics: vec![
                vec![1; sizes::CONTROLLER_GRAPHICS as usize];
                controller_type::COUNT
            ],
        };
        let bytes = maikor_game.as_bytes().unwrap();
        #[rustfmt::skip]
        assert_eq!(
            bytes.len(),
            FILE_HEADER_LENGTH
                + MAIKOR_HEADER_LENGTH
                + 1 + 8 + 10 + 4
                + sizes::CODE_BANK as usize
                + sizes::ATLAS as usize
                + 500
                + sizes::CONTROLLER_GRAPHICS as usize * controller_type::COUNT
        );
        let game = maikor_from_bytes(bytes);
        assert!(PartialEq::eq(&maikor_game, &game));
        assert_eq!(maikor_game, game);
    }
}
