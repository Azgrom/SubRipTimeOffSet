use std::fmt::{Display, Formatter, Result as FMTResult};

mod subrip;

pub use subrip::SubRipFile;
pub use sqlite_diesel::crud as sql;


impl SubRipFile {
    fn content_string_parser(&self) -> String {
        let mut content_string = String::new();

        for content in self.contents.iter().enumerate() {
            content_string.push_str(&(content.0 + 1).to_string());
            content_string.push('\n');
            content_string.push_str(&content.1.to_string());
        }

        content_string
    }

    pub fn register_subtitle(&self) {
        let content_string = self.content_string_parser();

        sql::write_reg(&self.filename, content_string);
        sql::publish_reg();
    }

    pub fn show_subtitle() {
        sql::show_regs();
    }

    pub fn delete_registry(filename: String) {
        sql::delete_reg(filename);
    }
}

mod stdout_implementation {
    use super::*;

    impl Display for SubRipFile {
        fn fmt(&self, f: &mut Formatter) -> FMTResult {
            let content_string = self.content_string_parser();

            write!(
                f,
                "Filename: {}\nNumber of dialogs: {}\n\n\nDialogs:\n----------\n{}",
                self.filename,
                self.contents.len(),
                content_string
            )
        }
    }
}
