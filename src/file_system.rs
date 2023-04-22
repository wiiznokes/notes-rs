#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};


#[derive(Debug)]
pub struct DirNode {
    pub path: PathBuf,
    pub is_expand: bool,
    pub full_name: String,
    pub content: HashMap<String, Node>,
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

// Fonction pour créer la clé de la HashMap
fn create_key(path: &Path) -> String {
    let name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    if path.is_dir() {
        format!("d_{}", name)
    } else {
        format!("f_{}", name)
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


    let mut content = HashMap::new();
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

                

                content.insert(create_key(&entry_path), node);
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

    for (_, content_node) in &node.content {
        match content_node {
            Node::Dir(dir_node) => print_dir_node(dir_node, indent + 2),
            Node::File(file_node) => {
                println!("{:indent$} [F] {}", "", file_node.full_name, indent = indent + 2);
            }
        }
    }
}

pub fn test_file_system() -> Result<(), String> {
    let path = Path::new("src");
    match create_dir_node(path) {
        Ok(dir_node) => {
            
            print_dir_node(&dir_node, 4);
            println!("{:#?}", dir_node);

            Ok(())
        }
        Err(error) => {
            eprintln!("{}", error);
            Err("".to_string())
        }
        
    }
}