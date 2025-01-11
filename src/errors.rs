#[derive(Debug, thiserror::Error)]
pub enum XtaError {
    #[error("XtaErr: Couldn't read file - {0}")]
    FileError(String),

    #[error("XtaErr: Unexpected character `{0}` at line {1}")]
    ScannerError(char, usize),

    #[error("XtaErr: Invalid number format - {0}")]
    InvalidNumberFormat(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {}
