use crate::io::source::SourceSlice;

pub type MangoResult<T> = Result<T, MangoErr>;

#[derive(Debug, Clone)]
pub enum MangoErr {
    Read { friendly: String, debug: Optional<String> },
    //TODO @mark: SourceSlice will have to refer to Rc SourceFile, because current borrow way, errors cannot get to a higher level than SourceFile
    //TODO @mark: I thought about making a borrowed and an Rc version, but apparently it's not worth it https://stackoverflow.com/q/31264670
    Syntax { friendly: String, debug: Optional<String>, src: SourceSlice },
}
