#![allow(dead_code)]

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use iced::futures::channel::mpsc::Sender;
use iced::widget::scrollable::Properties;
use iced::widget::{
    column, row, Button, Column, Container, Row, Scrollable, Space, Text, TextInput,
};
use iced::Command;
use iced::{Element, Length};
use iced_aw::ContextMenu;

use crate::app::{self, AppMsg};
use crate::explorer::file_struct::{
    ActionType, Dir, EditNameType, Explorer, File, Node, PathId, XplImplReqMsg, XplMsg,
};
use crate::explorer::notify;
use crate::{explorer::file_struct, helpers::icons};

pub struct Tree {
    indent_space: f32,
}

#[derive(Clone, Debug)]
pub enum TreeMsg {
    Open,
    Xpl(XplImplReqMsg),
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            indent_space: 15f32,
        }
    }

    pub fn update(
        &mut self,
        message: TreeMsg,
        explorer_opt: &mut Option<Explorer>,
    ) -> Command<AppMsg> {
        match message {
            TreeMsg::Open => {}
            TreeMsg::Xpl(msg) => match msg {
                XplImplReqMsg::None => {}
                XplImplReqMsg::RootHasBeenRemoved => *explorer_opt = None,
            },
        }
        Command::none()
    }

    pub fn view<'a>(
        &'a self,
        explorer_opt: &'a Option<Explorer>,
        show_root_line: bool,
    ) -> Element<AppMsg> {
        let tree = match explorer_opt {
            Some(explorer) => match &explorer.files {
                Node::Dir(com, dir) => {
                    let mut lines: Vec<Element<AppMsg>> = Vec::new();

                    let indent_space = match show_root_line {
                        true => {
                            lines.push(view_line(&explorer.files, 0f32));
                            self.indent_space
                        }
                        false => 0f32,
                    };

                    if dir.is_expanded {
                        self.view_tree_rec(dir, &mut lines, indent_space);
                    }

                    Scrollable::new(column(lines)).into()
                }
                Node::File(..) => no_tree(),
            },
            _ => no_tree(),
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

    fn view_tree_rec<'a>(
        &'a self,
        racine: &'a Dir,
        lines: &mut Vec<Element<'a, AppMsg>>,
        indent: f32,
    ) {
        for node in racine.content.iter() {
            lines.push(view_line(node, indent));

            if let Node::Dir(_, dir) = node {
                if dir.is_expanded {
                    self.view_tree_rec(dir, lines, indent + self.indent_space);
                }
            }
        }
    }
}

fn no_tree<'a>() -> Element<'a, AppMsg> {
    Text::new("nothing to show").into()
}

fn view_line(node: &Node, indent: f32) -> Element<AppMsg> {
    let icon: Element<AppMsg> = match node {
        Node::Dir(com, dir) => {
            let msg_icon = AppMsg::Explorer(XplMsg::Expand(PathId {
                path: com.path.clone(),
                is_dir: true,
            }));
            if dir.is_expanded {
                Button::new(icons::chevron_down_icon())
                    .on_press(msg_icon)
                    .into()
            } else {
                Button::new(icons::chevron_right_icon())
                    .on_press(msg_icon)
                    .into()
            }
        }

        Node::File(..) => icons::file_icon().size(23).into(),
    };

    let name: Element<AppMsg> = if node.common().is_name_is_edited {
        TextInput::new("dir name", &node.common().name_cached)
            .width(Length::Fill)
            .size(15)
            .on_input(|value| {
                AppMsg::Explorer(XplMsg::EditName(
                    node.path_id(),
                    EditNameType::InputChanged(value),
                ))
            })
            .on_submit(AppMsg::Explorer(XplMsg::EditName(
                node.path_id(),
                EditNameType::Stop(ActionType::Ok),
            )))
            .into()
    } else {
        Text::new(&node.common().name)
            .width(Length::Fill)
            .size(15)
            .into()
    };

    let underlay = Row::new()
        .push(Space::new(Length::Fixed(indent), 0))
        .push(icon)
        .push(Space::new(Length::Fixed(10f32), 0))
        .push(name);

    ContextMenu::new(underlay, move || {
        column(vec![
            Button::new(Text::new("New File"))
                .on_press(AppMsg::Explorer(XplMsg::New(node.path_id())))
                .into(),
            Button::new(Text::new("New Dir"))
                .on_press(AppMsg::Explorer(XplMsg::New(node.path_id())))
                .into(),
            Button::new(Text::new("Cut"))
                .on_press(AppMsg::Explorer(XplMsg::Cut(node.path_id())))
                .into(),
            Button::new(Text::new("Copy"))
                .on_press(AppMsg::Explorer(XplMsg::Copy(node.path_id())))
                .into(),
            Button::new(Text::new("Paste"))
                .on_press(AppMsg::Explorer(XplMsg::Paste(node.path_id())))
                .into(),
            Button::new(Text::new("Rename"))
                .on_press(AppMsg::Explorer(XplMsg::EditName(
                    node.path_id(),
                    EditNameType::Start,
                )))
                .into(),
            Button::new(Text::new("Delete"))
                .on_press(AppMsg::Explorer(XplMsg::Delete(node.path_id())))
                .into(),
        ])
        .into()
    })
    .into()
}
