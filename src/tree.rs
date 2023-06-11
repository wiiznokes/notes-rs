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
use crate::explorer::{Dir, EditName, EntryType, Explorer, File, Node, search_node_by_path, XplMsg};
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
            Some(explorer) => self.view_tree(explorer.files.to_dir().unwrap()),
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


    fn view_tree<'a>(&'a self, racine: &'a Dir) -> Element<AppMsg> {
        let mut lines: Vec<Element<AppMsg>> = Vec::new();

        lines.push(Tree::dir_line(racine, 0f32));

        if racine.is_expanded {
            self.view_tree_rec(racine, &mut lines, self.indent_space);
        }


        Scrollable::new(column(lines))
            .into()
    }


    fn view_tree_rec<'a>(&'a self, racine: &'a Dir, lines: &mut Vec<Element<'a, AppMsg>>, indent: f32) {
        for node in racine.content.iter() {
            match node {
                Node::Dir(dir) => {
                    lines.push(Tree::dir_line(dir, indent));

                    if dir.is_expanded {
                        self.view_tree_rec(dir, lines, indent + self.indent_space);
                    }
                }

                Node::File(file) => {
                    lines.push(Tree::file_line(file, indent));
                }
            }
        }
    }


    fn file_line(file: &File, indent: f32) -> Element<AppMsg> {
        let icon = icons::file_icon().size(23);

        let name: Element<AppMsg> = if file.is_name_is_edited {
            TextInput::new("file name", &file.name_cached)
                .width(Length::Fill)
                .size(15)
                .on_input(|value| {
                    AppMsg::Explorer(XplMsg::EditName(EditName::InputChanged(file.path.clone(), EntryType::File, value)))
                })
                .on_submit(AppMsg::Explorer(XplMsg::EditName(EditName::Stop(file.path.clone(), EntryType::File))))
                .into()
        } else {
            Text::new(&file.name).width(Length::Fill).size(15).into()
        };



        context_menu(icon.into(), name.into(), file.path.clone(), EntryType::File, indent).into()

    }


    fn dir_line(dir: &Dir, indent: f32) -> Element<AppMsg> {

        let icon = if dir.is_expanded {
            Button::new(icons::chevron_down_icon())
                .on_press(AppMsg::Explorer(XplMsg::Expand(dir.path.clone())))
        } else {
            Button::new(icons::chevron_right_icon())
                .on_press(AppMsg::Explorer(XplMsg::Expand(dir.path.clone())))
        };

        let name: Element<AppMsg> = if dir.is_name_is_edited {
            TextInput::new("dir name", &dir.name_cached)
                .width(Length::Fill)
                .size(15)
                .on_input(|value| {
                    AppMsg::Explorer(XplMsg::EditName(EditName::InputChanged(dir.path.clone(), EntryType::Dir, value)))
                })
                .on_submit(AppMsg::Explorer(XplMsg::EditName(EditName::Stop(dir.path.clone(), EntryType::Dir))))
                .into()
        } else {
            Text::new(&dir.name).width(Length::Fill).size(15).into()
        };


        context_menu(icon.into(), name.into(), dir.path.clone(), EntryType::File, indent).into()

    }

}


fn context_menu<'a>(icon: Element<'a, AppMsg>, name: Element<'a, AppMsg>, path: PathBuf, entry_type: EntryType, indent: f32) ->  Element<'a, AppMsg> {

    let underlay = Row::new()
        .push(Space::new(Length::Fixed(indent), 0))
        .push(icon)
        .push(Space::new(Length::Fixed(10f32), 0))
        .push(name);


    ContextMenu::new(underlay, move ||
        column(vec![
            Button::new(Text::new("New File")).on_press(
                AppMsg::Explorer(XplMsg::New(path.clone(), EntryType::File))).into(),
            Button::new(Text::new("New Dir")).on_press(
                AppMsg::Explorer(XplMsg::New(path.clone(), EntryType::Dir))).into(),
            Button::new(Text::new("Cut")).on_press(
                AppMsg::Explorer(XplMsg::Cut(path.clone(), entry_type))).into(),
            Button::new(Text::new("Copy")).on_press(
                AppMsg::Explorer(XplMsg::Copy(path.clone(), entry_type))).into(),
            Button::new(Text::new("Paste")).on_press(
                AppMsg::Explorer(XplMsg::Paste(path.clone(), entry_type))).into(),
            Button::new(Text::new("Rename")).on_press(
                AppMsg::Explorer(XplMsg::EditName(EditName::Start(path.clone(), entry_type)))).into(),
        ]).into()
    ).into()
}









