//! A cargo subcommand for displaying line counts of source code in projects,
//! including a niave `unsafe` counter for Rust source files. This subcommand
//! was originally based off and inspired by the project
//! [tokei](https://github.com/aaronepower/tokei) by
//! [Aaronepower](https://github.com/aaronepower)
//!
//! ## Demo
//!
//! To count the source code in the [Rust](https://github.com/rust-lang/rust)
//! repository (checkout `4c99649`), and print some naive statistics on how much
//! "unsafe" code exists.
//!
//! **NOTE:** The Rust repository is quite large, if you're on a slow internet
//! connection consider using a smaller repository, such as the `cargo-count`
//! repo.
//!
//! ```ignore
//! $ git clone https://github.com/rust-lang/rust
//! $ cd rust
//! $ cargo count --separator , --unsafe-statistics
//! Gathering information...
//!          Language    Files  Lines    Blanks  Comments  Code     Unsafe (%)
//!          --------    -----  -----    ------  --------  ----     ----------
//! Rust        6,018  528,510  66,984  133,698   327,792  3,163
//! (0.96%)
//! C           54     9,962    1,445   1,492     7,025    7,025
//! (100.00%)
//!          CSS         4      1,266    149     52        1,065
//!          JavaScript  4      1,118    131     166       821
//!          Python      31     4,797    843     585       3,369
//! C Header    13     1,865    284     585       996      996
//! (100.00%)
//! C++         4      1,611    185     81        1,345    1,345
//! (100.00%)
//!          --------    -----  -----    ------  --------  ----     ----------
//! Totals:              6,128  549,129  70,021  136,659   342,413  12,529
//! (3.66%)
//!
//! ```
//!
//! The `--separator ,` sets a `,` character as the thousands separator, and
//! `--unsafe-statistics` looks for, and counts lines of `unsafe`.
//!
//! ## Compiling
//!
//! Follow these instructions to compile `cargo-count`, then skip down to
//! Installation.
//!
//!  1. Ensure you have current version of `cargo` and
//!     [Rust](https://www.rust-lang.org) installed
//!  2. Clone the project
//!     `$ git clone https://github.com/kbknapp/cargo-count && cd cargo-count`
//!  3. Build the project `$ cargo build --release` (**NOTE:** There is a large
//!     performance differnce when compiling without optimizations, so I
//!     recommend alwasy using `--release` to enable to them)
//!  4. Once complete, the binary will be located at
//!     `target/release/cargo-count`
//!
//! ## Installation and Usage
//!
//! All you need to do is place `cargo-count` somewhere in your `$PATH`. Then
//! run `cargo count` anywhere in your project directory. For full details see
//! below.
//!
//! ### Linux / OS X
//!
//! You have two options, place `cargo-count` into a directory that is already
//! located in your `$PATH` variable (To see which directories those are, open
//! a terminal and type `echo "${PATH//:/\n}"`, the quotation marks are
//! important), or you can add a custom directory to your `$PATH`
//!
//! **Option 1**
//! If you have write permission to a directory listed in your `$PATH` or you
//! have root permission (or via `sudo`), simply copy the `cargo-count` to that
//! directory `# sudo cp cargo-count /usr/local/bin`
//!
//! **Option 2**
//! If you do not have root, `sudo`, or write permission to any directory
//! already in `$PATH` you can create a directory inside your home directory,
//! and add that. Many people use `$HOME/.bin` to keep it hidden (and not
//! clutter your home directory), or `$HOME/bin` if you want it to be always
//! visible. Here is an example to make the directory, add it to `$PATH`, and
//! copy `cargo-count` there.
//!
//! Simply change `bin` to whatever you'd like to name the directory, and
//! `.bashrc` to whatever your shell startup file is (usually `.bashrc`,
//! `.bash_profile`, or `.zshrc`)
//!
//! ```sh
//! $ mkdir ~/bin
//! $ echo "export PATH=$PATH:$HOME/bin" >> ~/.bashrc
//! $ cp cargo-count ~/bin
//! $ source ~/.bashrc
//! ```
//!
//! ### Windows
//!
//! On Windows 7/8 you can add directory to the `PATH` variable by opening a
//! command line as an administrator and running
//!
//! ```sh
//! C:\> setx path "%path%;C:\path\to\cargo-count\binary"
//! ```
//!
//! Otherwise, ensure you have the `cargo-count` binary in the directory which
//! you operating in the command line from, because Windows automatically adds
//! your current directory to PATH (i.e. if you open a command line to
//! `C:\my_project\` to use `cargo-count` ensure `cargo-count.exe` is inside
//! that directory as well).
//!
//!
//! ### Options
//!
//! There are a few options for using `cargo-count` which should be somewhat
//! self explanitory.
//!
//! ```ignore
//! USAGE:
//!     cargo count [FLAGS] [OPTIONS] [--] [ARGS]
//!
//! FLAGS:
//! -S, --follow-symlinks      Follows symlinks and counts source files it
//! finds
//! -a, --all                  Do not ignore .gitignored paths
//!                                (Defaults to false when omitted)
//!     -h, --help                 Prints help information
//! --unsafe-statistics    Displays lines and percentages of "unsafe"
//! code
//!     -V, --version              Prints version information
//!     -v, --verbose              Print verbose output
//!
//! OPTIONS:
//! -l, --language <exts>...    Only count these languges (by source code
//! extension)
//!                                 (i.e. '-l js py cpp')
//! -e, --exclude <paths>...    Files or directories to exclude
//! (automatically includes '.git')
//! --utf8-rule <rule>      Sets the UTF-8 parsing rule (Defaults to
//! 'strict')
//!                                  [values: ignore lossy strict]
//! -s, --separator <sep>       Set the thousands separator for pretty
//! printing
//!
//! ARGS:
//!     to_count...    The files or directories (including children) to count
//!                    (defaults to current working directory when omitted)
//!
//! When using '--exclude <path>' the path given can either be relative to the
//! current
//! directory, or absolute. When '<path>' is a file, it must be relative to the
//! current
//! directory or it will not be found. Example, if the current directory has a
//! child
//! directory named 'target' with a child fild 'test.rs' and you use `--exclude
//! target/test.rs'
//!
//! Globs are also supported. For example, to eclude 'test.rs' files from all
//! child directories
//! of the current directory you could do '--exclude */test.rs'.
//! ```
//!
//! ## License
//!
//! `cargo-count` is released under the terms of the MIT. See the LICENSE-MIT
//! file for the details.
#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", allow(explicit_iter_loop))]
#![cfg_attr(feature = "lints", allow(should_implement_trait))]
#![cfg_attr(feature = "lints", allow(unstable_features))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![cfg_attr(not(any(feature = "nightly", feature = "unstable")), deny(unstable_features))]
#![deny(missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_import_braces,
        unused_qualifications)]

#[macro_use]
extern crate clap;
#[cfg(feature = "color")]
extern crate ansi_term;
extern crate tabwriter;
extern crate glob;
extern crate regex;
extern crate gitignore;

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
    debugln!("executing; cmd=cargo-count; args={:?}",
             env::args().collect::<Vec<_>>());
    let m = App::new("cargo-count")
        .version(&*format!("v{}", crate_version!()))
    // We have to lie about our binary name since this will be a third party
    // subcommand for cargo but we want usage strings to generated properly
        .bin_name("cargo")
    // Global version uses the version we supplied (Cargo.toml) for all subcommands
    // as well
        .settings(&[AppSettings::GlobalVersion,
                    AppSettings::SubcommandRequired])
    // We use a subcommand because everything parsed after `cargo` is sent to the
    // third party
    // plugin which will then be interpreted as a subcommand/positional arg by clap
        .subcommand(SubCommand::with_name("count")
            .author("Kevin K. <kbknapp@gmail.com>")
            .about("Displays line counts of code for cargo projects")
            .args_from_usage("
-e, --exclude [PATH]...    'Files or directories to exclude (automatically includes \'.git\')'
-a, --all                  'Do not ignore .gitignore'd paths'
--unsafe-statistics        'Displays lines and percentages of \"unsafe\" code'
-l, --language [EXT]...    'Only count these languges (i.e. \'-l js py cpp\')'
-v, --verbose              'Print verbose output'
-S, --follow-symlinks      'Follows symlinks and counts source files it finds [default: false]'
[PATH]...                  'The files or directories (including children) to count (defaults to \
                            current working directory when omitted)'")
            .arg(Arg::from_usage(
                    "-s, --separator [CHAR]   'Set the thousands separator for pretty printing'")
		.use_delimiter(false)
                .validator(single_char))
            .arg(Arg::from_usage("--utf8-rule [RULE]     'Sets the UTF-8 parsing rule'")
                .default_value("strict")
                .possible_values(&UTF8_RULES))
            .after_help("\
When using '--exclude <PATH>' the path given can either be relative to the current directory, or \
absolute. When '--exclude <PATH>' is a file or path, it must be relative to the current directory \
or it will not be found. Example, if the current directory has a child directory named 'target' \
with a child fild 'test.rs' and you use `--exclude target/test.rs'
\n\
Globs are also supported. For example, to eclude 'test.rs' files from all child directories of \
the current directory you could do '--exclude */test.rs'."))
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
    verboseln!(cfg, "{}: {:?}", Format::Warning("Excluding"), cfg.exclude);
    verbose!(cfg,
             "{}",
             if cfg.exts.is_some() {
                 format!("{} including files with extension: {}\n",
                         Format::Warning("Only"),
                         cfg.exts
                            .as_ref()
                            .unwrap()
                            .join(", "))
             } else {
                 "".to_owned()
             });

    debugln!("Checking for files or dirs to count from cli");

    let mut counts = Counts::new(&cfg);
    counts.fill_from();
    cli_try!(counts.count());
    cli_try!(counts.write_results());
    Ok(())
}

fn single_char(s: String) -> Result<(), String> {
    if s.len() == 1 {
        Ok(())
    } else {
        Err(
          format!(
            "the --separator argument option only accepts a single character but found '{}'",
             Format::Warning(s)))
    }
}
