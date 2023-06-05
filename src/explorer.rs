#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use std::ffi::OsStr;
use std::fs;
use std::path::{Iter, Path, PathBuf};

use iced::Command;
use iced::futures::channel::mpsc::Sender;

use crate::notify;

#[derive(Debug, Clone)]
pub struct Explorer {
    pub files: Node,
    pub root_path: PathBuf,

    watcher: Option<Sender<notify::Message>>,
}


#[derive(Debug, Clone)]
pub enum Message {
    Watcher(notify::Message),
}

#[derive(Debug, Clone)]
pub struct Dir {
    pub path: PathBuf,
    pub is_expanded: bool,
    pub content: Vec<Node>,

    pub name: String,
    pub name_cached: String,
    pub is_name_is_edited: bool,

    has_been_expanded: bool,
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
            has_been_expanded: false,
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
            is_name_is_edited: false,
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

    pub fn to_dir(&self) -> Result<&Dir, String> {
        match self {
            Node::Dir(dir) => Ok(dir),
            Node::File(f) => Err(format!("this is a file: {}", f.path.to_string_lossy())),
        }
    }

    pub fn to_file(&self) -> Result<&File, String> {
        match self {
            Node::Dir(d) => Err(format!("this is a directory: {}", d.path.to_string_lossy())),
            Node::File(file) => Ok(file),
        }
    }
}

impl Explorer {
    /// Construct a node of type Dir from a path
    ///
    /// Condition: root_path is a dir
    pub fn new(path: PathBuf) -> Result<Self, String> {
        is_dir_exist(&path)?;

        let dir_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => {
                if (path.to_string_lossy() == "/") {
                    "".to_string()
                } else {
                    return Err(format!(
                        "can't read the name of the path {}",
                        path.to_string_lossy()
                    ));
                }
            }
        };

        let mut content = Vec::new();

        if let Err(e) = fill_dir_content(&mut content, &path) {
            println!("{e}");
            return Err(e);
        }

        Ok(Explorer {
            files: Node::Dir(Dir {
                path: path.clone(),
                is_expanded: true,
                content,

                name: dir_name,
                has_been_expanded: true,
                ..Default::default()
            }),
            root_path: path,
            watcher: None,
        })
    }



    pub fn expand_dir(&mut self, path: PathBuf) -> Result<(), String> {
       
        let node = search_node_by_path(&mut self.files, path, true).unwrap();


        if let Node::Dir(dir) = node {
            dir.is_expanded = !dir.is_expanded;

        if dir.has_been_expanded {
            Ok(())
        } else {
            let res = fill_dir_content(&mut dir.content, &dir.path.clone());

            if let Some(ref mut watcher) = self.watcher {
                let msg_to_send = notify::Message::Watch(dir.path.clone());

                watcher
                    .try_send(msg_to_send)
                    .expect("error tring to send to watcher");
            }
            dir.has_been_expanded = true;
            res
        }
        } else {
            panic!("not a dir when expand");
        }
        
    }




    pub fn handle_message(&mut self, message: Message) -> Command<Message> {


        match message {
            Message::Watcher(msg) => {
                match msg {
                    notify::Message::Waiting(mut watcher) => {
                        let msg_to_send = notify::Message::Watch(self.root_path.clone());
                        watcher.try_send(msg_to_send)
                            .expect("error tring to send to watcher");
                      
                        self.watcher = Some(watcher);
                    },
                    notify::Message::Event(_) => todo!(),


                    _ => panic!("should never happend")
                }
            }
        }
        
        Command::none()
    }
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
                    println!(
                        "spécial file or dir have been passed: {}",
                        entry_path.display()
                    );
                    continue;
                }

                let node = if entry_path.is_dir() {
                    let entry_name = match entry_path.clone().file_name() {
                        Some(name) => name.to_string_lossy().to_string(),
                        None => {
                            return Err(format!(
                                "can't read the name of the path {}",
                                entry_path.to_string_lossy()
                            ));
                        }
                    };

                    Node::Dir(Dir {
                        path: entry_path,
                        name: entry_name,
                        ..Default::default()
                    })
                } else {
                    let entry_extension = entry_path
                        .clone()
                        .extension()
                        .unwrap_or(OsStr::new(""))
                        .to_string_lossy()
                        .to_string();

                    let entry_name = match entry_path.clone().file_name() {
                        Some(name) => name.to_string_lossy().to_string(),
                        None => {
                            return Err(format!(
                                "can't read the name of the path {}",
                                entry_path.to_string_lossy()
                            ));
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
                    Ok(index) => {
                        return Err(format!(
                            "can't insert {} in {}. name already exist in content",
                            node.name(),
                            node.path().to_string_lossy()
                        ))
                    }
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

    Ok(())
}

// Vérifie si le chemin est un répertoire existant
pub fn is_dir_exist(path: &PathBuf) -> Result<(), String> {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(())
            } else {
                Err(format!(
                    "error: {} n'est pas un répertoire.",
                    path.display()
                ))
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
    content.binary_search_by_key(&(!is_dir, name.to_lowercase(), name), |n| {
        (!n.is_dir(), n.name().to_lowercase(), n.name())
    })
}

pub fn search_node_by_path(
    root_node: &mut Node,
    search_path: PathBuf,
    is_dir: bool,
) -> Result<&mut Node, String> {
    if !root_node.is_dir() {
        return Err(format!(
            "tring to search a node: {} with a file",
            search_path.to_string_lossy()
        ));
    }

    let root_dir_path = root_node.path();

    let search_path_count = search_path.components().count();
    let mut iter_index = 0;
    let mut search_path_iter = search_path.iter();

    // make sure /a/b/c/d is in /a/b/c
    for root_part in root_dir_path.iter() {
        let search_path_part = match search_path_iter.next() {
            Some(path) => path,
            None => {
                return Err(format!(
                    "path: {} is not contain in root path {}",
                    search_path.to_string_lossy(),
                    root_dir_path.to_string_lossy()
                ))
            }
        };
        iter_index += 1;

        if (root_part != search_path_part) {
            return Err(format!(
                "path: {} is not contain in root path {}",
                search_path.to_string_lossy(),
                root_dir_path.to_string_lossy()
            ));
        }
    }

    let mut current_node = root_node;

    while let Some(search_path_part) = search_path_iter.next() {
        iter_index += 1;

        match current_node {
            Node::Dir(dir) => {
                let current_search_path_is_dir = if is_dir {
                    true
                } else {
                    iter_index != search_path_count
                };

                let node_index = match get_index_sorted(
                    search_path_part.to_string_lossy().to_string(),
                    current_search_path_is_dir,
                    &dir.content,
                ) {
                    Ok(index) => index,
                    Err(index) => {
                        return Err(format!(
                            "{} was not found in content of {}",
                            search_path_part.to_string_lossy(),
                            dir.path.to_string_lossy()
                        ))
                    }
                };

                current_node = &mut dir.content[node_index];
            }
            Node::File(file) => {
                if search_path_part.to_string_lossy() == file.name {
                    if search_path_iter.next().is_none() {
                        return Ok(current_node);
                    } else {
                        return Err(format!(
                            "{} is a file, but it's not the end off the path",
                            search_path_part.to_string_lossy()
                        ));
                    }
                } else {
                    return Err(format!(
                        "file {} is not equal to this file {} in {}",
                        search_path_part.to_string_lossy(),
                        file.name,
                        file.path.to_string_lossy()
                    ));
                }
            }
        };
    }

    // always a dir in this case
    Ok(current_node)
}
