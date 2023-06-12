use std::{path::{PathBuf, Path}, fs::{self, ReadDir}};

use path_clean::PathClean;

use crate::explorer::PathId;




pub fn remove(path_id: PathId) -> Result<(), String> {
    if path_id.is_dir {
        fs::remove_dir_all(path_id.path).map_err(|e| e.to_string())
    } else {
        fs::remove_file(path_id.path).map_err(|e| e.to_string())
    }
    
}




/// from: absolute path
/// to: relative path
pub fn rename(from: &PathBuf, to_relative: String) -> Result<(), String> {
    // todo: handle this case: from = /a/b , to = /a/b/c
    // we need a temp directory to mov the file b inside it, then we can create the b directory


    let parent_from = match from.parent() {
        Some(path) => path,
        None => return Err("no parent".to_string()),
    };

    let to_absolute = parent_from.join(to_relative).clean();

    println!("from {}, to {}", from.display(), to_absolute.display());

    let parent_to = match to_absolute.parent() {
        Some(path) => path,
        None => return Err("no parent".to_string()),
    };

    // ignore, because rename will fail if dir not exists
    let _ = fs::create_dir_all(parent_to);

    fs::rename(from, to_absolute).map_err(|e| e.to_string())

}



// return true if the node is a dir
pub fn is_dir_exist(path: &PathBuf) -> Result<bool, String> {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(error) => Err(format!(
            "error: can't get metadata of {}: {}",
            path.display(),
            error
        )),
    }
}


pub fn get_dir_entries(path: &PathBuf) -> Result<ReadDir, String>{
    fs::read_dir(path).map_err(|e: std::io::Error| e.to_string())
}