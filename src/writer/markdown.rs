use std::collections::BTreeMap;
use std::io;

use time;

use clog::Clog;
use git::Commit;
use error::Error;
use writer::{Writer, WriterResult};


/// Writes commits to a specified `Write` object in Markdown format
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
pub struct Markdown<'a, 'cc> {
    /// The `Write` object
    writer: &'a mut (io::Write + 'a),
    /// The options used when writing sections and commits
    options: &'cc Clog
}

impl<'a, 'cc> Markdown<'a, 'cc> {
    /// Creates a new instance of the `Markdown` struct using a `Write` object and a `Clog` object
    /// as the configuration options to use while writing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::io::{stdout, BufWriter};
    /// # use clog::{Clog, Markdown};
    /// let clog = Clog::new().unwrap_or_else(|e| { 
    ///     e.exit();
    /// });
    ///
    /// // Create a Markdown writer to wrap stdout
    /// let out = stdout();
    /// let mut out_buf = BufWriter::new(out.lock());
    /// let mut writer = Markdown::new(&mut out_buf, &clog);
    /// ```
    pub fn new<T>(writer: &'a mut T, options: &'cc Clog) -> Markdown<'a, 'cc>
        where T: io::Write {
        Markdown {
            writer: writer,
            options: options
        }
    }

}

impl<'a, 'cc> Writer for Markdown<'a, 'cc> {
    /// Writes the initial header inforamtion for a release
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::fs::File;
    /// # use std::io::Read;
    /// # use std::path::Path;
    /// # use std::collections::BTreeMap;
    /// # use clog::{Clog, SectionMap, Writer, Markdown};
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
    ///     // Write the header...
    ///     let mut writer = Markdown::new(&mut file, &clog);
    ///     writer.write_header().ok().expect("failed to write header");
    /// }
    /// ```
    fn write_header(&mut self) -> io::Result<()> {
        let subtitle = match self.options.subtitle.len() {
            0 => self.options.subtitle.to_owned(),
            _ => format!(" {}", self.options.subtitle)
        };

        let version_text = if self.options.patch_ver {
            format!("### {}{}", self.options.version, subtitle)
        } else {
            format!("## {}{}", self.options.version, subtitle)
        };

        let date = time::now_utc();

        match date.strftime("%Y-%m-%d") {
            Ok(date) => {
                write!(
                    self.writer,
                    "<a name=\"{}\"></a>\n{} ({})\n\n",
                    self.options.version, version_text, date
                )
            },
            Err(_)   => {
                write!(
                    self.writer,
                    "<a name=\"{}\"></a>\n{} ({})\n\n",
                    self.options.version, version_text, "XXXX-XX-XX"
                )
            }
        }
    }

    /// Writes a particular section of a changelog 
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
    fn write_section(&mut self, title: &str, section: &BTreeMap<&String, &Vec<Commit>>)
                            -> WriterResult {
        if section.len() == 0 { return Ok(()) }

        if let Err(..) = self.writer.write(&format!("\n#### {}\n\n", title)[..].as_bytes()) {
            return Err(Error::WriteErr);
        }

        for (component, entries) in section.iter() {
            let nested = (entries.len() > 1) && !component.is_empty();

            let prefix = if nested {
                if let Err(..) = write!(self.writer, "* **{}**\n", component) {
                    return Err(Error::WriteErr);
                }
                "  *".to_owned()
            } else if !component.is_empty() {
                format!("* **{}**", component)
            } else {
                format!("* ")
            };

            for entry in entries.iter() {
                if let Err(..) = write!(
                                    self.writer, "{} {} ({}",
                                    prefix,
                                    entry.subject,
                                    self.options.link_style
                                        .commit_link(&*entry.hash, &self.options.repo[..])
                                ) {
                    return Err(Error::WriteErr);
                }

                if entry.closes.len() > 0 {
                    let closes_string = entry.closes.iter()
                                                    .map(|s| self.options.link_style.issue_link(&s[..], &self.options.repo[..]))
                                                    // FIXME: Connect should be
                                                    // used on the Iterator
                                                    .collect::<Vec<String>>()
                                                    .connect(", ");

                    if let Err(..) = write!(self.writer, ", closes {}", closes_string) {
                        return Err(Error::WriteErr);
                    }
                }

                if let Err(..) = write!(self.writer, ")\n") {
                    return Err(Error::WriteErr);
                }
            }
        }

        Ok(())
    }

    /// Writes some contents to the `Write` writer object
    fn write(&mut self, content: &str) -> io::Result<()> {
        try!(write!(self.writer, "\n\n\n"));
        write!(self.writer, "{}", content)
    }
}
