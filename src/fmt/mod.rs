pub use self::md_writer::MarkdownWriter;
pub use self::json_writer::JsonWriter;

mod md_writer;
mod json_writer;

use clog::Clog;
use error::Error;
use sectionmap::SectionMap;

arg_enum! {
    #[derive(Debug)]
    pub enum ChangelogFormat {
        Json,
        Markdown
    }
}


/// Convienience type for returning results of writing a changelog with a `Clog` struct
///
/// # Example
///
/// ```no_run
/// # use clog::Clog;
/// # use clog::fmt::{FormatWriter, MarkdownWriter};
/// # use std::io;
/// let clog = Clog::new().unwrap_or_else(|e| { 
///     e.exit();
/// });
///
/// // Create a MarkdownWriter that wraps stdout
/// let out = io::stdout();
/// let mut out_buf = io::BufWriter::new(out.lock());
/// let mut writer = MarkdownWriter::new(&mut out_buf);
/// 
/// clog.write_changelog_with(&mut writer).unwrap_or_else(|e| {
///     // Prints the WriterResult error and exits appropriately
///     e.exit();
/// });
/// ```
pub type WriterResult = Result<(), Error>;

/// A trait that allows writing the results of a `clog` run which can then be written in an
/// arbitrary format. The single required function `write_changelog()` accepts a `clog::SectionMap`
/// which can be thought of similiar to a `clog` "AST" of sorts.
///
/// `clog` provides two default implementors of this traint, `clog::fmt::MarkdownWriter` and 
/// `clog::fmt::JsonWriter` for writing Markdown and JSON respectively
pub trait FormatWriter {
    /// Writes a changelog from a given `clog::SectionMap` which can be thought of as an "AST" of 
    /// sorts
    fn write_changelog(&mut self, options: &Clog, section_map: &SectionMap) -> WriterResult;
}