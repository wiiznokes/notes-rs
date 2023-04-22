#![allow(dead_code)]
#![allow(unused_variables)]


use std::fs;
use std::path::{Path, PathBuf};


#[derive(Debug)]
pub struct DirNode {
    pub path: PathBuf,
    pub is_expand: bool,
    pub full_name: String,
    pub content: Vec<Node>,
}

#[derive(Debug)]
pub enum Node {
    Dir(DirNode),
    File(FileNode),
}

#[derive(Debug)]
pub struct FileNode {
    pub extension: String,
    pub full_name: String,
    pub path: PathBuf,
}







impl Node {
    

    pub fn is_dir(&self) -> bool {
        match &self {
            Node::Dir(_) => true,
            Node::File(_) => false
        }
    }

    pub fn path(&self) -> PathBuf {
        match &self {
            Node::Dir(dir) => dir.path.clone(),
            Node::File(file) => file.path.clone()
        }
    }


    pub fn full_name(&self) -> String {
        match &self {
            Node::Dir(dir) => dir.full_name.clone(),
            Node::File(file) => file.full_name.clone()
        }
    }
}


// Vérifie si le chemin est un répertoire existant
fn is_dir_exist(path: &Path) -> Result<(), String> {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(())
            } else {
                Err(format!("{} n'est pas un répertoire.", path.display()))
            }
        }
        Err(error) => Err(format!("Erreur lors de la lecture de {}: {}", path.display(), error)),
    }
}


/// Si l'élément est présent dans le tableau, la méthode retourne l'index de l'élément.
/// Sinon, elle retourne l'index où l'élément pourrait être inséré pour maintenir l'ordre de tri.
pub fn insert_node_sorted(node: Node, content: &mut Vec<Node>) {
    let is_dir = node.is_dir();
    let idx = content
        .binary_search_by_key(&(!is_dir, node.full_name()), |n| (!n.is_dir(), n.full_name()));

    match idx {
        Ok(idx) => content.insert(idx, node),
        Err(idx) => content.insert(idx, node),
    }
}



// Fonction qui crée une structure DirNode remplie avec les données du répertoire spécifié
fn create_dir_node(path: &Path) -> Result<DirNode, String> {
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

                    let log = entry_path_owned
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned();

                    println!("{log}");
                    
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
                        path: entry_path_owned,
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
        is_expand: false,
        full_name: dir_name,
        content,
    })
}





fn print_dir_node(node: &DirNode, indent: usize) {
    let prefix = if node.is_expand { "[-]" } else { "[+]" };
    println!("{:indent$}{} {}", "", prefix, node.full_name, indent = indent);

    for content_node in &node.content {
        match content_node {
            Node::Dir(dir_node) => print_dir_node(dir_node, indent + 2),
            Node::File(file_node) => {
                println!("{:indent$} [F] {}", "", file_node.full_name, indent = indent + 2);
            }
        }
    }
}

pub fn test_file_system() -> Result<(), String> {
    let path = Path::new("aaa_test");
    match create_dir_node(path) {
        Ok(dir_node) => {
            
            print_dir_node(&dir_node, 4);

            Ok(())
        }
        Err(error) => {
            eprintln!("{}", error);
            Err("".to_string())
        }
        
    }
}