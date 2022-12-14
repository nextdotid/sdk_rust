#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP request error: {0}")]
    HttpError(#[from] hyper::Error),
    #[error("Hex parsing error: {0}")]
    HexError(#[from] hex::FromHexError),
    #[error("Secp256k1 error: {0}")]
    Secp256k1Error(#[from] libsecp256k1::Error),
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("Remote server error: {0}")]
    ServerError(String),
    #[error("Error when parsing body: {0}")]
    JSONParsingError(#[from] serde_json::Error),
    #[error("Error when parsing URL: {0}")]
    URLParsingError(#[from] url::ParseError),
    #[error("Error when parsing int from string: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

pub type Result<T> = core::result::Result<T, Error>;
