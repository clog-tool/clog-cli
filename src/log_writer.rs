use std::collections::HashMap;
use std::io::{Write, Result};
use time;
use format_util;
use common::{ LogEntry };
use std::borrow::ToOwned;

pub struct LogWriter<'a, 'lwo> {
    writer: &'a mut (Write + 'a),
    options: LogWriterOptions<'lwo>
}

pub struct LogWriterOptions<'a> {
    pub repository_link: &'a str,
    pub version: String,
    pub subtitle: String 
}

impl<'a, 'lwo> LogWriter<'a, 'lwo> {

    fn commit_link(hash: &String, options: &LogWriterOptions) -> String {
        let short_hash = format_util::get_short_hash(&hash[..]);
        match &options.repository_link[..] {
            "" => format!("({})", short_hash),
            link => format!("[{}]({}/commit/{})", short_hash, link, hash)

        }
    }

    fn issue_link(&self, issue: &String) -> String {
        match &self.options.repository_link[..] {
            "" => format!("(#{})", issue),
            link => format!("[#{}]({}/issues/{})", issue, link, issue)
        }
    }

    pub fn write_header(&mut self) -> Result<()> {
        let subtitle = match self.options.subtitle.len() {
            0 => self.options.subtitle.to_owned(),
            _ => format!(" {}", self.options.subtitle)
        };

        let version_text = format!("## {}{}", self.options.version, subtitle);

        let date = time::now_utc();

        match date.strftime("%Y-%m-%d") {
            Ok(date) => write!(self.writer, "<a name=\"{}\"></a>\n{} ({})\n\n", self.options.version, version_text, date),
            Err(_)   => write!(self.writer, "<a name=\"{}\"></a>\n{} ({})\n\n", self.options.version, version_text, "XXXX-XX-XX")
        }
    }

    pub fn write_section(&mut self, title: &str, section: &HashMap<String, Vec<LogEntry>>)
                            -> Result<()> {
        if section.len() == 0 { return Ok(()) }

        try!(self.writer.write(&format!("\n#### {}\n\n", title)[..].as_bytes()));

        for (component, entries) in section.iter() {
            let nested = entries.len() > 1;

            //TODO: implement the empty component stuff
            let prefix = if nested {
                try!(write!(self.writer, "* **{}**\n", component));
                "  *".to_owned()
            } else {
                format!("* **{}**", component)
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

    pub fn new<T>(writer: &'a mut T, options: LogWriterOptions<'lwo>) -> LogWriter<'a, 'lwo>
        where T: Write + Send {
        LogWriter {
            writer: writer,
            options: options
        }
    }
}
