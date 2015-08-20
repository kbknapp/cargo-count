use std::fmt as StdFmt;
use std::ops::Deref;
use std::path::PathBuf;

use fmt;

#[derive(Debug)]
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
	Go
}

impl Language {
	pub fn from_ext(ext: &str) -> Option<Language> {
		match ext {
			"cpp"  => Some(Language::Cpp),
			"hpp"  => Some(Language::Hpp),
			"c"    => Some(Language::C),
			"h"    => Some(Language::Header),
			"css"  => Some(Language::Css),
			"java" => Some(Language::Java),
			"js"   => Some(Language::JavaScript),
			"rs"   => Some(Language::Rust),
			"xml"  => Some(Language::Xml),
			"html" => Some(Language::Html),
			"py"   => Some(Language::Python),
			"rb"   => Some(Language::Ruby),
			"php"  => Some(Language::Php),
			"toml" => Some(Language::Toml),
			"pl"   => Some(Language::Perl),
			"go"   => Some(Language::Go),
			_      => None
		}
	}

	pub fn name(&self) -> &str {
		match *self {
			Language::Cpp        => "C++",
			Language::Hpp        => "C++ Header",
			Language::C          => "C",
			Language::Header     => "C Header",
			Language::Css        => "CSS",
			Language::Java       => "Java",
			Language::JavaScript => "JavaScript",
			Language::Rust       => "Rust",
			Language::Xml        => "XML",
			Language::Html       => "HTML",
			Language::Python     => "Python",
			Language::Ruby       => "Ruby",
			Language::Php        => "PHP",
			Language::Toml       => "TOML",
			Language::Perl       => "Perl",
			Language::Go         => "Go"
		}
	}

	pub fn extension(&self) -> &str {
		match *self {
			Language::Cpp        => "cpp",
			Language::Hpp        => "hpp",
			Language::C          => "c",
			Language::Header     => "h",
			Language::Css        => "css",
			Language::Java       => "java",
			Language::JavaScript => "js",
			Language::Rust       => "rs",
			Language::Xml        => "xml",
			Language::Html       => "html",
			Language::Python     => "py",
			Language::Ruby       => "rb",
			Language::Php        => "php",
			Language::Perl       => "pl",
			Language::Toml       => "toml",
			Language::Go         => "go"
		}
	}

	pub fn is_unsafe(&self) -> bool {
		match *self {
			Language::C | Language::Cpp | Language::Hpp | Language::Header |
			Language::Rust  => true,
			_               => false
		}
	}

	pub fn unsafe_keyword(&self) -> Option<&str> {
		match *self {
			Language::Rust  => Some("unsafe"),
			_               => None
		}
	}
}

impl StdFmt::Display for Language {
	fn fmt(&self, f: &mut StdFmt::Formatter) -> StdFmt::Result {
		write!(f, "{}", self.name())
	}
}

pub trait Comment {
	type Rep;
	fn single(&self) -> Option<Vec<<Self as Comment>::Rep>>;
	fn multi_start(&self) -> Option<<Self as Comment>::Rep>;
	fn multi_end(&self) -> Option<<Self as Comment>::Rep>;
}

impl Comment for Language {
	type Rep = &'static str;

	fn single(&self) -> Option<Vec<<Self as Comment>::Rep>> {
		match *self {
			Language::C |
			Language::Cpp |
			Language::Hpp |
			Language::Header |
			Language::Css |
			Language::Java |
			Language::JavaScript |
			Language::Rust |
			Language::Go           => Some(vec!["//"]),
			Language::Php          => Some(vec!["//", "#"]),
			Language::Xml |
			Language::Html         => Some(vec!["<!--"]),
			Language::Ruby |
			Language::Python |
			Language::Toml |
			Language::Perl         => Some(vec!["#"])

		}
	}

	fn multi_start(&self) -> Option<<Self as Comment>::Rep> {
		match *self {
			Language::C |
			Language::Cpp |
			Language::Hpp |
			Language::Header |
			Language::Css |
			Language::Java |
			Language::JavaScript |
			Language::Go |
			Language::Rust |
			Language::Php           => Some("/*"),
			Language::Xml |
			Language::Html          => Some("<!--"),
			Language::Ruby          => Some("=begin"),
			Language::Python        => Some("'''"),
			Language::Toml |
			Language::Perl          => None
		}
	}

	fn multi_end(&self) -> Option<<Self as Comment>::Rep> {
		match *self {
			Language::C |
			Language::Cpp |
			Language::Hpp |
			Language::Header |
			Language::Css |
			Language::Java |
			Language::Go |
			Language::JavaScript |
			Language::Rust |
			Language::Php          => Some("*/"),
			Language::Xml |
			Language::Html         => Some("-->"),
			Language::Ruby         => Some("=end"),
			Language::Python       => Some("'''"),
			Language::Toml |
			Language::Perl         => None
		}
	}
}

#[derive(Debug)]
pub struct Count {
	pub lang: Language,
	pub files: Vec<PathBuf>,
	pub code: u64,
	pub comments: u64,
	pub blanks: u64,
	pub lines: u64,
	pub usafe: u64,
	pub sep: Option<char>,
}

impl Count {
	pub fn new(lang: Language, sep: Option<char>) -> Self {
		Count {
			lang: lang,
			files: vec![],
			code: 0,
			comments: 0,
			blanks: 0,
			lines: 0,
			usafe: 0,
			sep: sep
		}
	}

	#[allow(dead_code)]
	pub fn is_empty(&self) -> bool {
		self.code == 0 && self.comments == 0 && self.blanks == 0 && self.lines == 0
	}

	pub fn add_file(&mut self, f: PathBuf) {
		self.files.push(f);
	}

	pub fn lines(&self) -> String {
		fmt::format_number(self.lines, self.sep)
	}

	pub fn code(&self) -> String {
		fmt::format_number(self.code, self.sep)
	}

	pub fn blanks(&self) -> String {
		fmt::format_number(self.blanks, self.sep)
	}

	pub fn usafe(&self) -> String {
		fmt::format_number(self.usafe, self.sep)
	}

	pub fn comments(&self) -> String {
		fmt::format_number(self.comments, self.sep)
	}

	pub fn total_files(&self) -> String {
		fmt::format_number(self.files.len() as u64, self.sep)
	}

}

impl Deref for Count {
	type Target = Language;
	fn deref(&self) -> &<Self as Deref>::Target {
		&self.lang
	}
}

impl StdFmt::Display for Count {
	fn fmt(&self, f: &mut StdFmt::Formatter) -> StdFmt::Result {
		write!(f,
			"{}\t{}\t{}\t{}\t{}\t{}",
			self.lang,
			self.total_files(),
			self.lines(),
			self.blanks(),
			self.comments(),
			self.code()
		)
	}
}