use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub(crate) struct NothingFoundError {
    message: String,
}

// Implement the std::fmt::Display trait for your error type
impl fmt::Display for NothingFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Implement the std::error::Error trait for your error type
pub(crate) impl error::Error for NothingFoundError {}