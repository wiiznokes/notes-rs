#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]



use std::path::PathBuf;

use iced::Command;
use iced::{Element, Length};

use crate::icons;

use iced::widget::{row, Button, Column, Container, Row, Space, Text, TextInput};

use crate::app::{self};

use crate::file_system::{get_node, DirNode, FileNode, Node};

#[derive(Clone, Debug)]
pub struct DirsTree {}

#[derive(Clone, Debug)]
pub enum Message {
    Open,
    Move,
    Remove,

    InputChanged(String, PathBuf),
    Rename,
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
        files: &mut Option<Node>,
    ) -> iced::Command<app::Message> {


        match message {
            Message::InputChanged(value, path) => match files {

                Some(dir) => {
                    let node = get_node(dir, path);


                    match node {
                        Some(Node::Dir(ref mut dir)) => dir.full_name_cached = value,

                        Some(Node::File(ref mut file)) => file.full_name_cached = value,
                        _ => { }
                    }
                }

                _ => { }
            },

            _ => { todo!() } 
        }

        Command::none()
    }

    pub fn view(&self, files: &Option<Node>) -> Element<app::Message> {
        let tree = match files {
            Some(Node::Dir(dir)) => view_tree(dir, 0f32),
            _ => Text::new("nothing to show").into(),
        };

        let content = Container::new(tree)
            .height(Length::Fill)
            .style(iced::theme::Container::Box);

        Container::new(content)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}

/// construit une Column
/// n'affiche pas le dirNode name
/// itère sur tout le content
/// si dir expand -> call recursive avec indent + 4 (ajout res to Col)
/// si file ajout row
/// return des columns imbriquées
fn view_tree(racine: &DirNode, indent: f32) -> Element<app::Message> {
    let col = Column::new();

    for node in &racine.content {
        match node {
            Node::Dir(dir) => {
                let icon = if (dir.is_expand) {
                    Button::new(icons::chevron_down_icon())
                } else {
                    Button::new(icons::chevron_right_icon())
                };

                col.push(
                    Row::new()
                        .push(Space::new(Length::Fixed(indent), 0))
                        .push(icon)
                        .push(
                            TextInput::new("placeholder", &dir.full_name_cached)
                                .width(Length::Shrink)
                                .size(15)
                                .on_input(|value| {
                                    app::Message::DirsTree(Message::InputChanged(
                                        value,
                                        node.path(),
                                    ))
                                }),
                        ),
                );

                if dir.is_expand {
                    let new_indent = indent + 4f32;
                    col.push(view_tree(&dir, new_indent));
                }
            }

            Node::File(file) => {
                col.push(
                    Row::new().push(Button::new(icons::file_icon())).push(
                        TextInput::new("placeholder", &file.full_name_cached)
                            .width(Length::Shrink)
                            .size(15)
                            .on_input(|value| {
                                app::Message::DirsTree(Message::InputChanged(value, node.path()))
                            }),
                    ),
                );
            }
        }
    }

    col.into()
}
