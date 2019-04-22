use std::fs::File;
use std::io;

/// Types which can be converted to valid, executable code.
pub trait ToCode {
    // todo: should this return binary?
    fn to_code(&self, file: &mut File) -> io::Result<()>;
}
