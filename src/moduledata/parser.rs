use super::ModuleData;

use std::path::Path;
use std::fs::File;
use std::io;

const MAGIC: [u8; 9] = *b"\x08MONOTONE";

impl ModuleData {
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<ModuleData> {
        let file = File::open(path);
        unimplemented!()
    }
}
