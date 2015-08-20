#[macro_use]
extern crate clap;
#[cfg(feature = "color")]
extern crate ansi_term;
extern crate tabwriter;
extern crate glob;
// #![cfg_attr(feature = "unstable", feature(plugin))]
// #![cfg_attr(feature = "unstable", plugin(regex_macros))]
extern crate regex;


use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::env;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use tabwriter::TabWriter;

use language::{Language, Count, Comment};
use error::CliError;
use fmt::Format;

#[macro_use]
mod macros;
mod fmt;
mod language;
mod fsutil;
mod error;

type CliResult<T> = Result<T, CliError>;

struct Config<'a> {
    verbose: bool,
    thousands: Option<char>,
    ignore_utf8: bool,
    usafe: bool,
    exclude: Vec<&'a str>,
    exts: Option<Vec<&'a str>>,
    to_count: Vec<PathBuf>
}

impl<'a> Config<'a> {
    fn from_matches(m: &'a ArgMatches<'a, 'a>) -> CliResult<Self> {
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
            ignore_utf8: m.is_present("ignore"),
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

fn main() {
    debugln!("executing; cmd=cargo-count; args={:?}", env::args().collect::<Vec<_>>());
    let m = App::new("cargo-count")
        .version(&*format!("v{}", crate_version!()))
        // We have to lie about our binary name since this will be a third party
        // subcommand for cargo
        .bin_name("cargo")
        // Global version uses the version we supplied (Cargo.toml) for all subcommands as well
        .settings(&[AppSettings::GlobalVersion,
                    AppSettings::SubcommandRequired])
        // We use a subcommand because parsed after `cargo` is sent to the third party plugin
        // which will be interpreted as a subcommand/positional arg by clap
        .subcommand(SubCommand::with_name("count")
            .author("Kevin K. <kbknapp@gmail.com>")
            .about("Displays line counts of code for cargo projects")
            .args_from_usage("-e, --exclude [paths]...    'Files or directories to exclude'
                              --unsafe-statistics         'Displays percentages of \"unsafe\" code'
                              --ignore                    'Ignore files and streams with invalid UTF-8'
                              -l, --language [exts]...    'The languages to count by file extension (i.e. \'-l js py cpp\')'
                              -v, --verbose               'Print verbose output'
                              [to_count]...               'The file or directory to count{n}\
                                                           (defaults to current working directory when omitted)'")
            .arg(Arg::from_usage("-s, --separator [sep]   'Set the thousands separator for pretty printing'")
                .validator(single_char)))
        .get_matches();

    if let Some(m) = m.subcommand_matches("count") {
        let cfg = Config::from_matches(m).unwrap_or_else(|e| e.exit());
        println!("Gathering information...");
        if let Err(e) = execute(cfg) {
            e.exit();
        }
    }
}

fn execute(cfg: Config) -> CliResult<()> {
    debugln!("executing; cmd=execute;");
    verboseln!(cfg, "{}: {}", Format::Warning("Excluding"), cfg.exclude.connect(", "));
    verbose!(cfg, "{}",
        if cfg.exts.is_some() {
            format!("{} including files with extension: {}\n", Format::Warning("Only"), cfg.exts.as_ref().unwrap().connect(", "))
        } else {
            "".to_owned()
        }
    );

    let mut tw = TabWriter::new(vec![]);
    cli_try!(write!(&mut tw, "\tLanguage\tFiles\tLines\tBlanks\tComments\tCode{}\n", if cfg.usafe {"\tUnsafe (%)"} else {""}));
    cli_try!(write!(&mut tw, "\t--------\t-----\t-----\t------\t--------\t----{}\n", if cfg.usafe {"\t----------"} else {""}));

    debugln!("Checking for files or dirs to count from cli");

    let mut langs: Vec<Count> = vec![];

    for path in cfg.to_count {
        debugln!("iter; path={:?};", path);
        if let Some(f) = path.to_str() {
            let files = fsutil::get_all_files(f, &cfg.exclude);

            for file in files {
                debugln!("iter; file={:?};", file);
                let extension = match Path::new(&file).extension() {
                    Some(result) => {
                        if let Some(ref exts) = cfg.exts {
                            if !exts.contains(&result.to_str().unwrap_or("")) { continue }
                        }
                        result.to_str().unwrap()
                    },
                    None => continue,
                };

                debugln!("found extension: {:?}", extension);
                if let Some(pos_lang) = Language::from_ext(extension) {
                    debugln!("Extension is valid");
                    let mut found = false;
                    debugln!("Searching for previous entries of that type");
                    for l in langs.iter_mut() {
                        if l.lang.extension() == extension {
                            debugln!("Found");
                            found = true;
                            l.add_file(PathBuf::from(&file));
                            break;
                        }
                    }
                    if !found {
                        debugln!("Not found, creating new");
                        let mut c = Count::new(pos_lang, cfg.thousands);
                        c.add_file(PathBuf::from(&file));
                        langs.push(c);
                    }
                } else {
                    debugln!("extension wasn't valid");
                }
            }
        } else {
            debugln!("path couldn't be converted to a str");
        }

    }

    let mut tot: usize = 0;
    let mut tot_lines: u64 = 0;
    let mut tot_comments: u64 = 0;
    let mut tot_blanks: u64 = 0;
    let mut tot_code: u64 = 0;
    let mut tot_usafe: u64 = 0;

    for count in langs.iter_mut() {
        debugln!("iter; count={:?};", count);
        let re = if let Some(kw) = count.lang.unsafe_keyword() {
            regex!(&*format!("[\\[\\] \\{{\\}}]{}[\\[\\] \\{{\\}}\n]", kw))
        } else {
            regex!("")
        };
        for file in count.files.iter() {
            debugln!("iter; file={:?};", file);
            let mut buffer = String::new();

            let mut file_ref = cli_try!(File::open(&file));

            if cfg.ignore_utf8 {
                if let Err(..) = file_ref.read_to_string(&mut buffer) {
                    continue
                }
            } else {
                cli_try!(file_ref.read_to_string(&mut buffer));
            }

            let mut is_in_comments = false;
            let mut is_in_unsafe = false;
            let mut bracket_count: i64 = 0;

            'new_line: for line in buffer.lines() {
                let line = line.trim();
                debugln!("iter; line={:?};", line);
                count.lines += 1;

                if is_in_comments {
                    debugln!("still in comments");
                    if line.contains(count.multi_end().unwrap()) {
                        debugln!("line contained ending comment, stopping comments");
                        is_in_comments = false;
                    }
                    count.comments += 1;
                    continue;
                }
                debugln!("not in comments");

                if line.trim().is_empty() {
                    debugln!("line was empty");
                    count.blanks += 1;
                    continue;
                }
                debugln!("Line isn't empty");

                if let Some(ms) = count.multi_start() {
                    debugln!("This file type has a multi start of: {:?}", ms);
                    if line.starts_with(ms) {
                        debugln!("line starts with multi comment");
                        count.comments += 1;
                        is_in_comments = line.contains(count.multi_end().unwrap());
                        debugln!("line also contained a multi end: {:?}", is_in_comments);
                        continue;
                    } else if line.contains(ms) {
                        debugln!("line contains a multi start");
                        count.code += 1;
                        is_in_comments = line.contains(count.multi_end().unwrap());
                        debugln!("line also contained a multi end: {:?}", is_in_comments);
                        continue;
                    }
                } else {
                    debugln!("No multi line comments for this type");
                }
                debugln!("No multi line comments for this line");

                if let Some(single_comments) = count.single() {
                    debugln!("This type has single line comments: {:?}", single_comments);
                    for single in single_comments {
                        if line.starts_with(single) {
                            debugln!("Line started with a comment");
                            count.comments += 1;
                            continue 'new_line;
                        } else {
                            debugln!("Line dind't start with a comment");
                        }
                    }
                } else {
                    debugln!("No single line comments for this type");
                }

                if cfg.usafe {
                    debugln!("Calculating --unsafe-statistics");
                    if count.lang.is_unsafe() {
                        debugln!("The language is not safe");
                        if let Some(kw) = count.lang.unsafe_keyword() {
                            debugln!("There is a keyword: {}", kw);
                            debugln!("line={:?}", line);
                            if re.is_match(line) {
                                debugln!("It contained the keyword; usafe_line={:?}", line);
                                count.usafe += 1;
                                let after_usafe = line.split(kw).collect::<Vec<_>>()[1];
                                debugln!("after_usafe={:?}", after_usafe);
                                is_in_unsafe = in_unsafe(after_usafe, None);
                                debugln!("after counting brackets; is_in_unsafe={:?}; bracket_count={:?}", is_in_unsafe, bracket_count);
                            } else if is_in_unsafe {
                                debugln!("It didn't contain the keyword, but we are still in unsafe");
                                count.usafe += 1;
                                is_in_unsafe = in_unsafe(line, Some(bracket_count));
                                debugln!("after counting brackets; is_in_unsafe={:?}; bracket_count={:?}", is_in_unsafe, bracket_count);
                            } else {
                                debugln!("It didn't contain the keyword, and we are not in unsafe");
                            }

                            if bracket_count < 0 {
                                debugln!("bracket_count < 0; resetting");
                                bracket_count = 0
                            }
                        } else {
                            debugln!("Language is unsafe, incing the count");
                            count.usafe += 1;
                        }
                    }
                }
                count.code += 1;
            }
        }

        if !cfg.usafe {
            cli_try!(write!(&mut tw, "\t{}\n", count));
        } else {
            let usafe_per = (count.usafe as f64 / count.code as f64) * 100.00f64;
            cli_try!(write!(&mut tw, "\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                count.lang.name(),
                count.total_files(),
                count.lines(),
                count.blanks(),
                count.comments(),
                count.code(),
                if usafe_per == 00f64 { "".to_owned() } else { format!("{} ({:.2}%)", count.usafe(), usafe_per) }
                ));
        }

        tot          += count.files.len();
        tot_lines    += count.lines;
        tot_comments += count.comments;
        tot_blanks   += count.blanks;
        tot_code     += count.code;
        tot_usafe    += count.usafe;
    }

    cli_try!(write!(&mut tw, "\t--------\t-----\t-----\t------\t--------\t----{}\n", if cfg.usafe { "\t----------"}else{""}));
    cli_try!(write!(&mut tw, "{}\t\t{}\t{}\t{}\t{}\t{}{}\n",
        "Totals:",
        fmt::format_number(tot as u64, cfg.thousands),
        fmt::format_number(tot_lines, cfg.thousands),
        fmt::format_number(tot_blanks, cfg.thousands),
        fmt::format_number(tot_comments, cfg.thousands),
        fmt::format_number(tot_code, cfg.thousands),
        if cfg.usafe {
            format!("\t{} ({:.2}%)", fmt::format_number(tot_usafe, cfg.thousands), ((tot_usafe as f64 / tot_code as f64) * 100.00f64) as f64)
        } else {
            "".to_owned()
        }));

    cli_try!(tw.flush());

    verboseln!(cfg, "{} {}", Format::Good("Displaying"), "the results:");
    if tot > 0 {
        cli_try!(write!(io::stdout(), "{}", String::from_utf8(tw.unwrap()).ok().expect("failed to get valid utf8 results")));
    } else {
        println!("\n\tNo source files were found matching the specified criteria");
    }

    Ok(())
}

fn in_unsafe(line: &str, count: Option<i64>) -> bool {
    let mut b: i64 = count.unwrap_or(0);
    for c in line.chars() {
        match c {
            '{' => b += 1,
            '}' => b -= 1,
            _   => (),
        }
    }

    b > 0
}

fn single_char(s: String) -> Result<(), String> {
    if s.len() != 1 {
        Err(format!("the --separator argument option only accepts a single character but found '{}'", Format::Warning(s)))
    } else {
        Ok(())
    }
}