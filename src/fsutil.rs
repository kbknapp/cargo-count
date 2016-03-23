use std::fs;
use std::io::Result;
use std::path::PathBuf;
use gitignore::File;

use glob;

pub fn get_all_files(v: &mut Vec<PathBuf>,
                         path: &PathBuf,
                         exclude: &[PathBuf],
                         follow_links: bool,
                         gitignore: &Option<File>) {
    debugln!("executing; get_all_files; path={:?}; exclude={:?}; all={:?}",
             path,
             exclude,
             all);
    if exclude.contains(path) {
        return;
    }

    if let Some(ref f) = *gitignore {
        if f.is_excluded(path).unwrap() {
            return;
        }
    }

    debugln!("Getting metadata");
    if let Ok(result) = get_metadata(&path, follow_links) {
        debugln!("Found");
        if result.is_dir() {
            debugln!("It's a dir");
            let dir = fs::read_dir(&path).unwrap();
            for entry in dir {
                let entry = entry.unwrap();
                let file_path = entry.path();
                get_all_files(v,
                              &file_path.to_path_buf(),
                              &exclude,
                              follow_links,
                              gitignore);
            }
        } else {
            debugln!("It's a file");
            v.push(path.clone());
        }
    } else {
        for path_buf in glob::glob(path.to_str().unwrap_or(""))
                            .expect("failed to get files from glob") {
            if let Ok(file_path) = path_buf {
                if let Ok(result) = get_metadata(&file_path, follow_links) {
                    if result.is_dir() {
                        debugln!("It's a dir");
                        let dir = fs::read_dir(&path).unwrap();
                        for entry in dir {
                            let entry = entry.unwrap();
                            let file_path = entry.path();
                            get_all_files(v,
                                          &file_path.to_path_buf(),
                                          &exclude,
                                          follow_links,
                                          gitignore);
                        }
                    } else {
                        debugln!("It's a file");
                        v.push(path.clone());
                    }
                }
            }
        }
    }
}

fn get_metadata(path: &PathBuf, follow_links: bool) -> Result<fs::Metadata> {
    if follow_links {
        fs::metadata(path)
    } else {
        fs::symlink_metadata(path)
    }
}
