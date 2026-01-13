use std::fmt::{Display, Formatter};
use thiserror::Error;
use crate::base::error::DatabaseError::{BusinessError, NotFoundError};

#[derive(Error, Debug)]
pub enum DatabaseError{
    #[error("Business Error")]
    BusinessError(String),
    #[error("Connection Error")]
    ConnectionError(#[from] ConnectionError),
    #[error("Not Found Error : {0}")]
    NotFoundError(String),
    #[error("R2d2 Error")]
    R2d2Error(#[from] r2d2::Error),
    #[error("RusqliteError Error")]
    RusqliteError(#[from] rusqlite::Error),  
}

#[derive(Error, Debug)]
pub enum ConnectionError{

    #[error("Can't Get Connection Error")]
    CanNotGetConnectionError(#[source] r2d2::Error),

    #[error("Get Connection Timeout Error")]
    TimeoutError(#[source] r2d2::Error),
}