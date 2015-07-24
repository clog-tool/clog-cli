use std::collections::BTreeMap;
use std::io;

use time;

use clog::Clog;
use git::Commit;
use error::Error;
use fmt::{FormatWriter, WriterResult};
use sectionmap::SectionMap;


/// Writes commits to a specified `Write` object in Markdown format
///
/// # Example
///
/// ```no_run
/// # use std::fs::File;
/// # use std::io::Read;
/// # use std::path::Path;
/// # use std::collections::BTreeMap;
/// # use clog::{Clog, JsonWriter, FormatWriter, SectionMap};
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
///     let mut writer = JsonWriter::new(&mut file);
///     clog.write_changelog_with(&mut writer).unwrap_or_else(|e| { 
///         e.exit();
///     });
/// }
///
/// ```
pub struct JsonWriter<'a>(&'a mut io::Write);


impl<'a> JsonWriter<'a> {
    /// Creates a new instance of the `Markdown` struct using a `Write` object and a `Clog` object
    /// as the configuration options to use while writing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::io::{stdout, BufWriter};
    /// # use clog::{Clog, MarkdownWriter};
    /// let clog = Clog::new().unwrap_or_else(|e| { 
    ///     e.exit();
    /// });
    ///
    /// // Create a Markdown writer to wrap stdout
    /// let out = stdout();
    /// let mut out_buf = BufWriter::new(out.lock());
    /// let mut writer = MarkdownWriter::new(&mut out_buf);
    /// ```
    pub fn new<T: io::Write>(writer: &'a mut T) -> JsonWriter<'a> {
        JsonWriter(writer)
    }
}

impl<'a> JsonWriter<'a> {
    /// Writes the initial header inforamtion for a release
    fn write_header(&mut self, options: &Clog) -> io::Result<()> {
        write!(self.0, "\"header\":{{\"version\":{:?},\"patch_version\":{:?},\"subtitle\":{},",
            options.version,
            options.patch_ver,
            match options.subtitle.len() {
                0 => "null".to_owned(),
                _ => format!("{:?}", &*options.subtitle)
            }
        ).unwrap();

        let date = time::now_utc();

        match date.strftime("%Y-%m-%d") {
            Ok(date) => {
                write!(
                    self.0,
                    "\"date\":\"{}\"}},",
                    date
                )
            },
            Err(_)   => {
                write!(
                    self.0,
                    "\"date\":null}},",
                )
            }
        }
    }

    /// Writes a particular section of a changelog 
    fn write_section(&mut self, options: &Clog, section: &BTreeMap<&String, &Vec<Commit>>)
                            -> WriterResult {
        if section.len() == 0 { 
            write!(self.0, "\"commits\":null").unwrap();
            return Ok(()) 
        }

        write!(self.0, "\"commits\":[").unwrap();
        for (component, entries) in section.iter() {
            let mut e_it = entries.iter().peekable();
            while let Some(entry) = e_it.next() {
                write!(self.0, "{{\"component\":").unwrap();
                if component.is_empty() {
                    write!(self.0, "null,").unwrap();
                } else {
                    write!(self.0, "{:?},", component).unwrap();
                }
                write!(
                    self.0 , "\"subject\":{:?},\"commit_link\":{:?},\"closes\":",
                    entry.subject,
                    options.link_style
                           .commit_link(&*entry.hash, &*options.repo)
                ).unwrap(); 

                if entry.closes.len() > 0 {
                    write!(self.0, "[").unwrap();
                    let mut c_it = entry.closes.iter().peekable();
                    while let Some(issue) = c_it.next() {
                        write!(self.0,
                            "{{\"issue\":{},\"issue_link\":{:?}}}",
                            issue,
                            options.link_style.issue_link(issue, &options.repo)
                        ).unwrap();
                        if c_it.peek().is_some() {
                            write!(self.0, ",").unwrap();
                        }
                    }
                    write!(self.0,
                        "]}}").unwrap();
                } else {
                    write!(self.0, "null}}").unwrap();
                }
                if e_it.peek().is_some() {
                    write!(self.0, ",").unwrap();
                }
            }
        }
        write!(self.0, "]").unwrap();
        Ok(())
    }

    /// Writes some contents to the `Write` writer object
    #[allow(dead_code)]
    fn write(&mut self, content: &str) -> io::Result<()> {
        write!(self.0, "{}", content)
    }
}

impl<'a> FormatWriter for JsonWriter<'a> {
    fn write_changelog(&mut self, options: &Clog, sm: &SectionMap) -> WriterResult {
        debugln!("Writing JSON changelog");
        write!(self.0, "{{").unwrap();
        if let Some(..) = self.write_header(options).err() {
        debugln!("Error writing JSON header");
            return Err(Error::WriteErr);
        }

        write!(self.0, "\"sections\":").unwrap();
        let mut s_it = sm.sections.iter().peekable();
        if s_it.peek().is_some() {
        debugln!("There are sections to write");
            write!(self.0, "[").unwrap();
            while let Some((sec, compmap)) = s_it.next() {
                debugln!("Writing section: {}", &*sec);
                write!(self.0, "{{\"title\":{:?},", &*sec).unwrap();

                try!(self.write_section(options, &compmap.iter().collect::<BTreeMap<_,_>>()));

                write!(self.0, "}}").unwrap();
                if s_it.peek().is_some() && s_it.peek().unwrap().0.len() > 0 {
                    debugln!("There are more sections, adding comma");
                    write!(self.0, ",").unwrap();
                } else {
                    debugln!("There are no more sections, no comma required");
                }
            }
            write!(self.0, "]").unwrap();
        } else {
            debugln!("There are no sections to write");
            write!(self.0, "null").unwrap();
        }

        write!(self.0, "}}").unwrap();
        debugln!("Finished writing sections, flushing");
        self.0.flush().unwrap();
        Ok(())
    }
}
