#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use std::fs;
use std::path::{Iter, Path, PathBuf};


#[derive(Debug, Clone)]
pub struct DirNode {
    pub path: PathBuf,
    pub expanded: bool,
    pub full_name: String,
    pub full_name_cached: String,
    pub edit_active: bool,
    pub content: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Node {
    Dir(DirNode),
    File(FileNode),
}

#[derive(Debug, Clone)]
pub struct FileNode {
    pub extension: String,
    pub full_name: String,
    pub full_name_cached: String,
    pub edit_active: bool,
    pub path: PathBuf,
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
            Node::Dir(dir) => dir.full_name.clone(),
            Node::File(file) => file.full_name.clone(),
        }
    }
}

// Vérifie si le chemin est un répertoire existant
pub fn is_dir_exist(path: &Path) -> Result<(), String> {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(())
            } else {
                Err(format!("{} n'est pas un répertoire.", path.display()))
            }
        }
        Err(error) => Err(format!(
            "Erreur lors de la lecture de {}: {}",
            path.display(),
            error
        )),
    }
}

/// Si l'élément est présent dans le tableau, la méthode retourne l'index de l'élément.
/// Sinon, elle retourne l'index où l'élément pourrait être inséré pour maintenir l'ordre de tri.
pub fn insert_node_sorted(node: Node, content: &mut Vec<Node>) {
    let is_dir = node.is_dir();
    let idx = content.binary_search_by_key(&(!is_dir, node.full_name()), |n| {
        (!n.is_dir(), n.full_name())
    });

    match idx {
        Ok(idx) => content.insert(idx, node),
        Err(idx) => content.insert(idx, node),
    }
}

// Fonction qui crée une structure DirNode remplie avec les données du répertoire spécifié
pub fn create_dir_node(path: &Path) -> Result<DirNode, String> {
    is_dir_exist(path)?;

    let dir_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    let mut content = Vec::new();
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

    for entry in dir_entries {
        match entry {
            Ok(dir_entry) => {
                let entry_path = dir_entry.path();

                if !entry_path.is_dir() && !entry_path.is_file() {
                    continue; // Ignore les fichiers spéciaux (par ex. les fichiers "cachés")
                }
                let node = if entry_path.is_dir() {
                    Node::Dir(create_dir_node(&entry_path)?)
                } else {
                    let entry_path_owned = entry_path.to_owned();

                    Node::File(FileNode {
                        extension: entry_path_owned
                            .extension()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned(),
                        full_name: entry_path_owned
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned(),
                        full_name_cached: entry_path_owned
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned(),
                        path: entry_path_owned,
                        edit_active: false,
                        
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

    Ok(DirNode {
        path: path.to_path_buf(),
        expanded: false,
        full_name: dir_name.clone(),
        full_name_cached: dir_name,
        content,
        edit_active: false
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
                    println!("file: {}", file.full_name);

                    if (path_part == file.path && path_iter.next().is_none()) {
                        println!("it's a file");
                        return Some(current);
                    } else {
                        println!("not found in file");
                        return None;
                    }
    
                }

                Node::Dir(dir) => {
                    println!("dir: {}", dir.full_name);

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

