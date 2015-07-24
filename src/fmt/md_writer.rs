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
/// # use clog::{Clog, MarkdownWriter, FormatWriter, SectionMap};
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
///     let mut writer = MarkdownWriter::new(&mut file);
///     writer.write_changelog(&clog, &sm).ok().expect("failed to write header");
/// }
///
/// ```
pub struct MarkdownWriter<'a>(&'a mut io::Write);


impl<'a> MarkdownWriter<'a> {
    /// Creates a new instance of the `Markdown` struct using a `Write` object and a `Clog` object
    /// as the configuration options to use while writing.
    pub fn new<T: io::Write + 'a>(writer: &'a mut T) -> MarkdownWriter<'a> {
        MarkdownWriter(writer)
    }

    fn write_header(&mut self, options: &Clog) -> io::Result<()> {
        let subtitle = match options.subtitle.len() {
            0 => options.subtitle.to_owned(),
            _ => format!(" {}", options.subtitle)
        };

        let version_text = if options.patch_ver {
            format!("### {}{}", options.version, subtitle)
        } else {
            format!("## {}{}", options.version, subtitle)
        };

        let date = time::now_utc();

        match date.strftime("%Y-%m-%d") {
            Ok(date) => {
                write!(
                    self.0,
                    "<a name=\"{}\"></a>\n{} ({})\n\n",
                    options.version, version_text, date
                )
            },
            Err(_)   => {
                write!(
                    self.0,
                    "<a name=\"{}\"></a>\n{} ({})\n\n",
                    options.version, version_text, "XXXX-XX-XX"
                )
            }
        }
    }

    /// Writes a particular section of a changelog 
    fn write_section(&mut self, options: &Clog, title: &str, section: &BTreeMap<&String, &Vec<Commit>>)
                            -> WriterResult {
        if section.len() == 0 { return Ok(()) }

        if let Err(..) = self.0.write(&format!("\n#### {}\n\n", title)[..].as_bytes()) {
            return Err(Error::WriteErr);
        }

        for (component, entries) in section.iter() {
            let nested = (entries.len() > 1) && !component.is_empty();

            let prefix = if nested {
                if let Err(..) = write!(self.0 , "* **{}**\n", component) {
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
                                    self.0 , "{} {} ([{}]({})",
                                    prefix,
                                    entry.subject,
                                    &entry.hash[0..8],
                                    options.link_style
                                           .commit_link(&*entry.hash, &options.repo[..])
                                ) {
                    return Err(Error::WriteErr);
                }

                if entry.closes.len() > 0 {
                    let closes_string = entry.closes.iter()
                                                    .map(|s| format!("[#{}]({})",
                                                        &*s,
                                                        options.link_style.issue_link(&*s, &options.repo)))
                                                    // FIXME: Connect should be
                                                    // used on the Iterator
                                                    .collect::<Vec<String>>()
                                                    .connect(", ");

                    if let Err(..) = write!(self.0 , ", closes {}", closes_string) {
                        return Err(Error::WriteErr);
                    }
                }

                if let Err(..) = write!(self.0 , ")\n") {
                    return Err(Error::WriteErr);
                }
            }
        }

        Ok(())
    }

    /// Writes some contents to the `Write` writer object
    #[allow(dead_code)]
    fn write(&mut self, content: &str) -> io::Result<()> {
        try!(write!(self.0 , "\n\n\n"));
        write!(self.0 , "{}", content)
    }
}

impl<'a> FormatWriter for MarkdownWriter<'a> {
    fn write_changelog(&mut self, options: &Clog, sm: &SectionMap) -> WriterResult {
        if let Some(..) = self.write_header(options).err() {
            return Err(Error::WriteErr);
        }
        for (sec, secmap) in sm.sections.iter() {
            try!(self.write_section(options, &sec[..], &secmap.iter().collect::<BTreeMap<_,_>>()));
        }

        self.0.flush().unwrap();

        Ok(())
    }
}
