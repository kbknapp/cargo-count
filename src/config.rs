use std::env;
use std::path::PathBuf;

use clap::ArgMatches;

use error::{CliError, CliResult};
use language::Language;

arg_enum! {
    #[derive(Debug)]
    pub enum Utf8Rule {
        Ignore,
        Lossy,
        Strict
    }
}

#[derive(Debug)]
pub struct Config<'a> {
    pub verbose: bool,
    pub thousands: Option<char>,
    pub utf8_rule: Utf8Rule,
    pub usafe: bool,
    pub exclude: Vec<&'a str>,
    pub exts: Option<Vec<&'a str>>,
    pub to_count: Vec<PathBuf>
}

impl<'a> Config<'a> {
    pub fn from_matches(m: &'a ArgMatches<'a, 'a>) -> CliResult<Self> {
        if let Some(ext_vec) = m.values_of("exts") {
            for e in ext_vec {
                if let None = Language::from_ext(e) {
                    return Err(CliError::UnknownExt(format!("unsupported source code extension '{}'", e.to_owned())))
                }
            }
        }
        Ok(Config {
            verbose: m.is_present("verbose"),
            thousands: m.value_of("sep").map(|s| s.chars().nth(0).unwrap() ),
            usafe: m.is_present("unsafe-statistics"),
            utf8_rule: value_t!(m.value_of("rule"), Utf8Rule).unwrap_or(Utf8Rule::Strict),
            exclude: m.values_of("paths").unwrap_or(vec![".git"]),
            to_count: if let Some(v) = m.values_of("to_count") {
                debugln!("There are some");
                let mut ret = vec![];
                for p in v {
                    ret.push(PathBuf::from(p));
                }
                debugln!("found files or dirs: {:?}", ret);
                ret
            } else {
                debugln!("There aren't any, using cwd");
                vec![cli_try!(env::current_dir())]
            },
            exts: m.values_of("exts")
        })
    }
}