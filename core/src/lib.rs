use std::borrow::Cow;
use std::fmt;
use std::ffi::OsStr;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// A validator is any *function* taking &OsStr and returning Result<(), ValidationError>
pub type OptionValidator = fn(&OsStr) -> Result<(), ValidationError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError(pub Cow<'static, str>);

/// Simple validation error carrying a human-readable message.
impl ValidationError {
    pub fn new<S: Into<Cow<'static, str>>>(msg: S) -> Self {
        ValidationError(msg.into())
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ValidationError {}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
