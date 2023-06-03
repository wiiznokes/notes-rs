#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use std::ffi::OsStr;
use std::fs;
use std::path::{Iter, Path, PathBuf};


#[derive(Debug, Clone)]
pub struct Dir {
    pub path: PathBuf,
    pub is_expanded: bool,
    pub content: Vec<Node>,

    pub name: String,
    pub name_cached: String,
    pub is_name_is_edited: bool,
}

impl Default for Dir {
    fn default() -> Self { 
        Dir {
            path: PathBuf::from(""),
            is_expanded: false,
            name: String::from(""),
            name_cached: String::from(""),
            content: Vec::new(),
            is_name_is_edited: false
        }
    }
}



#[derive(Debug, Clone)]
pub struct File {
    pub path: PathBuf,
    pub extension: String,

    pub name: String,
    pub name_cached: String,
    pub is_name_is_edited: bool,
}

impl Default for File {
    fn default() -> Self { 
        File {
            path: PathBuf::from(""),
            extension: String::from(""),
            name: String::from(""),
            name_cached: String::from(""),
            is_name_is_edited: false
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Dir(Dir),
    File(File),
}




impl Node {
    pub fn is_dir(&self) -> bool {
        match &self {
            Node::Dir(_) => true,
            Node::File(_) => false,
        }
    }

    pub fn path(&self) -> PathBuf {
        match &self {
            Node::Dir(dir) => dir.path.clone(),
            Node::File(file) => file.path.clone(),
        }
    }

    pub fn full_name(&self) -> String {
        match &self {
            Node::Dir(dir) => dir.name.clone(),
            Node::File(file) => file.name.clone(),
        }
    }

}




/// Construct a node of type Dir from a path
/// 
/// Condition: root_path is a dir
pub fn create_node(path: PathBuf) -> Result<Dir, String> {

    if let Err(e) = is_dir_exist(&path) {
        return Err(e);
    }


    let dir_name = match path.clone().file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => {
            return Err(format!("can't read the name of the path {}", path.to_string_lossy()));
        }
    };
    
    let mut content = Vec::new();
    let dir_entries = match fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(error) => {
            return Err(format!(
                "Erreur lors de la lecture de {}: {}",
                path.display(),
                error
            ))
        }
    };


    for entry_opt in dir_entries {
        match entry_opt {
            Ok(entry) => {
                let entry_path = entry.path();

                if !entry_path.is_dir() && !entry_path.is_file() {
                    println!("spécial file or dir have been passed: {}", entry_path.display());
                    continue;
                }


                let node = if entry_path.is_dir() {


                    let entry_name = match entry_path.clone().file_name() {
                        Some(name) => name.to_string_lossy().to_string(),
                        None => {
                            return Err(format!("can't read the name of the path {}", entry_path.to_string_lossy()));
                        }
                    };

                    Node::Dir(Dir {
                        path: entry_path,
                        name: entry_name,
                        ..Default::default()
                    })

                } else {
                    
                    let entry_extension = entry_path.clone().extension()
                        .unwrap_or(OsStr::new("")).to_string_lossy().to_string();

                    let entry_name = match entry_path.clone().file_name() {
                        Some(name) => name.to_string_lossy().to_string(),
                        None => {
                            return Err(format!("can't read the name of the path {}", entry_path.to_string_lossy()));
                        }
                    };

                    Node::File(File {
                        extension: entry_extension,
                        name: entry_name,
                        path: entry_path,
                        ..Default::default()
                    })
                };

                insert_node_sorted(node, &mut content);
            }
            Err(error) => {
                return Err(format!(
                    "Erreur lors de la lecture du contenu de {}: {}",
                    path.display(),
                    error
                ))
            }
        }
    }

    Ok(
        Dir {
            path: path,
            is_expanded: true,
            content: content,

            name: dir_name,
            ..Default::default()
        }
    )

    
}



// Vérifie si le chemin est un répertoire existant
pub fn is_dir_exist(path: &PathBuf) -> Result<(), String> {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(())
            } else {
                Err(format!("error: {} n'est pas un répertoire.", path.display()))
            }
        }
        Err(error) => Err(format!(
            "error: can't get metadata of {}: {}",
            path.display(),
            error
        )),
    }
}

/// Insert node in content with this rules:
/// - all directory before files
/// - alpha numeric (ASCII), with case insensitive (a = A)
/// condition: content must be sorted with this rules before using this function
fn insert_node_sorted(node: Node, content: &mut Vec<Node>) {

    // notice we use negation when node is a dir
    // because 0 will have a smaller index than 1
    let first_key = !node.is_dir();

    // we lower all letter because 'A' < '_' < 'a' in ASCII, and
    // I prefer having '.' and '_' files on top
    let second_key = node.full_name().to_lowercase();

    // we use a third key in case of egality, because Linux is sensitive (a != A) 
    let third_key = node.full_name();
    
    let insert_index = content.binary_search_by_key(
        &(first_key, second_key, third_key), 
    |n| {
        (!n.is_dir(), n.full_name().to_lowercase(), node.full_name())
    });

    match insert_index {
        Ok(idx) => content.insert(idx, node),
        Err(idx) => content.insert(idx, node),
    }
}







/// assumptions
/// - path is absolute
/// - dir is a node of type Dir
pub fn get_node(root_dir: &mut Node, path: PathBuf) -> Option<&mut Node> {


    println!("get node: path = {}", path.clone().display());


    let root_dir_path = root_dir.path();
    let mut path_iter = path.iter();

    for root_part in root_dir_path.iter() {
        let path_part = path_iter.next()
            .expect("path is not in root path");

        if (root_part != path_part) {
            panic!("path is not in root path");
        }
        
    }
    

  
    let mut path_part_opt = path_iter.next();
 
    if (path_part_opt.is_none()) {
        println!("it's the root node !");
        return Some(root_dir);
    }

    
    let mut current = root_dir;


    while (path_part_opt.is_some()) {
        if let Some(path_part) = path_part_opt {
            match current {
                Node::File(file) => {
                    println!("file: {}", file.name);

                    if (path_part == file.path && path_iter.next().is_none()) {
                        println!("it's a file");
                        return Some(current);
                    } else {
                        println!("not found in file");
                        return None;
                    }
    
                }

                Node::Dir(dir) => {
                    println!("dir: {}", dir.name);

                    let next_node: Option<&mut Node> = dir
                        .content
                        .iter_mut()
                        .find(|node| node.full_name() == path_part.to_str().unwrap());

                    match next_node {
                        None => {
                            println!("not found in dir");
                            return None;
                        }
                        Some(node) => {
                            current = node;
                        }
                    }
                }
            }
            path_part_opt = path_iter.next();
        }
       
    }

    Some(current)
}

