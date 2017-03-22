mod counter;

use fmt;
use language::Language;

use std::fmt as StdFmt;
use std::ops::Deref;
use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct Count {
    pub lang: Language,
    pub code: u64,
    pub comments: u64,
    pub blanks: u64,
    pub lines: u64,
    pub usafe: u64,
    pub files: u64,
}

impl Count {
    pub fn new(lang: Language) -> Self {
        Count {
            lang: lang,
            .. Count::default()
        }
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
        fmt::format_number(self.files, self.sep)
    }
}

// impl Deref for Count {
//     type Target = Language;
//     fn deref(&self) -> &<Self as Deref>::Target {
//         &self.lang
//     }
// }

// impl StdFmt::Display for Count {
//     fn fmt(&self, f: &mut StdFmt::Formatter) -> StdFmt::Result {
//         write!(f,
//                "{}\t{}\t{}\t{}\t{}\t{}",
//                self.lang,
//                self.total_files(),
//                self.lines(),
//                self.blanks(),
//                self.comments(),
//                self.code())
//     }
// }
