mod counts;


use fmt;
use language::Language;
pub use self::counts::Counts;

use std::fmt as StdFmt;
use std::ops::Deref;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Count {
    pub lang: Language,
    pub files: Vec<PathBuf>,
    pub code: u64,
    pub comments: u64,
    pub blanks: u64,
    pub lines: u64,
    pub usafe: u64,
    pub sep: Option<char>,
}

impl Count {
    pub fn new(lang: Language, sep: Option<char>) -> Self {
        Count {
            lang: lang,
            files: vec![],
            code: 0,
            comments: 0,
            blanks: 0,
            lines: 0,
            usafe: 0,
            sep: sep,
        }
    }

    pub fn add_file(&mut self, f: PathBuf) {
        self.files.push(f);
    }

    pub fn lines(&self) -> String {
        fmt::format_number(self.lines, self.sep)
    }

    pub fn code(&self) -> String {
        fmt::format_number(self.code, self.sep)
    }

    pub fn blanks(&self) -> String {
        fmt::format_number(self.blanks, self.sep)
    }

    pub fn usafe(&self) -> String {
        fmt::format_number(self.usafe, self.sep)
    }

    pub fn comments(&self) -> String {
        fmt::format_number(self.comments, self.sep)
    }

    pub fn total_files(&self) -> String {
        fmt::format_number(self.files.len() as u64, self.sep)
    }
}

impl Deref for Count {
    type Target = Language;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.lang
    }
}

impl StdFmt::Display for Count {
    fn fmt(&self, f: &mut StdFmt::Formatter) -> StdFmt::Result {
        write!(f,
               "{}\t{}\t{}\t{}\t{}\t{}",
               self.lang,
               self.total_files(),
               self.lines(),
               self.blanks(),
               self.comments(),
               self.code())
    }
}
