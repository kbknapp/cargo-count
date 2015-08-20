use std::fmt;

#[cfg(all(feature = "color", not(target_os = "windows")))]
use ansi_term::Colour::{Red, Green, Yellow};
#[cfg(all(feature = "color", not(target_os = "windows")))]
use ansi_term::ANSIString;

#[allow(dead_code)]
pub enum Format<T> {
     Error(T),
     Warning(T),
     Good(T),
}

#[cfg(all(feature = "color", not(target_os = "windows")))]
impl<T: AsRef<str>> Format<T> {
    fn format(&self) -> ANSIString {
        match *self {
            Format::Error(ref e) => Red.bold().paint(e.as_ref()),
            Format::Warning(ref e) => Yellow.paint(e.as_ref()),
            Format::Good(ref e) => Green.paint(e.as_ref()),
        }
    }

}

#[cfg(all(feature = "color", not(target_os = "windows")))]
impl<T: AsRef<str>> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.format())
    }
}

#[cfg(any(not(feature = "color"), target_os = "windows"))]
impl<T: fmt::Display> Format<T> {
    fn format(&self) -> &T {
        match *self {
            Format::Error(ref e) => e,
            Format::Warning(ref e) => e,
            Format::Good(ref e) => e,
        }
    }
}

#[cfg(any(not(feature = "color"), target_os = "windows"))]
impl<T: fmt::Display> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.format())
    }
}

pub fn format_number(n: u64, sep: Option<char>) -> String {
    debugln!("executing; format_number; n={}", n);
    let s = format!("{}", n);
    if let Some(sep) = sep {
        debugln!("There was a separator {}", sep);
        let mut ins_sep = s.len() % 3;
        ins_sep = if ins_sep == 0 { 3 } else {ins_sep};
        let mut ret = vec![];
        for (i, c) in s.chars().enumerate() {
            debugln!("iter; c={}; ins_sep={}; ret={:?}", c, ins_sep, ret);
            if ins_sep == 0 && i != 0 {
                debugln!("Inserting the separator");
                ret.push(sep);
                ins_sep = 3;
            }
            ret.push(c);
            ins_sep -= 1;
        }
        debugln!("returning; ret={}", ret.iter().cloned().collect::<String>());
        ret.iter().cloned().collect()
    } else {
        debugln!("There was not a separator");
        s
    }
}