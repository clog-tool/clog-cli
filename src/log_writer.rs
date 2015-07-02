use std::collections::BTreeMap;
use std::io::{Write, Result};

use time;

use git::Commit;
use clog::Clog;

/// Writes commits to a specified `Write` object
pub struct LogWriter<'a, 'cc> {
    /// The `Write` object
    writer: &'a mut (Write + 'a),
    /// The options used when writing sections and commits
    options: &'cc Clog
}

impl<'a, 'cc> LogWriter<'a, 'cc> {
    /// Creates a new instance of the LogWriter using a `Write` object and a `Clog` object as the
    /// configuration options to use while writing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::fs::File;
    /// # use std::io::Read;
    /// # use std::path::Path;
    /// # use std::collections::BTreeMap;
    /// # use clog::{Clog, LogWriter};
    /// let clog = Clog::new().unwrap_or_else(|e| { 
    ///     println!("Error initializing: {}", e);
    ///     std::process::exit(1);
    /// });
    ///
    /// // Open and prepend, or create the changelog file...
    /// let mut contents = String::new();
    /// File::open(&Path::new(&clog.changelog[..])).map(|mut f| f.read_to_string(&mut contents).ok()).ok();
    /// let mut file = File::create(&Path::new(&clog.changelog[..])).ok().unwrap();
    ///
    /// // Create the LogWriter... 
    /// let mut writer = LogWriter::new(&mut file, &clog);
    /// ```
    pub fn new<T>(writer: &'a mut T, options: &'cc Clog) -> LogWriter<'a, 'cc>
        where T: Write + Send {
        LogWriter {
            writer: writer,
            options: options
        }
    }

    /// Writes the initial header inforamtion for a release
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::fs::File;
    /// # use std::io::Read;
    /// # use std::path::Path;
    /// # use std::collections::BTreeMap;
    /// # use clog::{Clog, LogWriter, SectionMap};
    /// let clog = Clog::new().unwrap_or_else(|e| { 
    ///     println!("Error initializing: {}", e);
    ///     std::process::exit(1);
    /// });
    ///
    /// // Get the commits we're interested in...
    /// let sm = SectionMap::from_commits(clog.get_commits());
    ///
    /// // Open and prepend, or create the changelog file...
    /// let mut contents = String::new();
    /// File::open(&Path::new(&clog.changelog[..])).map(|mut f| f.read_to_string(&mut contents).ok()).ok();
    /// let mut file = File::create(&Path::new(&clog.changelog[..])).ok().unwrap();
    ///
    /// // Write the header...
    /// let mut writer = LogWriter::new(&mut file, &clog);
    /// writer.write_header().ok().expect("failed to write header");
    /// ```
    pub fn write_header(&mut self) -> Result<()> {
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
            Ok(date) => write!(self.writer, "<a name=\"{}\"></a>\n{} ({})\n\n", self.options.version, version_text, date),
            Err(_)   => write!(self.writer, "<a name=\"{}\"></a>\n{} ({})\n\n", self.options.version, version_text, "XXXX-XX-XX")
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
    /// # use clog::{Clog, LogWriter, SectionMap};
    /// let clog = Clog::new().unwrap_or_else(|e| { 
    ///     println!("Error initializing: {}", e);
    ///     std::process::exit(1);
    /// });
    ///
    /// // Get the commits we're interested in...
    /// let sm = SectionMap::from_commits(clog.get_commits());
    ///
    /// // Open and prepend, or create the changelog file...
    /// let mut contents = String::new();
    /// File::open(&Path::new(&clog.changelog[..])).map(|mut f| f.read_to_string(&mut contents).ok()).ok();
    /// let mut file = File::create(&Path::new(&clog.changelog[..])).ok().unwrap();
    ///
    /// // Write the header...
    /// let mut writer = LogWriter::new(&mut file, &clog);
    /// writer.write_header().ok().expect("failed to write header");
    ///
    /// // Write the sections
    /// for (sec, secmap) in sm.sections {
    ///    writer.write_section(&sec[..], &secmap.iter().collect::<BTreeMap<_,_>>()).ok().expect(&format!("failed to write {}", sec)[..]);
    /// }
    /// writer.write(&contents[..]).ok().expect("failed to write contents");
    /// ```
    pub fn write_section(&mut self, title: &str, section: &BTreeMap<&String, &Vec<Commit>>)
                            -> Result<()> {
        if section.len() == 0 { return Ok(()) }

        try!(self.writer.write(&format!("\n#### {}\n\n", title)[..].as_bytes()));

        for (component, entries) in section.iter() {
            let nested = (entries.len() > 1) && !component.is_empty();

            let prefix = if nested {
                try!(write!(self.writer, "* **{}**\n", component));
                "  *".to_owned()
            } else if !component.is_empty() {
                format!("* **{}**", component)
            } else {
                format!("* ")
            };

            for entry in entries.iter() {
                try!(write!(self.writer, "{} {} ({}",
                                         prefix,
                                         entry.subject,
                                         self.options.link_style
                                             .commit_link(&entry.hash[..], &self.options.repo[..])));

                if entry.closes.len() > 0 {
                    let closes_string = entry.closes.iter()
                                                    .map(|s| self.options.link_style.issue_link(&s[..], &self.options.repo[..]))
                                                    // FIXME: Connect should be
                                                    // used on the Iterator
                                                    .collect::<Vec<String>>()
                                                    .connect(", ");

                    try!(write!(self.writer, ", closes {}", closes_string));
                }

                try!(write!(self.writer, ")\n"));
            }
        }

        Ok(())
    }

    /// Writes some contents to the `Write` writer object
    pub fn write(&mut self, content: &str)  -> Result<()> {
        try!(write!(self.writer, "\n\n\n"));
        write!(self.writer, "{}", content)
    }
}
