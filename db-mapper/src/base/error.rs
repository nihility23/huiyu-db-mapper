use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::base::error::DatabaseError::{BusinessError, NotFoundError};

#[derive(Debug)]
pub enum DatabaseError{
    BusinessError(String),
    NotFoundError(String),
}

impl Error for DatabaseError {

}

impl Display for DatabaseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BusinessError(msg) => {
                write!(f, "{} : {}", "Business error",msg)
            }
            NotFoundError(msg) => {
                write!(f, "{} : {}", "Not found",msg)
            }
        }
    }
}