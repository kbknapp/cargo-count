use std::fs::File;
use std::io;
use std::path::Path;

use ignore::DirEntry;
use memmap::{Mmap, Protection};

use decoder::DecodeReader;
use pathutil::strip_prefix;
use printer::Printer;
use search_buffer::BufferSearcher;
use search_stream::{InputBuffer, Searcher};

use Result;

pub enum Work {
    DirEntry(DirEntry),
}

pub struct WorkerBuilder {
    opts: Options,
}

#[derive(Clone, Debug)]
struct Options {
    mmap: bool,
    eol: u8,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            mmap: false,
            eol: b'\n',
        }
    }
}

impl WorkerBuilder {
    /// Create a new builder for a worker.
    ///
    /// A reusable input buffer and a grep matcher are required, but there
    /// are numerous additional options that can be configured on this builder.
    pub fn new(grep: Grep) -> WorkerBuilder {
        WorkerBuilder {
            grep: grep,
            opts: Options::default(),
        }
    }

    /// Create the worker from this builder.
    pub fn build(self) -> Worker {
        let mut inpbuf = InputBuffer::new();
        inpbuf.eol(self.opts.eol);
        Worker {
            grep: self.grep,
            inpbuf: inpbuf,
            decodebuf: vec![0; 8 * (1<<10)],
            opts: self.opts,
        }
    }

    /// If enabled, try to use memory maps for searching if possible.
    pub fn mmap(mut self, yes: bool) -> Self {
        self.opts.mmap = yes;
        self
    }
}

/// Worker is responsible for executing searches on file paths, while choosing
/// streaming search or memory map search as appropriate.
pub struct Worker {
    inpbuf: InputBuffer,
    decodebuf: Vec<u8>,
    opts: Options,
}

impl Worker {
    /// Execute the worker with the given printer and work item.
    ///
    /// A work item can either be stdin or a file path.
    pub fn run<W: WriteColor>(
        &mut self,
        printer: &mut Printer<W>,
        work: Work,
    ) -> u64 {
        let result = match work {
            Work::DirEntry(dent) => {
                let mut path = dent.path();
                let file = match File::open(path) {
                    Ok(file) => file,
                    Err(err) => {
                        if !self.opts.no_messages {
                            eprintln!("{}: {}", path.display(), err);
                        }
                        return 0;
                    }
                };
                if let Some(p) = strip_prefix("./", path) {
                    path = p;
                }
                if self.opts.mmap {
                    self.search_mmap(printer, path, &file)
                } else {
                    self.search(printer, path, file)
                }
            }
        };
        match result {
            Ok(count) => {
                count
            }
            Err(err) => {
                if !self.opts.no_messages {
                    eprintln!("{}", err);
                }
                0
            }
        }
    }

    fn search<R: io::Read, W: WriteColor>(
        &mut self,
        printer: &mut Printer<W>,
        path: &Path,
        rdr: R,
    ) -> Result<u64> {
        let rdr = DecodeReader::new(
            rdr, &mut self.decodebuf, self.opts.encoding);
        let searcher = Searcher::new(
            &mut self.inpbuf, printer, &self.grep, path, rdr);
        searcher
            .map_err(From::from)
    }

    fn search_mmap<W: WriteColor>(
        &mut self,
        printer: &mut Printer<W>,
        path: &Path,
        file: &File,
    ) -> Result<u64> {
        if try!(file.metadata()).len() == 0 {
            // Opening a memory map with an empty file results in an error.
            // However, this may not actually be an empty file! For example,
            // /proc/cpuinfo reports itself as an empty file, but it can
            // produce data when it's read from. Therefore, we fall back to
            // regular read calls.
            return self.search(printer, path, file);
        }
        let mmap = try!(Mmap::open(file, Protection::Read));
        let buf = unsafe { mmap.as_slice() };
        if buf.len() >= 3 && Encoding::for_bom(buf).is_some() {
            // If we have a UTF-16 bom in our memory map, then we need to fall
            // back to the stream reader, which will do transcoding.
            return self.search(printer, path, file);
        }
        let searcher = BufferSearcher::new(printer, &self.grep, path, buf);
        Ok(searcher
            .run())
    }
}
