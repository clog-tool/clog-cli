use std::collections::hashmap::HashMap;
use std::io::Writer;
use time;
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

        let date = time::now_utc().strftime("%Y-%m-%d");

        if self.options.repository_link.len() > 0 {
            write!(self.writer, "{} ({})\n\n", version_text, date);
        } else {
            write!(self.writer, "<a name=\"{}\"</a>\n{} ({})\n\n",
                                self.options.version,
                                version_text,
                                date);
        }
    }

    pub fn write_section (&mut self, title: &str, section: &HashMap<String, Vec<LogEntry>>) {
        if section.len() == 0 { return; }

        let repo_link = &self.options.repository_link;
        // FIXME: Refactor these to non-static methods
        let issue_link = |s| -> String {
            LogWriter::get_issue_link(repo_link, s)
        };
        let commit_link = |s| -> String {
            LogWriter::get_commit_link(repo_link, s)
        };

        self.writer.write_line(format!("\n#### {}\n\n", title).as_slice());

        for (component, entries) in section.iter() {
            let nested = entries.len() > 1;

            //TODO: implement the empty component stuff
            let prefix = if nested {
                write!(self.writer, "* **{}**\n", component);
                "  *".to_string()
            } else {
                format!("* **{}**", component)
            };

            for entry in entries.iter() {
                write!(self.writer, "{} {} ({}",
                                    prefix,
                                    entry.subject,
                                    commit_link(&entry.hash));

                if entry.closes.len() > 0 {
                    let closes_string = entry.closes.iter()
                                                    .map(|s| issue_link(s))
                                                    // FIXME: Connect should be
                                                    // used on the Iterator
                                                    .collect::<Vec<String>>()
                                                    .connect(", ");

                    write!(self.writer, ", closes {}", closes_string);
                }

                write!(self.writer, ")\n");
            };
        };
    }


    pub fn write (&mut self, content: &str) {
        write!(self.writer, "\n\n\n");
        write!(self.writer, "{}", content);
    }

    pub fn new<T:Writer + Send>(writer: &'a mut T, options: LogWriterOptions) -> LogWriter<'a> {
        LogWriter {
            writer: writer,
            options: options
        }
    }
}
