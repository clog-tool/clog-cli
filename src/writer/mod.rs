use std::collections::BTreeMap;
use std::io;

pub use self::markdown::Markdown;

mod markdown;

use error::Error;
use git::Commit;


/// Convienience type for returning results of writing a changelog with a `Clog`
/// struct
///
/// # Example
///
/// ```no_run
/// # use clog::Clog;
/// # use std::io;
/// let out = io::stdout();
/// let clog = Clog::new();
/// clog.write_changelog_with(out.lock()).unwrap_or_else(|e| {
///     // Prints the error and exits appropriately
///     e.exit();
/// });
/// ```
pub type WriterResult = Result<(), Error>;

pub trait Writer {
    fn write_header(&mut self) -> io::Result<()>;
    fn write_section(&mut self, title: &str, section: &BTreeMap<&String, &Vec<Commit>>) -> WriterResult;
    fn write(&mut self, content: &str) -> io::Result<()>;
}