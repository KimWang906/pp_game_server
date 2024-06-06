use std::fmt;

#[derive(Debug, Clone)]
pub enum CheckTokenError {
    VerifyTokenError,
    VaildationTokenError,
}

impl fmt::Display for CheckTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckTokenError::VerifyTokenError => write!(f, "Error while verifying the token"),
            CheckTokenError::VaildationTokenError => write!(f, "Error while validating the token"),
        }
    }
}