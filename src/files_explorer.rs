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

    has_been_expanded: bool
}

impl Default for Dir {
    fn default() -> Self { 
        Dir {
            path: PathBuf::from(""),
            is_expanded: false,
            name: String::from(""),
            name_cached: String::from(""),
            content: Vec::new(),
            is_name_is_edited: false,
            has_been_expanded: false
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

    pub fn name(&self) -> String {
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

    if let Err(e) = fill_dir_content(&mut content, &path) {
        return Err(e);
    }


    Ok(
        Dir {
            path: path,
            is_expanded: true,
            content: content,

            name: dir_name,
            has_been_expanded: true,
            ..Default::default()
        }
    )

    
}



pub fn expand_dir(dir: &mut Dir) -> Result<(), String> {
    dir.is_expanded = true;

    if dir.has_been_expanded {
        return Ok(());
    }

  
    return fill_dir_content(&mut dir.content, &dir.path.clone());
    
}


fn fill_dir_content(content: &mut Vec<Node>, path: &PathBuf) -> Result<(), String> {

    let dir_entries = match fs::read_dir(path) {
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

                match get_index_sorted(node.name(), node.is_dir(), content) {
                    Ok(index) => return Err(format!("can't insert {} in {}. name already exist in content", node.name(), entry_path.to_string_lossy())),
                    Err(index) => content.insert(index, node),
                }

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

    return Ok(());
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


/// If the value is found then [`Result::Ok`] is returned, containing the index of the matching element. 
/// If the value is not found then [`Result::Err`] is returned, 
/// containing the index where a matching element could be inserted while maintaining sorted order
/// 
/// The sortage follow this rules:
/// - all directory before files
/// - alpha numeric (ASCII), with case insensitive (a = A)
/// 
/// Condition: content must be sorted with this rules before using this function
#[must_use]
fn get_index_sorted(name: String, is_dir: bool, content: &Vec<Node>) -> Result<usize, usize> {

    // notice we use negation when node is a dir
    // because 0 will have a smaller index than 1
    //
    // we lower all letter because 'A' < '_' < 'a' in ASCII, and
    // I prefer having '.' and '_' files on top

    // we use a third key in case of egality, because Linux is sensitive (a != A) 
    content.binary_search_by_key(
        &(!is_dir, name.to_lowercase(), name), 
    |n| {
        (!n.is_dir(), n.name().to_lowercase(), n.name())
    })
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
                        .find(|node| node.name() == path_part.to_str().unwrap());

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




pub fn search_node_by_path(root_node: & Node, search_path: PathBuf, is_dir: bool) -> Result<& Node, String> {

    if !root_node.is_dir() {
        return Err(format!("tring to search a node: {} with a file", search_path.to_string_lossy()));
    }

    let root_dir_path = root_node.path();
    let mut search_path_iter = search_path.iter();

    // make sure /a/b/c/d is in /a/b/c
    for root_part in root_dir_path.iter() {
        let search_path_part = match search_path_iter.next() {
            Some(path) => path,
            None => return Err(format!("path: {} is not contain in root path {}", search_path.to_string_lossy(), root_dir_path.to_string_lossy())),
        };

        if (root_part != search_path_part) {
            return Err(format!("path: {} is not contain in root path {}", search_path.to_string_lossy(), root_dir_path.to_string_lossy()))
        }
        
    }
    

  
    let mut search_path_part_opt = search_path_iter.next();
    
    let mut current_node = root_node;

    while let Some(search_path_part) = search_path_part_opt {
        match current_node {
            Node::Dir(dir) => {

                // pas forcement un dir!
                get_index_sorted(search_path_part, true,)
            },
            Node::File(_) => todo!(),
        }
    }

    while (search_path_part_opt.is_some()) {
        if let Some(path_part) = path_part_opt {
            match current_node {
                Node::File(file) => {
                    println!("file: {}", file.name);

                    if (path_part == file.path && path_iter.next().is_none()) {
                        println!("it's a file");
                        return Some(current_node);
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
                        .find(|node| node.name() == path_part.to_str().unwrap());

                    match next_node {
                        None => {
                            println!("not found in dir");
                            return None;
                        }
                        Some(node) => {
                            current_node = node;
                        }
                    }
                }
            }
            path_part_opt = path_iter.next();
        }
       
    }

    Ok(current_node)

}