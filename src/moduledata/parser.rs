use super::ModuleData;
use constants::*;

use std;
use std::default::Default;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{io, mem, error, fmt};

#[derive(Copy, Debug)]
#[repr(C)]
struct FileHeader {
    file_id: Magic,
    title: Comment,
    comment: Comment,
    version: u8,
    total_patterns: u8,
    total_tracks: u8,
    cell_size: u8,
    pattern_order: Order,
}

impl Clone for FileHeader {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for FileHeader {
    fn default() -> Self {
        FileHeader {
            file_id: MAGIC,
            title: ShortString { size: 0, string: [0; 40] },
            comment: ShortString { size: 0, string: [0; 40] },
            version: 0,
            total_patterns: 0,
            total_tracks: 0,
            cell_size: 0,
            pattern_order: Order([0xFF; NUM_ORDERS]),
        }
    }
}

impl ModuleData {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<ModuleData, ParserError> {
        let mut file = File::open(path)?;
        let mut header = FileHeader::default();
        unsafe {
            let header: &mut [u8] = std::slice::from_raw_parts_mut(&mut header as *mut _ as _, mem::size_of::<FileHeader>());
            file.read_exact(header)?;
        }
        println!("{:?}", header);
        if header.file_id != MAGIC {
            return Err(ParserError::UnknownFileType);
        }
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum ParserError {
    UnknownFileType,
    IoError(io::Error),
}

impl fmt::Display for ParserError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::ParserError::*;
        match *self {
            UnknownFileType => write!(formatter, "Unknown file type: Header parsing failed."),
            IoError(ref err) => err.fmt(formatter),
        }
    }
}

impl error::Error for ParserError {
    fn description(&self) -> &str {
        use self::ParserError::*;
        match *self {
            UnknownFileType => "Wrong file type",
            IoError(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for ParserError {
    fn from(err: io::Error) -> Self {
        ParserError::IoError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_header_size() {
        assert!(mem::size_of::<FileHeader>() == FILE_HEADER_SIZE);
    }
}
