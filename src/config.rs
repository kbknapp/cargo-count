use num_cpus;
use atty;
use clap::ArgMatches;

use error::{CliError, CliResult};
use language::Language;
use std::env;
use std::path::{Path, PathBuf};

arg_enum! {
    #[derive(Debug)]
    pub enum Utf8Rule {
        Ignore,
        Lossy,
        Strict
    }
}

#[derive(Debug)]
pub struct Config {
    pub verbose: bool,
    pub all: bool,
    pub thousands: Option<char>,
    pub utf8_rule: Utf8Rule,
    pub usafe: bool,
    pub exclude: Vec<PathBuf>,
    pub exts: Option<Vec<String>>,
    pub follow_links: bool,
    pub paths: Vec<PathBuf>,
}

impl Config {
    pub fn from_matches(m: &ArgMatches) -> CliResult<Self> {
        Ok(Config {
               verbose: m.is_present("verbose"),
               all: m.is_present("all"),
               mmap: m.is_present("mmap"),
               thousands: m.value_of("separator").map(|s| s.chars().nth(0).unwrap()),
               usafe: m.is_present("unsafe-statistics"),
               utf8_rule: value_t!(m.value_of("utf8-rule"), Utf8Rule).unwrap_or(Utf8Rule::Strict),
               exclude: exclude(m),
               threads: threads(m),
               exts: m.values_of("language").map(|v| v.collect()),
               follow_links: m.is_present("follow-symlinks"),
               paths: paths(m),
           })
    }

    /// Returns true if there is exactly one file path given to search.
    pub fn is_one_path(&self) -> bool {
        self.paths.len() == 1 && self.paths[0].is_file()
    }

    /// Create a worker whose configuration is taken from the
    /// command line.
    pub fn worker(&self) -> Worker {
        WorkerBuilder::new()
            .mmap(self.mmap)
            .build()
    }

}

/// Returns the approximate number of threads
fn threads(m: &ArgMatches) -> usize {
    let threads = m.value_of("threads").unwrap();
    if threads == 0 {
        return cmp::min(12, num_cpus::get());
    }
    threads
}

/// Return all file paths that cargo-count should search.
fn paths(m: &ArgMatches) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = match m.values_of_os("PATH") {
        None => vec![],
        Some(vals) => vals.map(|p| Path::new(p).to_path_buf()).collect(),
    };
    paths
}

fn exclude(m: &ArgMatches) -> Vec<PathBuf> {
    if let Some(v) = m.values_of("exclude") {
        debugln!("There are some");
        let mut ret = vec![];
        for p in v {
            let pb = Path::new(p);
            if pb.is_relative() {
                ret.push(env::current_dir()?.join(p));
            } else {
                ret.push(pb.to_path_buf());
            }
        }
        debugln!("found files or dirs: {:?}", ret);
        ret.push(env::current_dir()?.join(".git"));
        return ret;
    }
    debugln!("There aren't any, adding .git");
    vec![env::current_dir()?.join(".git")]
}

/// Return all of the ignore files given on the command line.
fn ignore_files(m: &ArgMatches) -> Vec<PathBuf> {
    match m.values_of_os("ignore-file") {
        None => return vec![],
        Some(vals) => vals.map(|p| Path::new(p).to_path_buf()).collect(),
    }
}

// to_count: if let Some(v) = m.values_of("PATH") {
//     debugln!("There are some");
//     let mut ret = vec![];
//     for p in v {
//         ret.push(PathBuf::from(p));
//     }
//     debugln!("found files or dirs: {:?}", ret);
//     ret
// } else {
//     debugln!("There aren't any, using cwd");
//     vec![cli_try!(env::current_dir())]
// },