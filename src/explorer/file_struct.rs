#![allow(dead_code)]

use std::cell::RefCell;
use std::ffi::OsStr;
use std::path::{Iter, Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use ::notify::Event;
use iced::futures::channel::mpsc::Sender;
use iced::Command;
use path_clean::PathClean;

use crate::app::AppMsg;
use crate::explorer::notify;
use crate::{helpers::fs, map_err_return, map_none_return};

use super::notify::NtfMsg;

#[derive(Debug, Clone)]
pub struct Explorer {
    pub files: Node,
    pub root_path: PathBuf,

    pub watcher: Rc<RefCell<Sender<notify::NtfMsg>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    Ok,
    Cancel,
}

#[derive(Debug, Clone)]
pub enum EditNameType {
    Start,
    Stop(ActionType),
    InputChanged(String),
}

#[derive(Debug, Clone)]
pub struct PathId {
    pub path: PathBuf,
    pub is_dir: bool,
}

#[derive(Debug, Clone)]
pub enum XplMsg {
    Watcher(notify::NtfMsg),

    New(PathId),
    Cut(PathId),
    Copy(PathId),
    Paste(PathId),
    EditName(PathId, EditNameType),
    Delete(PathId),

    Expand(PathId),
}


#[derive(Debug, Clone)]
pub struct CommonNode {
    pub path: PathBuf,

    pub name: String,
    pub name_cached: String,
    pub is_name_is_edited: bool,
}

impl Default for CommonNode {
    fn default() -> Self {
        CommonNode {
            path: PathBuf::new(),
            name: String::new(),
            name_cached: String::new(),
            is_name_is_edited: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Dir {
    pub is_expanded: bool,
    has_been_expanded: bool,

    pub content: Vec<Node>,
}

#[derive(Debug, Clone, Default)]
pub struct File {
    pub extension: String,
}

#[derive(Debug, Clone)]
pub enum Node {
    Dir(CommonNode, Dir),
    File(CommonNode, File),
}

impl Node {
    pub fn is_dir(&self) -> bool {
        match &self {
            Node::Dir(..) => true,
            Node::File(..) => false,
        }
    }

    pub fn common(&self) -> &CommonNode {
        match self {
            Node::Dir(com, _) => com,
            Node::File(com, _) => com,
        }
    }

    pub fn common_mut(&mut self) -> &mut CommonNode {
        match self {
            Node::Dir(com, _) => com,
            Node::File(com, _) => com,
        }
    }

    pub fn path_id(&self) -> PathId {
        PathId {
            path: self.common().path.clone(),
            is_dir: self.is_dir(),
        }
    }

    pub fn to_dir(&self) -> Result<(&CommonNode, &Dir), String> {
        match self {
            Node::Dir(com, dir) => Ok((com, dir)),
            _ => Err("not a dir".to_string()),
        }
    }

    pub fn to_dir_mut(&mut self) -> Result<(&mut CommonNode, &mut Dir), String> {
        match self {
            Node::Dir(com, dir) => Ok((com, dir)),
            _ => Err("not a dir".to_string()),
        }
    }
}


#[derive(Debug, Clone)]
pub enum XplResult {
    RootHasBeenRemoved,
}



impl Explorer {
    /// Construct a node of type Dir from a path
    ///
    /// Condition: root_path is a dir
    pub fn new(path: PathBuf, watcher: Rc<RefCell<Sender<NtfMsg>>>) -> Result<Self, String> {
        if !fs::is_dir_exist(&path).unwrap_or(false) {
            return Err(format!(
                "path {} is not a directory",
                path.to_string_lossy()
            ));
        };

        let dir_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => {
                if path.to_string_lossy() == "/" {
                    "/".to_string()
                } else {
                    return Err(format!(
                        "can't read the name of the path {}",
                        path.to_string_lossy()
                    ));
                }
            }
        };

        let mut content = Vec::new();

        fill_dir_content(&mut content, &path);

        // may cause bug (idk)
        watcher.borrow_mut()
            .try_send(notify::NtfMsg::Watch(path.clone()))
            .expect("can't send to watcher");

        Ok(Explorer {
            files: Node::Dir(
                CommonNode {
                    path: path.clone(),
                    name: dir_name,
                    ..Default::default()
                },
                Dir {
                    is_expanded: true,
                    content,
                    has_been_expanded: true,
                },
            ),
            root_path: path,
            watcher,
        })
    }



    pub fn handle_message(&mut self, message: XplMsg) -> Option<XplResult> {
        match message {
            XplMsg::Watcher(msg) => match msg {
                notify::NtfMsg::Event(event) => return self.handle_event(event),
                _ => panic!("{:?}", msg),
            },
            XplMsg::New(_) => {}
            XplMsg::Cut(_) => {}
            XplMsg::Copy(_) => {}
            XplMsg::Paste(_) => {}
            XplMsg::EditName(path_id, edit_name) => {
                if let Err(e) = self.edit_name(path_id, edit_name) {
                    eprintln!("{}", e);
                }
            }
            XplMsg::Expand(path_id) => self.expand_dir(path_id),
            XplMsg::Delete(path_id) => {
                if let Err(e) = self.remove(path_id) {
                    eprintln!("{}", e);
                }
            }
        }

        None
    }

    fn handle_event(&mut self, event: Event) -> Option<XplResult> {
        println!("{:?}", event);

        match event.kind {
            ::notify::EventKind::Create(create_kind) => match create_kind {
                ::notify::event::CreateKind::File => {
                    let path = &event.paths[0];
                    let (com, dir) =
                        map_err_return!(search_parent_node(&mut self.files, path.clone()));

                    map_err_return!(insert_node_in_vec(&mut dir.content, path, false));
                }
                ::notify::event::CreateKind::Folder => {
                    let path = &event.paths[0];
                    let (com, dir) =
                        map_err_return!(search_parent_node(&mut self.files, path.clone()));

                    map_err_return!(insert_node_in_vec(&mut dir.content, path, true));
                }
                _ => {}
            },
            ::notify::EventKind::Modify(modify_kind) => match modify_kind {
                ::notify::event::ModifyKind::Data(_) => {}
                ::notify::event::ModifyKind::Name(rename_kind) => match rename_kind {
                    ::notify::event::RenameMode::To => {
                        let path = &event.paths[0];
                        let (com, dir) =
                            map_err_return!(search_parent_node(&mut self.files, path.clone()));

                        map_err_return!(insert_node_in_vec(&mut dir.content, path, path.is_dir()));
                    }
                    ::notify::event::RenameMode::From => {
                        let path = &event.paths[0];
                        if path == &self.root_path {
                            println!("root path has been removed");
                            return Some(XplResult::RootHasBeenRemoved);
                        }
                        
                        let (com, dir) =
                            map_err_return!(search_parent_node(&mut self.files, path.clone()));

                        let name = map_none_return!(
                            path.file_name(),
                            "can't find name {}",
                            path.display()
                        )
                        .to_string_lossy()
                        .to_string();

                        let index = map_err_return!(get_index_unknown_type(name, &dir.content));

                        dir.content.remove(index);
                    }
                    _ => {}
                },
                _ => {}
            },
            ::notify::EventKind::Remove(remove_kind) => {
                match remove_kind {
                    ::notify::event::RemoveKind::File => {
                        let path = &event.paths[0];

                        if path == &self.root_path {
                            println!("root path has been removed");
                            return Some(XplResult::RootHasBeenRemoved);
                        }

                        let (com, dir) =
                            map_err_return!(search_parent_node(&mut self.files, path.clone()));

                        let name = map_none_return!(
                            path.file_name(),
                            "can't find name {}",
                            path.display()
                        )
                        .to_string_lossy()
                        .to_string();

                        // see: https://github.com/notify-rs/notify/issues/493
                        let index = map_err_return!(get_index_sorted(name, false, &dir.content));

                        dir.content.remove(index);
                    }
                    ::notify::event::RemoveKind::Folder => {
                        let path = &event.paths[0];
                        let (com, dir) =
                            map_err_return!(search_parent_node(&mut self.files, path.clone()));

                        let name = map_none_return!(
                            path.file_name(),
                            "can't find name {}",
                            path.display()
                        )
                        .to_string_lossy()
                        .to_string();
                        let index = map_err_return!(get_index_sorted(name, true, &dir.content));

                        dir.content.remove(index);
                    }

                    _ => {}
                }
            }

            _ => {}
        }
        None
    }

    fn expand_dir(&mut self, path_id: PathId) {
        let (com, dir) = search_node_by_path(&mut self.files, path_id)
            .unwrap()
            .to_dir_mut()
            .unwrap();

        if dir.has_been_expanded {
            dir.is_expanded = !dir.is_expanded;
        } else {
            fill_dir_content(&mut dir.content, &com.path.clone());

            dir.is_expanded = !dir.is_expanded;
            dir.has_been_expanded = true;

            
            self.watcher.borrow_mut()
                .try_send(notify::NtfMsg::Watch(com.path.clone()))
                .expect("error trying to send to watcher");
        }
    }

    fn edit_name(&mut self, path_id: PathId, edit_name: EditNameType) -> Result<(), String> {
        let node = match search_node_by_path(&mut self.files, path_id) {
            Ok(node) => node,
            Err(e) => return Err(e),
        };

        match edit_name {
            EditNameType::Start => {
                let com = node.common_mut();
                com.name_cached = com.name.clone();
                com.is_name_is_edited = true;
            }
            EditNameType::Stop(action_type) => {
                node.common_mut().is_name_is_edited = false;

                let com = node.common();
                match action_type {
                    ActionType::Ok => {
                        return fs::rename(&com.path.clone(), com.name_cached.clone());
                    }

                    ActionType::Cancel => {}
                }
            }
            EditNameType::InputChanged(value) => {
                node.common_mut().name_cached = value;
            }
        }
        Ok(())
    }

    fn remove(&mut self, path_id: PathId) -> Result<(), String> {
        fs::remove(path_id)
    }
}

fn fill_dir_content(content: &mut Vec<Node>, path: &PathBuf) {
    let dir_entries = match fs::get_dir_entries(path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("can't get dir entries: {}", e);
            return;
        }
    };

    for entry_opt in dir_entries {
        match entry_opt {
            Ok(entry) => {
                let entry_path = entry.path();
                let is_dir = entry_path.is_dir();
                if !is_dir && !entry_path.is_file() {
                    eprintln!("special file have been passed: {}", entry_path.display());
                    continue;
                }

                let _ = insert_node_in_vec(content, &entry_path, is_dir)
                    .map_err(|e| eprintln!("{:?}", e));
            }
            Err(error) => {
                eprintln!(
                    "error when reading the content of {}: {}",
                    path.display(),
                    error
                );
                continue;
            }
        }
    }
}

fn insert_node_in_vec(content: &mut Vec<Node>, path: &Path, is_dir: bool) -> Result<(), String> {
    let name = match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err("no file name".to_string()),
    };

    let node = if is_dir {
        Node::Dir(
            CommonNode {
                path: path.to_path_buf(),
                name,
                ..Default::default()
            },
            Dir {
                ..Default::default()
            },
        )
    } else {
        let entry_extension = path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_string_lossy()
            .to_string();

        Node::File(
            CommonNode {
                path: path.to_path_buf(),
                name,
                ..Default::default()
            },
            File {
                extension: entry_extension,
            },
        )
    };

    match get_index_sorted(node.common().name.clone(), node.is_dir(), content) {
        Ok(index) => {
            return Err(format!(
                "can't insert {} in {}. name already exist in content",
                node.common().name.clone(),
                node.common().path.to_string_lossy()
            ));
        }
        Err(index) => content.insert(index, node),
    }
    Ok(())
}

// todo: 11 before 100

/// If the value is found then [`Ok`] is returned, containing the index of the matching element.
/// If the value is not found then [`Err`] is returned,
/// containing the index where a matching element could be inserted while maintaining sorted order
///
/// Sorting follow this rules:
/// - all directory before files
/// - alpha numeric (ASCII), with case insensitive (a = A)
///
/// Condition: content must be sorted with this rules before using this function
fn get_index_sorted(name: String, is_dir: bool, content: &[Node]) -> Result<usize, usize> {
    // notice we use negation when node is a dir
    // because 0 will have a smaller index than 1
    //
    // we lower all letter because 'A' < '_' < 'a' in ASCII, and
    // I prefer having '.' and '_' files on top

    // we use a third key in case of equality, because Linux is sensitive (a != A)
    content.binary_search_by_key(&(!is_dir, name.to_lowercase(), name), |n| {
        (
            !n.is_dir(),
            n.common().name.to_lowercase(),
            n.common().clone().name,
        )
    })
}

/// if a node with the same name is found, return this index, else return [`Err`]
fn get_index_unknown_type(name: String, content: &[Node]) -> Result<usize, String> {
    match get_index_sorted(name.clone(), true, content) {
        Ok(index) => Ok(index),
        Err(_) => match get_index_sorted(name, false, content) {
            Ok(index) => Ok(index),
            Err(_) => Err("node index not found".to_string()),
        },
    }
}

pub fn search_parent_node(
    root_node: &mut Node,
    path: PathBuf,
) -> Result<(&mut CommonNode, &mut Dir), String> {
    let parent_path = match path.parent() {
        Some(parent_path) => parent_path,
        None => return Err("no parent".to_string()),
    };

    match search_node_by_path(
        root_node,
        PathId {
            path: parent_path.into(),
            is_dir: true,
        },
    ) {
        Ok(node) => node.to_dir_mut(),
        Err(e) => Err(e),
    }
}

pub fn search_node_by_path(root_node: &mut Node, path_id: PathId) -> Result<&mut Node, String> {
    if !root_node.is_dir() {
        return Err(format!(
            "trying to search a node: {} with a file",
            path_id.path.to_string_lossy()
        ));
    }

    let root_dir_path = root_node.common().path.clone();

    let search_path_count = path_id.path.components().count();
    let mut iter_index = 0;
    let mut search_path_iter = path_id.path.iter();

    // make sure /a/b/c/d is in /a/b/c
    for root_part in root_dir_path.iter() {
        let search_path_part = match search_path_iter.next() {
            Some(path) => path,
            None => {
                return Err(format!(
                    "path: {} is not contain in root path {}",
                    path_id.path.to_string_lossy(),
                    root_dir_path.to_string_lossy()
                ));
            }
        };
        iter_index += 1;

        if root_part != search_path_part {
            return Err(format!(
                "path: {} is not contain in root path {}",
                path_id.path.to_string_lossy(),
                root_dir_path.to_string_lossy()
            ));
        }
    }

    let mut current_node = root_node;

    while let Some(search_path_part) = search_path_iter.next() {
        iter_index += 1;

        match current_node {
            Node::Dir(com, dir) => {
                let current_search_path_is_dir = if path_id.is_dir {
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
                            com.path.to_string_lossy()
                        ));
                    }
                };

                current_node = &mut dir.content[node_index];
            }
            Node::File(com, file) => {
                return if search_path_part.to_string_lossy() == com.name {
                    if search_path_iter.next().is_none() {
                        Ok(current_node)
                    } else {
                        Err(format!(
                            "{} is a file, but it's not the end off the path",
                            search_path_part.to_string_lossy()
                        ))
                    }
                } else {
                    Err(format!(
                        "file {} is not equal to this file {} in {}",
                        search_path_part.to_string_lossy(),
                        com.name,
                        com.path.to_string_lossy()
                    ))
                };
            }
        };
    }

    // always a dir in this case
    Ok(current_node)
}
