pub mod validators;

use clap::{App, Arg, SubCommand, AppSettings};

pub fn build_cli() -> App<'static, 'static> {
    App::new("cargo-count")
        .version(concat!("v", crate_version!()))
        // We have to lie about our binary name since this will be a third party subcommand for
        // cargo but we want usage strings to generated properly
        .bin_name("cargo")
        // Global version uses the version we supplied (Cargo.toml) for all subcommands as well
        .settings(&[AppSettings::GlobalVersion,
                    AppSettings::SubcommandRequired])
        // We use a subcommand because everything parsed after `cargo` is sent to the third party
        // plugin which will then be interpreted as a subcommand/positional arg by clap
        .subcommand(SubCommand::with_name("count")
            .author("Kevin K. <kbknapp@gmail.com>")
            .max_term_width(80)
            .about("Displays line counts of code for cargo projects")
            .arg(Arg::from_usage("-j, --threads [NUM]")
                .default_value("0")
                .hide_default_value(true)
                .help("The number of threads to use (defaults to auto detection)"))
            .arg(Arg::from_usage("--mmap")
                .help("Load files using memory maps"))
            .arg(Arg::from_usage("-e, --exclude [PATH]...")
                .help("Files or directories to exclude (automatically includes \'.git\')"))
            .arg(Arg::from_usage("-a, --all")
                .help("Do not ignore .gitignore'd paths"))
            .arg(Arg::from_usage("-i, --ignore-file [FILE]...")
                .number_of_values(1)
                .help("Specify additional ignore files"))
            .arg(Arg::from_usage("--unsafe-statistics")
                .help("Displays lines and percentages of \"unsafe\" code"))
            .arg(Arg::from_usage("-l, --language [EXT]...")
                .number_of_values(1)
                .validator(validators::supported_lang)
                .help("Only count these languges, by file extension (i.e. \'-ljs -lpy -lcpp\')"))
            .arg(Arg::from_usage("-v, --verbose")
                .help("Print verbose output"))
            .arg(Arg::from_usage("-S, --follow-symlinks")
                .help("Follows symlinks and counts source files it finds"))
            .arg(Arg::from_usage("[PATH]...")
                .default_value("./")
                .help("The files or directories (including children) to count"))
            .arg(Arg::from_usage("-s, --separator [CHAR]")
                .use_delimiter(false) // Because ',' is interpretted as a delimiter
                .validator(validators::single_char)
                .help("Set the thousands separator for pretty printing"))
            .arg(Arg::from_usage("--utf8-rule [RULE]")
                .default_value("strict")
                .possible_values(&UTF8_RULES)
                .help("Sets the UTF-8 parsing rule"))
            .after_help("\
                When using '--exclude <PATH>' the path given can either be relative to the \
                current directory, or absolute. 
                \n\
                Globs are also supported. For example, to exclude 'test.rs' files from all child \
                directories of the current directory you could do '--exclude */test.rs'."))
}