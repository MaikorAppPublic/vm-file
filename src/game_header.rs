use crate::file_utils::ReaderExt;
use crate::read_write_impl::{convert_string, Readable, Writeable};
use crate::GameFileError::FileAccessError;
use crate::{
    FileFormatInvalid, GameFileError, GameFileHeader, InvalidFileVersion, FILE_FORMAT_VER,
    ID_HEADER, MAX_STRING_LEN,
};
use maikor_platform::input::controller_type;

impl GameFileHeader {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        build: u32,
        compiled_for_maikor_version: u16,
        min_maikor_version: u16,
        ram_bank_count: u8,
        name: String,
        version: String,
        author: String,
        code_bank_count: u8,
        atlas_bank_count: u8,
        controller_graphics_bank_count: u8,
    ) -> Self {
        Self {
            id,
            build,
            compiled_for_maikor_version,
            min_maikor_version,
            ram_bank_count,
            name,
            version,
            author,
            code_bank_count,
            atlas_bank_count,
            controller_graphics_bank_count,
        }
    }
}

impl GameFileHeader {
    pub fn validate(&self) -> Result<(), String> {
        let mut error = String::new();

        if self.build == 0 {
            error.push_str("Build ver must be at least 1\n");
        }
        if self.compiled_for_maikor_version < self.min_maikor_version {
            error.push_str("Minimum maikor version must <= compile version\n");
        }
        if self.author.trim().is_empty() {
            error.push_str("Author must have at least one character\n");
        } else if self.author.trim().len() > MAX_STRING_LEN {
            error.push_str("Author is too long, max of 255 characters\n");
        }
        if self.name.trim().is_empty() {
            error.push_str("Name must have at least one character\n");
        } else if self.name.trim().len() > MAX_STRING_LEN {
            error.push_str("Name is too long, max of 255 characters\n");
        }
        if self.version.trim().is_empty() {
            error.push_str("Version must have at least one character\n");
        } else if self.version.trim().len() > MAX_STRING_LEN {
            error.push_str("Version is too long, max of 255 characters\n");
        }
        if self.id.trim().is_empty() {
            error.push_str("ID must have at least one character\n");
        } else if self.id.trim().len() > MAX_STRING_LEN {
            error.push_str("ID is too long, max of 255 characters\n");
        }
        if self.controller_graphics_bank_count as usize != controller_type::COUNT {
            error.push_str(&format!(
                "Incorrect number of controller graphics (expected {}, found {})\n",
                controller_type::COUNT,
                self.controller_graphics_bank_count
            ));
        }
        if self.atlas_bank_count == 0 {
            error.push_str("Must have at least one atlas bank\n");
        }

        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
    }
}

impl Readable for GameFileHeader {
    fn from_reader<R: ReaderExt>(reader: &mut R) -> Result<GameFileHeader, GameFileError> {
        let file_header = reader
            .read_u16()
            .map_err(|e| FileAccessError(e, "reading file header"))?;
        let file_ver = reader
            .read_u8()
            .map_err(|e| FileAccessError(e, "reading file ver"))?;
        if file_header != u16::from_be_bytes([ID_HEADER[0], ID_HEADER[1]]) {
            return Err(FileFormatInvalid());
        }
        if file_ver != FILE_FORMAT_VER {
            return Err(InvalidFileVersion(file_ver));
        }

        let min_maikor_version = reader
            .read_u16()
            .map_err(|e| FileAccessError(e, "reading min ver"))?;
        let compiled_for_maikor_version = reader
            .read_u16()
            .map_err(|e| FileAccessError(e, "reading compiled ver"))?;
        let build = reader
            .read_u32()
            .map_err(|e| FileAccessError(e, "reading build"))?;
        let id = reader
            .read_len_string()
            .map_err(|e| FileAccessError(e, "reading id"))?;
        let name = reader
            .read_len_string()
            .map_err(|e| FileAccessError(e, "reading name"))?;
        let version = reader
            .read_len_string()
            .map_err(|e| FileAccessError(e, "reading version"))?;
        let author = reader
            .read_len_string()
            .map_err(|e| FileAccessError(e, "reading author"))?;
        let code_bank_count = reader
            .read_u8()
            .map_err(|e| FileAccessError(e, "reading code bank count"))?;
        let ram_bank_count = reader
            .read_u8()
            .map_err(|e| FileAccessError(e, "reading ram bank count"))?;
        let atlas_bank_count = reader
            .read_u8()
            .map_err(|e| FileAccessError(e, "reading atlas bank count"))?;
        let controller_graphics_bank_count = reader
            .read_u8()
            .map_err(|e| FileAccessError(e, "reading controller graphics count"))?;

        Ok(GameFileHeader::new(
            id,
            build,
            compiled_for_maikor_version,
            min_maikor_version,
            ram_bank_count,
            name,
            version,
            author,
            code_bank_count,
            atlas_bank_count,
            controller_graphics_bank_count,
        ))
    }
}

impl Writeable for GameFileHeader {
    fn as_bytes(&self) -> Result<Vec<u8>, GameFileError> {
        let mut output = vec![];
        output.extend_from_slice(&ID_HEADER);
        output.push(FILE_FORMAT_VER);
        output.extend_from_slice(&self.min_maikor_version.to_be_bytes());
        output.extend_from_slice(&self.compiled_for_maikor_version.to_be_bytes());
        output.extend_from_slice(&self.build.to_be_bytes());
        output.extend_from_slice(&convert_string("ID", &self.id)?);
        output.extend_from_slice(&convert_string("Name", &self.name)?);
        output.extend_from_slice(&convert_string("Version", &self.version)?);
        output.extend_from_slice(&convert_string("Author", &self.author)?);
        output.push(self.code_bank_count);
        output.push(self.ram_bank_count);
        output.push(self.atlas_bank_count);
        output.push(self.controller_graphics_bank_count);

        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use crate::read_write_impl::{Readable, Writeable};
    use crate::GameFileHeader;
    use std::io::BufReader;

    #[test]
    #[rustfmt::skip]
    fn test_write() {
        let header = GameFileHeader::new(
            String::from("com.raybritton.test"),
            12414,
            16,
            1,
            0,
            String::from("Test"),
            String::from("1.1.0"),
            String::from("Ray Britton"),
            1,
            4,
            9,
        );

        assert_eq!(
            header.as_bytes().unwrap(),
            [
                253, 161,       //header
                1,              //file ver
                0, 1,           //min ver
                0, 16,          //target ver
                0, 0, 48, 126,  //build
                19,             //id len
                99, 111, 109, 46, 114, 97, 121, 98, 114, 105, 116, 116, 111, 110, 46, 116, 101, 115, 116, //id 
                4,              //name len
                84, 101, 115, 116, //name
                5,              //ver len
                49, 46, 49, 46, 48,  //ver
                11,             //author len
                82, 97, 121, 32, 66, 114, 105, 116, 116, 111, 110, //author 
                1,              //code banks
                0,              //ram banks
                4,              //atlas banks
                9               //controller banks
            ]
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_read() {
        let bytes = vec![
            253, 161, //header
            1,   //file ver
            1, 0, //min ver
            2, 0, //target ver
            0, 1, 5, 2, //build
            6, //id len
            66, 89, 100, 65, 53, 70, //id
            5,  //name len
            84, 101, 115, 116, 33, //name
            2,  //ver len
            118, 49, //ver
            3,  //author len
            82, 97, 121, //author
            2,   //code banks
            1,   //ram banks
            88,  //atlas banks
            9    //controller banks
        ];
        let mut reader = BufReader::new(&*bytes);

        let header = GameFileHeader::from_reader(&mut reader).unwrap();
        
        assert_eq!(header.min_maikor_version, 256);
        assert_eq!(header.compiled_for_maikor_version, 512);
        assert_eq!(header.build, 66818);
        assert_eq!(header.id, String::from("BYdA5F"));
        assert_eq!(header.name, String::from("Test!"));
        assert_eq!(header.code_bank_count, 2);
        assert_eq!(header.ram_bank_count, 1);
        assert_eq!(header.atlas_bank_count, 88);
        assert_eq!(header.controller_graphics_bank_count, 9);
        assert_eq!(header.version, String::from("v1"));
        assert_eq!(header.author, String::from("Ray"));
    }

    #[test]
    fn test_read_write() {
        let header = GameFileHeader::new(
            String::from("TEST TEST TEST TEST TEST"),
            12511,
            12,
            3341,
            10,
            String::from("TE ST APP TEST APP TESPT"),
            String::from("v1.1.2351b"),
            String::from("Ray Britton testing"),
            12,
            100,
            25,
        );

        let bytes = header.as_bytes().unwrap();

        let parsed_header = GameFileHeader::from_reader(&mut BufReader::new(&*bytes)).unwrap();

        assert_eq!(header, parsed_header);
    }
}
