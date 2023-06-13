use std::{path::{PathBuf, Path}, fs::{self, ReadDir}};
use std::ffi::OsStr;
use std::fs::File;

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

    if to_absolute.file_name().is_none() {
        return Err("no file name".to_string());
    }

    let parent_to = match to_absolute.parent() {
        Some(path) => path,
        None => return Err("no parent".to_string()),
    };


    match fs::metadata(to_absolute.clone()) {
        Ok(_) => { return Err("file or folder already exists".to_string()) }
        Err(_) => {}
    }




    // ignore, because rename will fail if dir not exists
    let _ = fs::create_dir_all(parent_to);


    println!("from {}, to {}", from.display(), to_absolute.display());

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






#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use crate::fs::rename;
    use serial_test::serial;


    #[test]
    #[serial]
    fn absolute() {
        let path = "/tmp/note_test/".to_string();
        let _ = fs::remove_dir_all(path.clone());
        fs::create_dir_all(path.clone() + "dir1").unwrap();
        File::create(path.clone() + "/dir1/file1").unwrap();

        assert_eq!(rename(&PathBuf::from(path.clone() + "/dir1/file1"), path.clone() + "/dir1/file2").is_ok(), true);
        assert_eq!(PathBuf::from(path.clone() + "/dir1/file1").exists(), false);
        assert_eq!(PathBuf::from(path.clone() + "/dir1/file2").is_file(), true);
    }

    #[test]
    #[serial]
    fn relative() {
        let path = "/tmp/note_test/".to_string();
        let _ = fs::remove_dir_all(path.clone());
        fs::create_dir_all(path.clone() + "dir1").unwrap();
        File::create(path.clone() + "/dir1/file1").unwrap();

        assert_eq!(rename(&PathBuf::from(path.clone() + "/dir1/file1"), "file2".to_string()).is_ok(), true);
        assert_eq!(PathBuf::from(path.clone() + "/dir1/file1").exists(), false);
        assert_eq!(PathBuf::from(path.clone() + "/dir1/file2").is_file(), true);


    }
    #[test]
    #[serial]
    fn relative_parent() {
        let path = "/tmp/note_test/".to_string();
        let _ = fs::remove_dir_all(path.clone());

        fs::create_dir_all(path.clone() + "dir1/dir1.1/").unwrap();
        File::create(path.clone() + "/dir1/dir1.1/file1").unwrap();
        fs::create_dir_all(path.clone() + "dir2").unwrap();

        assert_eq!(rename(&PathBuf::from(path.clone() + "/dir1/dir1.1/file1"), "../../dir2/file2".to_string()).is_ok(), true);
        assert_eq!(PathBuf::from(path.clone() + "/dir2/file2").exists(), true);
        assert_eq!(PathBuf::from(path.clone() + "/dir1/dir1.1/file1").is_file(), false);
    }


    #[test]
    #[serial]
    fn rename_folder() {
        let path = "/tmp/note_test/".to_string();
        let _ = fs::remove_dir_all(path.clone());

        fs::create_dir_all(path.clone() + "dir1/dir1.1/dir1.1.1").unwrap();
        File::create(path.clone() + "/dir1/dir1.1/file1").unwrap();

        assert_eq!(rename(&PathBuf::from(path.clone() + "/dir1/"), "dir2".to_string()).is_ok(), true);
        assert_eq!(PathBuf::from(path.clone() + "/dir1/").exists(), false);
        assert_eq!(PathBuf::from(path.clone() + "/dir2").is_dir(), true);
        assert_eq!(PathBuf::from(path.clone() + "/dir2/dir1.1/dir1.1.1").is_dir(), true);
        assert_eq!(PathBuf::from(path.clone() + "/dir2/dir1.1/file1").is_file(), true);
    }

    #[test]
    #[serial]
    fn bad_name() {
        let path = "/tmp/note_test/".to_string();
        let _ = fs::remove_dir_all(path.clone());
        fs::create_dir_all(path.clone() + "dir1").unwrap();
        File::create(path.clone() + "/dir1/file1").unwrap();

        assert_eq!(rename(&PathBuf::from(path.clone() + "/dir1/file1"), "./".to_string()).is_err(), true);
        assert_eq!(PathBuf::from(path.clone() + "/dir1/file1").is_file(), true);

        // bad name 2
        assert_eq!(rename(&PathBuf::from(path.clone() + "/dir1/file1"), "..".to_string()).is_err(), true);
        assert_eq!(PathBuf::from(path.clone() + "/dir1/file1").is_file(), true);
    }

    // rename a file with a folder with the same name ie: /a/b/c -> /a/b/c/d
    #[test]
    #[serial]
    fn need_temp() {
        let path = "/tmp/note_test/".to_string();
        let _ = fs::remove_dir_all(path.clone());
        fs::create_dir_all(path.clone() + "dir1").unwrap();
        File::create(path.clone() + "/dir1/file1").unwrap();

        assert_eq!(rename(&PathBuf::from(path.clone() + "/dir1/file1"), "file1/file1".to_string()).is_ok(), true);
        assert_eq!(PathBuf::from(path.clone() + "/dir1/file1").is_dir(), true);
        assert_eq!(PathBuf::from(path.clone() + "/dir1/file1/file1").is_file(), true);
    }

    #[test]
    #[serial]
    fn already_exists() {
        let path = "/tmp/note_test/".to_string();
        let _ = fs::remove_dir_all(path.clone());
        fs::create_dir_all(path.clone() + "dir1").unwrap();
        File::create(path.clone() + "/dir1/file1").unwrap();
        fs::create_dir_all(path.clone() + "dir2").unwrap();
        File::create(path.clone() + "/dir2/file1").unwrap();

        let mut f1 = File::open(path.clone() + "/dir1/file1").unwrap();
        let mut f2 = File::open(path.clone() + "/dir2/file1").unwrap();


        let mut ct_f1 = String::new();
        let mut ct_f2 = String::new();

        f1.read_to_string(&mut ct_f1).unwrap();
        f2.read_to_string(&mut ct_f2).unwrap();

        assert_eq!(rename(&PathBuf::from(path.clone() + "/dir1/file1"), "/dir2/file1".to_string()).is_err(), true);

        let mut res_ct_f1 = String::new();
        let mut res_ct_f2 = String::new();

        f1.read_to_string(&mut res_ct_f1).unwrap();
        f2.read_to_string(&mut res_ct_f2).unwrap();

        assert_eq!(ct_f1, res_ct_f1);
        assert_eq!(ct_f2, res_ct_f2);
    }

}

