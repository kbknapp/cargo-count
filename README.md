# cargo-count
Linux: [![Build Status](https://travis-ci.org/kbknapp/cargo-count.svg?branch=master)](https://travis-ci.org/kbknapp/cargo-count)

A cargo subcommand for displaying line counts of source code in projects

## Demo

Running `cargo count -s , --unsafe-statistics` in the [Rust](https://github.com/rust-lang/rust) repo yields these results:

```
Gathering information...
         Language    Files  Lines    Blanks  Comments  Code     Unsafe (%)
         --------    -----  -----    ------  --------  ----     ----------
         Rust        6,015  527,198  63,300  161,789   302,109  1,162 (0.38%)
         CSS         4      1,262    99      490       673
         Python      31     4,797    822     680       3,295
         C           54     9,962    1,154   2,945     5,863    5,836 (99.54%)
         C Header    13     1,865    243     650       972      937 (96.40%)
         JavaScript  4      1,118    131     142       845
         C++         4      1,611    185     81        1,345    1,345 (100.00%)
         --------    -----  -----    ------  --------  ----     ----------
Totals:              6,125  547,813  65,934  166,777   315,102  9,280 (2.95%)
```

The `-s ,` sets a `,` character as the thousands separator, and `--unsafe-statistics` looks for, and counts `unsafe` blocks.

## Compiling

Follow these instructions to compile `cargo-count`, then skip down to Installation.

 1. Ensure you have current version of `cargo` and [Rust](https://www.rust-lang.org) installed
 2. Clone the project `$ git clone https://github.com/kbknapp/cargo-count && cd cargo-count`
 3. Build the project `$ cargo build --release`
 4. Once complete, the binary will be located at `target/release/cargo-count`

## Installation and Usage

All you need to do is place `cargo-count` somewhere in your `$PATH`. Then run `cargo count` anywhere in your project directory. For full details see below.

### Linux / OS X

You have two options, place `cargo-count` into a directory that is already located in your `$PATH` variable (To see which directories those are, open a terminal and type `echo "${PATH//:/\n}"`, the quotation marks are important), or you can add a custom directory to your `$PATH`

**Option 1**
If you have write permission to a directory listed in your `$PATH` or you have root permission (or via `sudo`), simply copy the `cargo-count` to that directory `# sudo cp cargo-count /usr/local/bin`

**Option 2**
If you do not have root, `sudo`, or write permission to any directory already in `$PATH` you can create a directory inside your home directory, and add that. Many people use `$HOME/.bin` to keep it hidden (and not clutter your home directory), or `$HOME/bin` if you want it to be always visible. Here is an example to make the directory, add it to `$PATH`, and copy `cargo-count` there.

Simply change `bin` to whatever you'd like to name the directory, and `.bashrc` to whatever your shell startup file is (usually `.bashrc`, `.bash_profile`, or `.zshrc`)

```sh
$ mkdir ~/bin
$ echo "export PATH=$PATH:$HOME/bin" >> ~/.bashrc
$ cp cargo-count ~/bin
$ source ~/.bashrc
```

### Windows

On Windows 7/8 you can add directory to the `PATH` variable by opening a command line as an administrator and running

```sh
C:\> setx path "%path%;C:\path\to\cargo-count\binary"
```

Otherwise, ensure you have the `cargo-count` binary in the directory which you operating in the command line from, because Windows automatically adds your current directory to PATH (i.e. if you open a command line to `C:\my_project\` to use `cargo-count` ensure `cargo-count.exe` is inside that directory as well).


### Options

There are a few options for using `cargo-count` which should be somewhat self explanitory.

```
USAGE:
    cargo count [FLAGS] [OPTIONS] [--] [ARGS]

FLAGS:
    -h, --help                 Prints help information
        --ignore               Ignore files and streams with invalid UTF-8
        --unsafe-statistics    Displays percentages of "unsafe" code
    -V, --version              Prints version information
    -v, --verbose              Print verbose output

OPTIONS:
    -l, --language <exts>...    The languages to count by file extension (i.e. '-l js py cpp')
    -e, --exclude <paths>...    Files or directories to exclude
    -s, --separator <sep>       Set the thousands separator for pretty printing

ARGS:
    to_count...    The file or directory to count
                   (defaults to current working directory when omitted)
```

## License

`cargo-count` is released under the terms of either the MIT or Apache 2.0 license. See the LICENSE-MIT or LICENSE-APACHE file for the details.
