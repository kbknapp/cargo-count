

use fmt::Format;use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum CliError {
    Generic(String),
    UnknownExt(String),
    Unknown,
}

// Copies clog::error::Error;
impl CliError {
    /// Return whether this was a fatal error or not.
    #[allow(dead_code)]
    pub fn is_fatal(&self) -> bool {
        // For now all errors are fatal
        true
    }

    /// Print this error and immediately exit the program.
    ///
    /// If the error is non-fatal then the error is printed to stdout and the
    /// exit status will be `0`. Otherwise, when the error is fatal, the error
    /// is printed to stderr and the exit status will be `1`.
    pub fn exit(&self) -> ! {
        if self.is_fatal() {
            wlnerr!("{}", self);
            ::std::process::exit(1)
        } else {
            println!("{}", self);
            ::std::process::exit(0)
        }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} {}", Format::Error("error:"), self.description())
    }
}

impl Error for CliError {
    #[cfg_attr(feature = "lints", allow(match_same_arms))]
    fn description(&self) -> &str {
        match *self {
            CliError::Generic(ref d) => &*d,
            CliError::UnknownExt(ref d) => &*d,
            CliError::Unknown => "An unknown fatal error has occurred, please consider filing a bug-report!",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
