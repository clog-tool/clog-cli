use std::collections::hashmap::HashMap;
use std::io::{Writer, IoResult};
use time;
use format_util;
use common::{ LogEntry };

pub struct LogWriter<'a> {
    writer: &'a mut Writer+'a,
    options: LogWriterOptions<'a>
}

pub struct LogWriterOptions<'a> {
    pub repository_link: String,
    pub version: String,
    pub subtitle: String
}

impl<'a> LogWriter<'a> {

    fn commit_link(&self, hash: &String) -> String {
        let short_hash = format_util::get_short_hash(hash.as_slice());
        match self.options.repository_link.as_slice() {
            "" => format!("({})", short_hash),
            link => format!("[{}]({}/commit/{})", short_hash, link, hash)

        }
    }

    fn issue_link(&self, issue: &String) -> String {
        match self.options.repository_link.as_slice() {
            "" => format!("(#{})", issue),
            link => format!("[#{}]({}/issues/{})", issue, link, issue)
        }
    }

    pub fn write_header(&mut self) -> IoResult<()> {
        let subtitle = match self.options.subtitle.len() {
            0 => self.options.subtitle.clone(),
            _ => format!(" {}", self.options.subtitle)
        };

        let version_text = format!("## {}{}", self.options.version, subtitle);

        let date = time::now_utc().strftime("%Y-%m-%d");

        write!(self.writer, "<a name=\"{}\"></a>\n{} ({})\n\n", self.options.version, version_text, date)
    }

    pub fn write_section(&mut self, title: &str, section: &HashMap<String, Vec<LogEntry>>)
                            -> IoResult<()> {
        if section.len() == 0 { return Ok(()) }

        try!(self.writer.write_line(format!("\n#### {}\n\n", title).as_slice()));

        for (component, entries) in section.iter() {
            let nested = entries.len() > 1;

            //TODO: implement the empty component stuff
            let prefix = if nested {
                try!(write!(self.writer, "* **{}**\n", component));
                "  *".to_string()
            } else {
                format!("* **{}**", component)
            };

            for entry in entries.iter() {
                try!(write!(self.writer, "{} {} ({}",
                                         prefix,
                                         entry.subject,
                                         self.commit_link(&entry.hash)));

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


    pub fn write(&mut self, content: &str)  -> IoResult<()> {
        try!(write!(self.writer, "\n\n\n"));
        write!(self.writer, "{}", content)
    }

    pub fn new<T:Writer + Send>(writer: &'a mut T, options: LogWriterOptions) -> LogWriter<'a> {
        LogWriter {
            writer: writer,
            options: options
        }
    }
}
