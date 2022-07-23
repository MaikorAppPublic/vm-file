use crate::read_write_impl::validate_file;
use crate::GameFileError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Manifest {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub build: u32,
    pub main_code: String,
    pub min_maikor_version: u16,
    pub code_files: Vec<String>,
    pub atlas_files: Vec<String>,
    pub ram_banks: u8,
}

impl Manifest {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Manifest, GameFileError> {
        let path = path.as_ref();
        validate_file(path, false)?;
        let text = fs::read_to_string(path)
            .map_err(|e| GameFileError::FileAccessError(e, "reading manifest"))?;
        let manifest = serde_json::from_str(&text)
            .map_err(|e| GameFileError::ManifestParsingError(e.to_string()))?;
        Ok(manifest)
    }

    pub fn from_string(text: &str) -> Result<Manifest, GameFileError> {
        let manifest = serde_json::from_str(text)
            .map_err(|e| GameFileError::ManifestParsingError(e.to_string()))?;
        Ok(manifest)
    }
}
