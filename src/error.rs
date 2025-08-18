#[derive(Debug, thiserror::Error)]
pub enum MathlineError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error("invalid char: {0}")]
    InvalidChar(char),
}
