#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use iced::futures::channel::mpsc::Sender;

use iced::Command;
use iced::{Element, Length};

use crate::icons;

use crate::notify;

use iced::widget::{column, row, Button, Column, Container, Row, Space, Text, TextInput};

use crate::app::{self};

use crate::explorer::{search_node_by_path, Dir, File, Node, Explorer};

#[derive(Clone, Debug)]
pub struct DirsTree {}

#[derive(Clone, Debug)]
pub enum Message {
    Open,
    Move,
    Remove,

    InputChanged(PathBuf, String),
    Expand(PathBuf),

    Rename(PathBuf),
    EditName(PathBuf, bool), // false to cancel
    NewFile,
    NewDir,
    Cut,
    Copy,
    Paste,
}

impl DirsTree {
    pub fn new() -> DirsTree {
        DirsTree {}
    }

    pub fn update(
        &mut self,
        message: Message,
        explorer_opt: &mut Option<Explorer>,
    ) -> iced::Command<app::Message> {
        match explorer_opt {
            Some(explorer) => match message {
                Message::InputChanged(path, value) => {
                    /*

                    let node = search_node_by_path(root_node, path);

                    match node {
                        Some(Node::Dir(ref mut dir)) => {

                            dir.name_cached = value;
                        }

                        Some(Node::File(ref mut file)) => {
                            println!("{}", file.name_cached);

                            file.name_cached = value;
                        }
                        _ => { panic!("Aucun node trouvé"); }
                    }
                     */
                }

                Message::Expand(path) => {
                    explorer.expand_dir(path).unwrap();

                },

                Message::EditName(path, is_active_requested) => {
                    /*
                    let node = search_node_by_path(root_node, path);

                    match node {
                        Some(Node::Dir(ref mut dir)) => {

                            if (is_active_requested) {
                                dir.is_name_is_edited = true;
                            } else {
                                dir.name_cached = dir.name.clone();
                                dir.is_name_is_edited = false;
                            }
                        }

                        Some(Node::File(ref mut file)) => {
                            if (is_active_requested) {
                                file.is_name_is_edited = true;
                            } else {
                                file.name_cached = file.name.clone();
                                file.is_name_is_edited = false;
                            }
                        }
                        _ => { panic!("Aucun node trouvé"); }
                    }
                     */
                },

                Message::Rename(path) => { 
                    /*
                    // TODO: call file_system module, if sucess copy cached name to name
                    let node = search_node_by_path(root_node, path);

                    match node {
                        Some(Node::Dir(ref mut dir)) => {

                            dir.name = dir.name_cached.clone();
                            dir.is_name_is_edited = false;
                        }

                        Some(Node::File(ref mut file)) => {

                            file.name = file.name_cached.clone();
                            file.is_name_is_edited = false;

                        }
                
                        _ => { panic!("Aucun node trouvé"); }
                    }
                     */
                }

                _ => {
                    todo!()
                }
            },

            _ => {
                panic!("no root_node"); // should never happended, since there is nothing to show
            } 
        }
        Command::none()
    }

    pub fn view<'a>(&'a self, explorer_opt: &'a Option<Explorer>) -> Element<app::Message> {
        let tree = match explorer_opt {
            Some(explorer) => view_tree(explorer.files.to_dir().unwrap(), 0f32),
            _ => Text::new("nothing to show").into(),
        };

        let content = Container::new(tree)
            .height(Length::Fill)
            .width(215f32)
            .style(iced::theme::Container::Box);

        Container::new(content)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}

fn view_tree(racine: &Dir, indent: f32) -> Element<app::Message> {
    let mut view_rep: Vec<Element<app::Message>> = Vec::new();

    for node in racine.content.iter() {
        match node {
            Node::Dir(dir) => {
                let icon = if (dir.is_expanded) {
                    Button::new(icons::chevron_down_icon())
                        .on_press(app::Message::DirsTree(Message::Expand(node.path())))
                } else {
                    Button::new(icons::chevron_right_icon())
                        .on_press(app::Message::DirsTree(Message::Expand(node.path())))
                };

                let text: Element<app::Message> = if (dir.is_name_is_edited) {
                    TextInput::new("dir name", &dir.name_cached)
                                .width(Length::Fill)
                                .size(15)
                                .on_input(|value| {
                                    app::Message::DirsTree(Message::InputChanged(
                                        node.path(),
                                        value,
                                    ))
                                })
                                .on_submit(app::Message::DirsTree(Message::Rename(node.path())))
                                .into()
                } else {
                    Text::new(&dir.name)
                        .width(Length::Fill)
                        .size(15)
                        .into()
                };

                view_rep.push(
                    Row::new()
                        .push(Space::new(Length::Fixed(indent), 0))
                        .push(icon)
                        .push(space_custom(10f32))
                        .push(text)
                        .into(),
                );

                if dir.is_expanded {
                    let new_indent = indent + 15f32;
                    view_rep.push(view_tree(&dir, new_indent));
                }
            }

            Node::File(file) => {

                let icon = Button::new(icons::file_icon())
                    .on_press(app::Message::DirsTree(Message::EditName(node.path(), !file.is_name_is_edited)));

                let text: Element<app::Message> = if (file.is_name_is_edited) {
                    TextInput::new("dir name", &file.name_cached)
                                .width(Length::Fill)
                                .size(15)
                                .on_input(|value| {
                                    app::Message::DirsTree(Message::InputChanged(
                                        node.path(),
                                        value,
                                    ))
                                })
                                .on_submit(app::Message::DirsTree(Message::Rename(node.path())))
                                .into()
                } else {
                    Text::new(&file.name)
                        .width(Length::Fill)
                        .size(15)
                        .into()
                };

                view_rep.push(
                    Row::new()
                        .push(space_custom(indent))
                        .push(icon)
                        .push(space_custom(10f32))
                        .push(text)
                        .into(),
                );
            }
        }
    }

    column(view_rep).into()
}



fn space_custom(indent: f32)  -> Space {
    Space::new(Length::Fixed(indent), 0)
}