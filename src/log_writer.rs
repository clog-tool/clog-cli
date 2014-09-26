use std::collections::hashmap::HashMap;
use std::io::Writer;
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

    fn get_commit_link (repository: &String, hash: &String) -> String {
        let short_hash = hash.as_slice().slice_chars(0,8);
        if repository.len() > 0 {
            format!("[{}]({}/commit/{})", short_hash, repository, hash)
        }
        else {
            format!("({})", short_hash)
        }
    }

    fn get_issue_link (repository: &String, issue: &String) -> String {
        if repository.len() > 0 {
            format!("[#{}]({}/issues/{})", issue, repository, issue)
        }
        else {
            format!("(#{})", issue)
        }
    }

    pub fn write_header (&mut self) {

        let subtitle = match self.options.subtitle.len() {
            0 => self.options.subtitle.clone(),
            _ => format!(" {}", self.options.subtitle)
        };

        let version_text = format!("## {}{}", self.options.version, subtitle);

        fn get_date () -> String {
            ::time::now_utc().strftime("%Y-%m-%d")
        }

        if self.options.repository_link.len() > 0 {
            self.writer.write(format!("{} ({})\n\n", version_text, get_date()).as_bytes());
        }
        else {
            self.writer.write(format!("<a name=\"{}\"</a>\n{} ({})\n\n", self.options.version, version_text, get_date()).as_bytes());
        }
    }

    pub fn write_section (&mut self, title: &str, section: &HashMap<String, Vec<LogEntry>>) {

        if section.len() == 0 {
            return;
        }

        self.writer.write_line(format!("\n#### {}\n\n", title).as_slice());

        for (component, entries) in section.iter() {
            let mut prefix:String;
            let nested = entries.len() > 1;

            //TODO: implement the empty component stuff
            if nested {
                self.writer.write(format!("* **{}**\n", component).as_bytes());
                prefix = "  *".to_string();
            }
            else {
                prefix = format!("* **{}**", component)
            }

            for entry in entries.iter() {
                self.writer.write(format!("{} {} ({}", prefix, entry.subject, LogWriter::get_commit_link(&self.options.repository_link, &entry.hash)).as_bytes());
                if entry.closes.len() > 0 {

                    let closes_string = entry.closes.iter().fold("".to_string(), |a, b| {
                        match a.len() {
                            0 => format!("{}", LogWriter::get_issue_link(&self.options.repository_link, b)),
                            _ => format!("{}, {}", a, LogWriter::get_issue_link(&self.options.repository_link, b))
                        }
                    });
                    self.writer.write(format!(", closes {}", closes_string).as_bytes());
                }

                self.writer.write(")\n".as_bytes());
            };
        };
    }


    pub fn write (&mut self, content: &str) {
        self.writer.write("\n\n\n".as_bytes());
        self.writer.write(content.as_bytes());
    }

    pub fn new<T:Writer + Send>(writer: &'a mut T, options: LogWriterOptions) -> LogWriter<'a> {
        LogWriter {
            writer: writer,
            options: options
        }
    }
}
