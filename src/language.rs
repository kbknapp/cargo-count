use comment::Comment;
use std::fmt as StdFmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Language {
    C,
    Header,
    Hpp,
    Cpp,
    Css,
    Html,
    Java,
    JavaScript,
    Perl,
    Php,
    Python,
    Ruby,
    Rust,
    Xml,
    Toml,
    Go,
    Assembly,
    Shell,
    D,
    Nim,
}

impl Language {
    pub fn from_ext(ext: &str) -> Option<Language> {
        match ext {
            "cpp" | "cp" | "cc" | "cxx" | "c++" | "C" => Some(Language::Cpp),
            "hpp" | "h++" => Some(Language::Hpp),
            "c" => Some(Language::C),
            "h" => Some(Language::Header),
            "css" => Some(Language::Css),
            "java" => Some(Language::Java),
            "js" => Some(Language::JavaScript),
            "rs" => Some(Language::Rust),
            "xml" => Some(Language::Xml),
            "html" | "htm" => Some(Language::Html),
            "py" => Some(Language::Python),
            "rb" => Some(Language::Ruby),
            "php" => Some(Language::Php),
            "toml" => Some(Language::Toml),
            "pl" => Some(Language::Perl),
            "go" => Some(Language::Go),
            "agc" | "asm" | "a51" | "inc" | "nasm" | "s" | "ms" => Some(Language::Assembly),
            "ps1" | "psd1" | "psm1" | "sh" | "bash" | "bats" | "cgi" | "command" | "fcgi"
            | "ksh" | "sh.in" | "tmux" | "tool" | "zsh" | "tcsh" | "csh" | "fish" => {
                Some(Language::Shell)
            }
            "d" | "di" => Some(Language::D),
            "nim" | "nimrod" => Some(Language::Nim),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        match *self {
            Language::Cpp => "C++",
            Language::Hpp => "C++ Header",
            Language::C => "C",
            Language::Header => "C Header",
            Language::Css => "CSS",
            Language::Java => "Java",
            Language::JavaScript => "JavaScript",
            Language::Rust => "Rust",
            Language::Xml => "XML",
            Language::Html => "HTML",
            Language::Python => "Python",
            Language::Ruby => "Ruby",
            Language::Php => "PHP",
            Language::Toml => "TOML",
            Language::Perl => "Perl",
            Language::Go => "Go",
            Language::Assembly => "Assembly",
            Language::Shell => "Shell",
            Language::D => "D",
            Language::Nim => "Nim",
        }
    }

    pub fn is_unsafe(&self) -> bool {
        match *self {
            Language::C
            | Language::Cpp
            | Language::Hpp
            | Language::Header
            | Language::Rust
            | Language::Assembly
            | Language::Nim => true,
            _ => false,
        }
    }

    pub fn unsafe_keyword(&self) -> Option<&str> {
        match *self {
            Language::Rust => Some("unsafe"),
            _ => None,
        }
    }
}

impl StdFmt::Display for Language {
    fn fmt(&self, f: &mut StdFmt::Formatter) -> StdFmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Comment for Language {
    type Rep = &'static str;

    fn single(&self) -> Option<Vec<<Self as Comment>::Rep>> {
        match *self {
            Language::C
            | Language::Cpp
            | Language::Hpp
            | Language::Header
            | Language::Css
            | Language::Java
            | Language::JavaScript
            | Language::Rust
            | Language::Go
            | Language::D => Some(vec!["//"]),
            Language::Php => Some(vec!["//", "#"]),
            Language::Xml | Language::Html => Some(vec!["<!--"]),
            Language::Ruby
            | Language::Python
            | Language::Toml
            | Language::Perl
            | Language::Assembly
            | Language::Shell
            | Language::Nim => Some(vec!["#"]),
        }
    }

    fn multi_start(&self) -> Option<<Self as Comment>::Rep> {
        match *self {
            Language::C
            | Language::Cpp
            | Language::Hpp
            | Language::Header
            | Language::Css
            | Language::Java
            | Language::JavaScript
            | Language::Go
            | Language::Rust
            | Language::Php
            | Language::D => Some("/*"),
            Language::Xml | Language::Html => Some("<!--"),
            Language::Ruby => Some("=begin"),
            Language::Python => Some("'''"),
            Language::Nim => Some("#["),
            Language::Toml | Language::Perl | Language::Assembly | Language::Shell => None,
        }
    }

    fn multi_end(&self) -> Option<<Self as Comment>::Rep> {
        match *self {
            Language::C
            | Language::Cpp
            | Language::Hpp
            | Language::Header
            | Language::Css
            | Language::Java
            | Language::Go
            | Language::JavaScript
            | Language::Rust
            | Language::Php
            | Language::D => Some("*/"),
            Language::Xml | Language::Html => Some("-->"),
            Language::Ruby => Some("=end"),
            Language::Python => Some("'''"),
            Language::Nim => Some("]#"),
            Language::Toml | Language::Perl | Language::Assembly | Language::Shell => None,
        }
    }
}
