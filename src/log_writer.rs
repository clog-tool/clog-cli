use std::collections::BTreeMap;
use std::io::{Write, Result};

use time;

use logentry::LogEntry;
use clogconfig::ClogConfig;

pub struct LogWriter<'a, 'cc> {
    writer: &'a mut (Write + 'a),
    options: &'cc ClogConfig
}

impl<'a, 'cc> LogWriter<'a, 'cc> {
    fn commit_link(hash: &String, options: &ClogConfig) -> String {
        let short_hash = &hash[0..8];
        match &options.repo[..] {
            "" => format!("({})", short_hash),
            link => format!("[{}]({}/commit/{})", short_hash, link, hash)

        }
    }

    fn issue_link(&self, issue: &String) -> String {
        match &self.options.repo[..] {
            "" => format!("(#{})", issue),
            link => format!("[#{}]({}/issues/{})", issue, link, issue)
        }
    }

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

    pub fn write_section(&mut self, title: &str, section: &BTreeMap<&String, &Vec<LogEntry>>)
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
                                         LogWriter::commit_link(&entry.hash, &self.options)));

                if entry.closes.len() > 0 {
                    let closes_string = entry.closes.iter()
                                                    .map(|s| self.issue_link(s))
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


    pub fn write(&mut self, content: &str)  -> Result<()> {
        try!(write!(self.writer, "\n\n\n"));
        write!(self.writer, "{}", content)
    }

    pub fn new<T>(writer: &'a mut T, options: &'cc ClogConfig) -> LogWriter<'a, 'cc>
        where T: Write + Send {
        LogWriter {
            writer: writer,
            options: options
        }
    }
}
