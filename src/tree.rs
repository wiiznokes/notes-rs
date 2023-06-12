#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use iced::{Element, Length};
use iced::Command;
use iced::futures::channel::mpsc::Sender;
use iced::widget::{Button, column, Column, Container, row, Row, Scrollable, Space, Text, TextInput};
use iced::widget::scrollable::Properties;
use iced_aw::ContextMenu;

use crate::app::{self, AppMsg};
use crate::explorer::{ActionType, Dir, Explorer, File, Node, search_node_by_path, XplMsg, PathId, EditNameType};
use crate::{explorer, icons};
use crate::notify;

pub struct Tree {
    indent_space: f32,

}

#[derive(Clone, Debug)]
pub enum TreeMsg {
    Open,
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
        match explorer_opt {
            Some(explorer) => match message {
                TreeMsg::Open => {  }
            },

            _ => { panic!("no root_node, {:?}", message); }
        }
        Command::none()
    }


    pub fn view<'a>(&'a self, explorer_opt: &'a Option<Explorer>) -> Element<AppMsg> {
        let tree = match explorer_opt {
            Some(explorer) => self.view_tree(&explorer.files, false),
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


    fn view_tree<'a>(&'a self, racine: &'a Node, show_root_line: bool) -> Element<AppMsg> {
        let mut lines: Vec<Element<AppMsg>> = Vec::new();

        let indent_space = match show_root_line {
            true => {
                lines.push(view_line(&racine, 0f32));
                self.indent_space
            },
            false => {
                0f32
            },
        };

        let dir = racine.to_dir().unwrap();
        if dir.is_expanded {
            self.view_tree_rec(dir, &mut lines, indent_space);
        }


        Scrollable::new(column(lines))
            .into()
    }


    fn view_tree_rec<'a>(&'a self, racine: &'a Dir, lines: &mut Vec<Element<'a, AppMsg>>, indent: f32) {
        for node in racine.content.iter() {
            lines.push(view_line(node, indent));

            if let Node::Dir(dir) = node {
                if dir.is_expanded {
                    self.view_tree_rec(dir, lines, indent + self.indent_space);
                }
            }
        }
    }

}

fn view_line<'a>(node: &'a Node, indent: f32) -> Element<'a, AppMsg> {

    let icon: Element<AppMsg> = match node {
        Node::Dir(dir) => {
            let msg_icon = AppMsg::Explorer(XplMsg::Expand(PathId { path: dir.path.clone(), is_dir: true }));
            if dir.is_expanded {
                Button::new(icons::chevron_down_icon())
                    .on_press(msg_icon).into()
            } else {
                Button::new(icons::chevron_right_icon())
                    .on_press(msg_icon).into()
            }
        }

        Node::File(_) => {
            icons::file_icon().size(23).into()
        }
    };

    let name: Element<AppMsg> = if node.is_name_is_edited() {
        TextInput::new("dir name", &node.name_cached())
            .width(Length::Fill)
            .size(15)
            .on_input(|value| {
                AppMsg::Explorer(XplMsg::EditName(node.path_id(), EditNameType::InputChanged(value)))
            })
            .on_submit(AppMsg::Explorer(XplMsg::EditName(node.path_id(), EditNameType::Stop(ActionType::Ok))))
            .into()
    } else {
        Text::new(node.name()).width(Length::Fill).size(15).into()
    };

    

    let underlay = Row::new()
        .push(Space::new(Length::Fixed(indent), 0))
        .push(icon)
        .push(Space::new(Length::Fixed(10f32), 0))
        .push(name);

    
    ContextMenu::new(underlay, move ||
        column(vec![
            Button::new(Text::new("New File")).on_press(
                AppMsg::Explorer(XplMsg::New(node.path_id()))).into(),
            Button::new(Text::new("New Dir")).on_press(
                AppMsg::Explorer(XplMsg::New(node.path_id()))).into(),
            Button::new(Text::new("Cut")).on_press(
                AppMsg::Explorer(XplMsg::Cut(node.path_id()))).into(),
            Button::new(Text::new("Copy")).on_press(
                AppMsg::Explorer(XplMsg::Copy(node.path_id()))).into(),
            Button::new(Text::new("Paste")).on_press(
                AppMsg::Explorer(XplMsg::Paste(node.path_id()))).into(),
            Button::new(Text::new("Rename")).on_press(
                AppMsg::Explorer(XplMsg::EditName(node.path_id(), EditNameType::Start))).into(),
        ]).into()
    ).into()

}