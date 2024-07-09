use std::{
    convert::From,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use clog::error::Error as ClogErr;

use crate::fmt::Format;

#[derive(Debug)]
#[allow(dead_code)]
pub enum CliError {
    Semver(Box<dyn Error>, String),
    Generic(String),
    Unknown,
}

// Copies clog::error::Error;
impl CliError {
    /// Return whether this was a fatal error or not.
    pub fn is_fatal(&self) -> bool {
        // For now all errors are fatal
        true
    }

    /// Print this error and immediately exit the program.
    ///
    /// If the error is non-fatal then the error is printed to stdout and the
    /// exit status will be `0`. Otherwise, when the error is fatal, the error
    /// is printed to stderr and the exit status will be `1`.
    pub fn exit(&self) -> ! {
        if self.is_fatal() {
            wlnerr!("{}", self);
            ::std::process::exit(1)
        } else {
            println!("{}", self);
            ::std::process::exit(0)
        }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{} {}",
            Format::Error("error:"),
            match self {
                CliError::Generic(d) => d,
                CliError::Unknown => {
                    "An unknown fatal error has occurred, please consider filing a bug-report!"
                }
                CliError::Semver(_, s) => s,
            }
        )
    }
}

impl Error for CliError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            CliError::Semver(e, _) => Some(&**e),
            CliError::Generic(..) => None,
            CliError::Unknown => None,
        }
    }
}

impl From<ClogErr> for CliError {
    fn from(ce: ClogErr) -> Self { CliError::Generic(ce.to_string().to_owned()) }
}
