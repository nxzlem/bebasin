use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    Error(Box<dyn std::error::Error>),
    IOError(std::io::Error),
    PestRuleError(pest::error::Error<crate::parser::Rule>),
    SerdeJSONError(serde_json::Error),
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    NixError(nix::Error),
    ZipError(zip::result::ZipError),
    String(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_msg = match self {
            ErrorKind::Error(err) => err.to_string(),
            ErrorKind::IOError(err) => err.to_string(),
            ErrorKind::PestRuleError(err) => err.to_string(),
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            ErrorKind::NixError(err) => err.to_string(),
            ErrorKind::SerdeJSONError(err) => err.to_string(),
            ErrorKind::ZipError(err) => err.to_string(),
            ErrorKind::String(err) => err.to_owned(),
        };

        write!(f, "{}", error_msg)
    }
}
