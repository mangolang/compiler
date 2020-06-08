use ::std::fs;
use ::std::path::Path;

use crate::io::source::SourceFile;

pub fn read(pth: Path) -> MangoRes<SourceFile> {
    match fs::read_to_string(pth) {
        Ok(content) => SourceFile::new(content),
        Err(_) => SourceFile::new(content),
    }
}
