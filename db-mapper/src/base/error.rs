use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Business Error")]
    CommonError(String),
    #[error("Not Found Error : {0}")]
    NotFoundError(String),
    #[error("Unknow Error : {0}")]
    UnKnowError(String),
    #[error("Convert Error : {0}")]
    ConvertError(String),

    // 三方库异常
    #[error("R2d2 Error：{0}")]
    R2d2Error(#[from] r2d2::Error),
    #[error("RusqliteError Error: {0}")]
    RusqliteError(#[from] rusqlite::Error),
    #[error("FromUtf8Error Error: {0}")]
    StringConvertError(#[from] FromUtf8Error),
}
