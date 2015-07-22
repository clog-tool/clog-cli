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


/// Convienience type for returning results of writing a changelog with a `Clog`
/// struct
///
/// # Example
///
/// ```no_run
/// # use clog::{Clog, Writer, Markdown};
/// # use std::io;
/// let clog = Clog::new().unwrap_or_else(|e| { 
///     e.exit();
/// });
/// let out = io::stdout();
/// let mut out_buf = io::BufWriter::new(out.lock());
/// let mut writer = Markdown::new(&mut out_buf, &clog);
/// clog.write_changelog_with(&mut writer).unwrap_or_else(|e| {
///     // Prints the error and exits appropriately
///     e.exit();
/// });
/// ```
pub type WriterResult = Result<(), Error>;

/// Specifies an arbitrary format to write the changelog data that will be used similar to the
/// following example
///
/// # Example
///
/// ```no_run
/// # use std::fs::File;
/// # use std::io::Read;
/// # use std::path::Path;
/// # use std::collections::BTreeMap;
/// # use clog::{Clog, Markdown, Writer, SectionMap};
/// let clog = Clog::new().unwrap_or_else(|e| { 
///     e.exit();
/// });
///
/// // Get the commits we're interested in...
/// let sm = SectionMap::from_commits(clog.get_commits());
///
/// // Open and prepend, or create the changelog file...
/// let mut contents = String::new();
/// if let Some(ref file) = clog.outfile {
///     File::open(file).map(|mut f| f.read_to_string(&mut contents).ok()).ok();
///     let mut file = File::create(file).ok().unwrap();
///
///     // Write the header...
///     let mut writer = Markdown::new(&mut file, &clog);
///     writer.write_header().ok().expect("failed to write header");
///
///     // Write the sections
///     for (sec, secmap) in sm.sections {
///         writer.write_section(&sec[..], &secmap.iter().collect::<BTreeMap<_,_>>()).ok().expect(&format!("failed to write {}", sec)[..]);
///     }
///
///     // Write old changelog data last
///     writer.write(&contents[..]).ok().expect("failed to write contents");
/// }
///
/// ```
pub trait FormatWriter {
    /// Writes a particular a changelog in a particular format
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::fs::File;
    /// # use std::io::Read;
    /// # use std::path::Path;
    /// # use std::collections::BTreeMap;
    /// # use clog::{Clog, Markdown, Writer, SectionMap};
    /// let clog = Clog::new().unwrap_or_else(|e| { 
    ///     e.exit();
    /// });
    ///
    /// // Get the commits we're interested in...
    /// let sm = SectionMap::from_commits(clog.get_commits());
    ///
    /// // Open and prepend, or create the changelog file...
    /// let mut contents = String::new();
    /// if let Some(ref file) = clog.outfile {
    ///     File::open(file).map(|mut f| f.read_to_string(&mut contents).ok()).ok();
    ///     let mut file = File::create(file).ok().unwrap();
    ///
    ///     // Write the header...
    ///     let mut writer = Markdown::new(&mut file, &clog);
    ///     writer.write_header().ok().expect("failed to write header");
    ///
    ///     // Write the sections
    ///     for (sec, secmap) in sm.sections {
    ///         writer.write_section(&sec[..], &secmap.iter().collect::<BTreeMap<_,_>>()).ok().expect(&format!("failed to write {}", sec)[..]);
    ///     }
    ///
    ///     // Write old changelog data last
    ///     writer.write(&contents[..]).ok().expect("failed to write contents");
    /// }
    ///
    /// ```
    fn write_changelog(&mut self, options: &Clog, section_map: &SectionMap) -> WriterResult;
}