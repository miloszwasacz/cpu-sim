use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegNameConvertError(pub(super) usize);

impl fmt::Display for RegNameConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{} is not a valid register", self.0)
    }
}

impl Error for RegNameConvertError {}
