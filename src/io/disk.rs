use ::std::fs;
use ::std::path::Path;

use crate::common::error::{MangoResult, MangoErr, ErrMsg};
use crate::io::source::SourceFile;

pub fn read(pth: &Path) -> MangoResult<SourceFile> {
    match fs::read_to_string(pth) {
        Ok(content) => Ok(SourceFile::new(pth.to_string_lossy(), content)),
        Err(err) => Err(MangoErr::Read { message: ErrMsg {
            friendly: format!("Could not read source in '{}'", pth.to_string_lossy()),
            debug: Some(format!("{:?}", err)),
        }}),
    }
}
