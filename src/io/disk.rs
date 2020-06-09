use ::std::fs;
use ::std::path::Path;

use crate::common::error::{MangoResult, MangoErr};
use crate::io::source::SourceFile;

pub fn read(pth: &Path) -> MangoResult<SourceFile> {
    match fs::read_to_string(pth) {
        Ok(content) => Ok(SourceFile::new(content)),
        Err(err) => Err(MangoErr::Read {
            friendly: format!("Could not read source in '{}'", pth.to_string_lossy()),
            debug: format!("{:?}", err),
        }),
    }
}
