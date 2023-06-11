#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use iced::{Element, Length};
use iced::{alignment, Application, Color, theme};
use iced::Command;
use iced::widget::{Button, Row, Text};
use iced::widget::{
    button, checkbox, container, horizontal_space, pick_list, row, slider, svg, text, text_input,
    toggler, vertical_slider,
};
use iced::widget::column as col;
use iced::widget::Space;
use iced_aw::menu::{CloseCondition, ItemHeight, ItemWidth, MenuBar, MenuTree, PathHighlight};
use iced_aw::quad;
use iced_aw::style::menu_bar;

use crate::app;
use crate::button::labeled_button;

//use iced_aw::{helpers::menu_tree, menu_bar, menu_tree};


#[derive(Clone, Debug)]
pub struct Actions {
    tt: i32,
}

#[derive(Clone, Debug)]
pub enum Message {
    Toggle,
    Search,

    NewFile,
    OpenFile,
    OpenFolder,
    Settings,
    Quit,

    Push,
    Fetch,

    None,
}

impl Actions {
    pub fn new() -> Actions {
        Actions { tt: 0 }
    }

    pub fn update(&mut self, message: Message) -> Command<app::Message> {
        match message {
            _ => { println!("{:?}", message); }
        }

        Command::none()
    }

    pub fn view(&self) -> Element<app::Message> {
        let right_menu_trees = vec![
            toggle_menu(),
            search_menu(),
            files_menu(),
        ];

        let left_menu_trees = vec![
            fetch_menu(),
            push_menu(),
        ];


        row!(
            new_menu_bar(right_menu_trees),
            horizontal_space(Length::Fill),
            new_menu_bar(left_menu_trees),
        )
            .padding([2, 8])
            .align_items(alignment::Alignment::Center)
            .into()

    }
}


fn new_menu_bar(menu_trees: Vec<MenuTree<app::Message, iced::Renderer>>) -> MenuBar<app::Message, iced::Renderer> {
    MenuBar::new(menu_trees)
        .item_width(ItemWidth::Uniform(180))
        .item_height(ItemHeight::Uniform(25))
        .spacing(4.0)
        .bounds_expand(30)
        .path_highlight(Some(PathHighlight::MenuActive))
        .close_condition(CloseCondition {
            leave: false,
            click_outside: true,
            click_inside: true,
        })
}

fn toggle_menu<'a>() -> MenuTree<'a, app::Message, iced::Renderer> {
    let main_button = labeled_button("Toggle", app::Message::Actions(Message::Toggle));
    MenuTree::new(main_button)
}

fn search_menu<'a>() -> MenuTree<'a, app::Message, iced::Renderer> {
    let main_button = labeled_button("Search", app::Message::Actions(Message::Search));
    MenuTree::new(main_button)
}


fn files_menu<'a>() -> MenuTree<'a, app::Message, iced::Renderer> {
    let main_button = labeled_button("Files", app::Message::Actions(Message::None));

    let children = vec![
        MenuTree::new(labeled_button("New File", app::Message::Actions(Message::NewFile))),
        MenuTree::new(labeled_button("Open File", app::Message::Actions(Message::OpenFile))),
        MenuTree::new(labeled_button("Open Folder", app::Message::Actions(Message::OpenFolder))),
        MenuTree::new(labeled_button("Settings", app::Message::Actions(Message::Settings))),
        MenuTree::new(labeled_button("Quit", app::Message::Actions(Message::Quit))),
    ];

    MenuTree::with_children(main_button, children)
}


fn fetch_menu<'a>() -> MenuTree<'a, app::Message, iced::Renderer> {
    let main_button = labeled_button("Fetch", app::Message::Actions(Message::Fetch));
    MenuTree::new(main_button)
}


fn push_menu<'a>() -> MenuTree<'a, app::Message, iced::Renderer> {
    let main_button = labeled_button("Push", app::Message::Actions(Message::Push));
    MenuTree::new(main_button)
}


