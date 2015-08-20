use std::fs;
use std::fs::metadata;
use std::path::PathBuf;

use glob;

pub fn get_all_files<'a>(path: &'a str, exclude: &Vec<&'a str>) -> Vec<PathBuf> {
    debugln!("executing; get_all_files; path={:?}; exclude={:?};", path, exclude);
    let mut files = vec![];

    debugln!("Getting metadata");
    if let Ok(result) = metadata(&path) {
        debugln!("Found");
        if result.is_dir() {
            debugln!("It's a dir");
            let dir = fs::read_dir(&path).unwrap();
            'file: for entry in dir {
                let entry = entry.unwrap();
                let file_path = entry.path();
                let file_str = file_path.to_str().expect("file_path isn't a valid str");
                let file_string = file_str.to_owned();
                let path_metadata = metadata(&file_string).unwrap();

                for ignored in exclude {
                    debugln!("iter; ignored={:?}", ignored);
                    if file_str.contains(ignored) {
                        debugln!("iter; ignored={:?}", ignored);
                        continue 'file;
                    }
                }
                if path_metadata.is_dir() {
                    for file in get_all_files(&*file_string, &exclude) {
                        files.push(file);
                    }
                } else if path_metadata.is_file() {
                    files.push(PathBuf::from(file_str));
                }
            }
        } else {
            debugln!("It's a file");
            if !exclude.contains(&path) {
                debugln!("It's not excluded");
                files.push(PathBuf::from(path));
            } else {
                debugln!("It's excluded");
            }
        }
    } else {
        for path_buf in glob::glob(&path).ok().expect("failed to get files from glob") {
            let file_path = path_buf.unwrap();
            files.push(file_path);
        }
    }

    files
}