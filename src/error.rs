use std::convert::From;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

use clog::error::Error as ClogErr;

use fmt::Format;

#[derive(Debug)]
pub enum CliError {
    Semver(Box<Error>, String),
    Generic(String),
    Unknown
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
        write!(f, "{} {}", Format::Error("error:"), self.description())
    }
}

impl Error for CliError {
    fn description<'a>(&'a self) -> &'a str {
        match *self {
            CliError::Semver(_, ref s) => &*s,
            CliError::Generic(ref d)   => &*d,
            CliError::Unknown      => "An unknown fatal error has occurred, please consider filing a bug-report!"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CliError::Semver(ref e, _) => Some(&**e),
            CliError::Generic(..)  => None,
            CliError::Unknown      => None
        }
    }
}

impl From<ClogErr> for CliError {
    fn from(ce: ClogErr) -> Self {
        CliError::Generic(ce.description().to_owned())
    }
}