use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug, Serialize)]
pub enum DatabaseError {
    #[error("Business Error")]
    CommonError(String),
    #[error("Not Found Error : {0}")]
    NotFoundError(String),
    #[error("Unknow Error : {0}")]
    UnKnowError(String),
    #[error("Convert Error : {0}")]
    ConvertError(String),
    #[error("Access Error: {0}")]
    AccessError(String),
    #[error("Instance Already Exists: {0}")]
    InstanceAlreadyExistsError(String),
    #[error("Config Not Found: {0}")]
    ConfigNotFoundError(String),
    #[error("Pool Create Error: {0}")]
    PoolCreateError(String),
    #[error("Connect Can Not Get Error: {0}")]
    ConnectCanNotGetError(String),
    #[error("Row Convert Error: {0}")]
    RowConvertError(String),
    #[error("Not Supported Error: {0}")]
    NotSupportedError(String),
    #[error("Execute Error: {0}")]
    ExecuteError(String),
}

unsafe impl Sync for DatabaseError {}
