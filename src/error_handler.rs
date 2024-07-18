use thiserror::Error;
use tonic::{Code, Status};

#[derive(Error, Debug)]
pub enum LandeedError {
    #[error(transparent)]
    DBError(#[from] r2d2::Error),

    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),

    #[error(transparent)]
    HexError(#[from] hex::FromHexError),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error(transparent)]
    MacError(#[from] digest::MacError),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error(transparent)]
    UuidError(#[from] uuid::Error),
}

impl From<LandeedError> for tonic::Status {
    fn from(err: LandeedError) -> Status {
        match err {
            LandeedError::DieselError(diesel::result::Error::NotFound) => {
                Status::new(Code::NotFound, "Not found")
            }
            _ => Status::new(Code::Internal, err.to_string()),
        }
    }
}
