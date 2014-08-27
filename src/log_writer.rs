use std::collections::hashmap::HashMap;
use std::io::Writer;
use common::{ LogEntry };

pub struct LogWriter<'a> {
    writer: &'a mut Writer+'a,
    options: LogWriterOptions<'a>
}

pub struct LogWriterOptions<'a> {
    pub repository_link: String,
    //pub writer: &'a mut Writer+'a
}

impl<'a> LogWriter<'a> {

// function getCommitLink(repository, hash) {
//   var shortHash = hash.substring(0,8); // no need to show super long hash in log
//   return repository ?
//     util.format(LINK_COMMIT, shortHash, repository, hash) :
//     util.format(COMMIT, shortHash);
// }

    fn get_commit_link (repository: &String, hash: &String) -> String {
        let short_hash = hash.as_slice().slice_chars(0,8);
        format!("[{}]({}/commit/{})", short_hash, repository, hash)
    }

    pub fn write_section (&mut self, title: &str, section: &HashMap<String, Vec<LogEntry>>) {

        if section.len() == 0 {
            return;
        }

        self.writer.write_line(format!("\n#### {}\n\n", title).as_slice());

        section.iter().all(|(component, entries)| {
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

            entries.iter().all(|entry| {
                self.writer.write(format!("{} {} ({}", prefix, entry.subject, LogWriter::get_commit_link(&self.options.repository_link, &entry.hash)).as_bytes());
                //TODO: implement closes stuff

                self.writer.write(")\n".as_bytes());

                true
            });
            true
        });
    }

    pub fn new<T:Writer + Send>(writer: &'a mut T, options: LogWriterOptions) -> LogWriter<'a> {
        LogWriter {
            writer: writer,
            options: options
        }
    }
}