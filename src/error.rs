use std::error::Error as StdError;
use std::fmt;

/// An enum for describing and handling various errors encountered while dealing with `clog`
/// building, or writing of changelogs.
#[derive(Debug)]
pub enum Error {
    /// Generated while parsing config files such .clog.toml
    ConfigParseErr,
    /// Generated if the config file is not in TOML format
    ConfigFormatErr,
    /// Generated if the OS cannot determine the current directory
    CurrentDirErr,
    /// Generated when unable to read the TOML config file (perhaps due to permissions, etc.)
    TomlReadErr,
    /// Generated when an unrecognized link-style value is used
    LinkStyleErr,
    /// Generated when a string cannot be parsed to a SemVer value
    SemVerErr,
    /// Generated when there are errors creating the output file or stream
    CreateFileErr,
    /// Generated when there are errors writing to the changelog output file
    WriteErr,
    /// Generic catch all I/O related error
    IoErr,
    /// Unknown, but fatal error (a catch all)
    UnknownErr
}

// Shamelessly taken and adopted from https://github.com/BurntSushi :)
impl Error {
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ConfigParseErr  => "clog: error parsing config file",
            Error::ConfigFormatErr => "clog: incorrect format for config file",
            Error::CurrentDirErr   => "clog: cannot get current directory",
            Error::TomlReadErr     => "clog: cannot read TOML config file",
            Error::LinkStyleErr    => "clog: unrecognized link-style field",
            Error::SemVerErr       => "clog: cannot parse SemVer version",
            Error::CreateFileErr   => "clog: cannot create output file",
            Error::WriteErr        => "clog: cannot write to output file or stream",
            Error::UnknownErr      => "clog: unknown fatal error",
            Error::IoErr           => "clog: fatal i/o error with output file"
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            _ => None
        }
    }
}
