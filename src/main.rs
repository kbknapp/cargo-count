#[macro_use]
extern crate clap;
#[cfg(feature = "color")]
extern crate ansi_term;
extern crate tabwriter;
extern crate glob;
extern crate regex;

use std::io::Write;
#[cfg(feature = "debug")]
use std::env;

use clap::{App, AppSettings, Arg, SubCommand};

use config::Config;
use count::Counts;
use error::{CliError, CliResult};
use fmt::Format;

#[macro_use]
mod macros;
mod comment;
mod config;
mod count;
mod error;
mod fmt;
mod fsutil;
mod language;

static UTF8_RULES: [&'static str; 3] = ["strict", "lossy", "ignore"];

fn main() {
    debugln!("executing; cmd=cargo-count; args={:?}", env::args().collect::<Vec<_>>());
    let m = App::new("cargo-count")
        .version(&*format!("v{}", crate_version!()))
        // We have to lie about our binary name since this will be a third party
        // subcommand for cargo but we want usage strings to generated properly
        .bin_name("cargo")
        // Global version uses the version we supplied (Cargo.toml) for all subcommands as well
        .settings(&[AppSettings::GlobalVersion,
                    AppSettings::SubcommandRequired])
        // We use a subcommand because everything parsed after `cargo` is sent to the third party 
        // plugin which will then be interpreted as a subcommand/positional arg by clap
        .subcommand(SubCommand::with_name("count")
            .author("Kevin K. <kbknapp@gmail.com>")
            .about("Displays line counts of code for cargo projects")
            .args_from_usage("-e, --exclude [paths]...    'Files or directories to exclude (automatically includes \'.git\')'
                              --unsafe-statistics         'Displays lines and percentages of \"unsafe\" code'
                              -l, --language [exts]...    'Only count these languges (by source code extension){n}\
                                                           (i.e. \'-l js py cpp\')'
                              -v, --verbose               'Print verbose output'
                              [to_count]...               'The files or directories (including children) to count{n}\
                                                           (defaults to current working directory when omitted)'")
            .arg(Arg::from_usage("-s, --separator [sep]   'Set the thousands separator for pretty printing'")
                .validator(single_char))
            .arg(Arg::from_usage("--utf8-rule [rule]     'Sets the UTF-8 parsing rule (Defaults to \'strict\'){n}'")
                .possible_values(&UTF8_RULES)))
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

    debugln!("Checking for files or dirs to count from cli");

    let mut counts = Counts::new(&cfg);
    counts.fill_from();
    cli_try!(counts.count());
    cli_try!(counts.write_results());
    Ok(())
}

fn single_char(s: String) -> Result<(), String> {
    if s.len() != 1 {
        Err(format!("the --separator argument option only accepts a single character but found '{}'", Format::Warning(s)))
    } else {
        Ok(())
    }
}
