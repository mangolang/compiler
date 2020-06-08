
pub type MangoResult<T> = Result<T, MangoErr>;

#[derive(Debug, Clone)]
pub enum MangoErr {
    Read { friendly: String, debug: String },
}
