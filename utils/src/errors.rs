use thiserror::Error;

pub use anyhow::Result;

#[derive(Error, Debug)]
pub enum EigenError {
    #[error("invalid range proof, `{0}`")]
    InvalidValue(String),

    #[error("invalid range (expected {expected:?}, found {found:?})")]
    OutOfRangeError { expected: String, found: String },

    #[error("open file error")]
    FileError(#[from] std::io::Error),

    // #[error("json serialization error")]
    // SerdeError(#[from] serde_json::Error),
    #[error("poseidon hash error`{0}`")]
    PoseidonHashError(String),

    #[error("merkle tree error`{0}`")]
    MerkleTreeError(String),

    #[error("degree should be equal, but `{0}` != `{1}`")]
    MustEqualDegreeError(usize, usize),

    #[error("expression error, msg `{0}`")]
    ExpressionError(String),

    #[error("invalid op, msg `{0}`")]
    InvalidOperator(String),

    #[error("verify FRI proof failed")]
    FriVerifierFailed,

    #[error("database failure")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Unknown error, `{0}`")]
    Unknown(String),
}

impl From<String> for EigenError {
    fn from(e: String) -> Self {
        EigenError::Unknown(e)
    }
}
