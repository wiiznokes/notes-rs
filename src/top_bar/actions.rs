#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use iced::widget::column as col;
use iced::widget::Space;
use iced::widget::{
    button, checkbox, container, horizontal_space, pick_list, row, slider, svg, text, text_input,
    toggler, vertical_slider,
};
use iced::widget::{Button, Row, Text};
use iced::Command;
use iced::{alignment, theme, Application, Color};
use iced::{Element, Length};
use iced_aw::menu::{CloseCondition, ItemHeight, ItemWidth, MenuBar, MenuTree, PathHighlight};
use iced_aw::quad;
use iced_aw::style::menu_bar;

use crate::app;
use crate::helpers::button::labeled_button;

//use iced_aw::{helpers::menu_tree, menu_bar, menu_tree};

#[derive(Clone, Debug)]
pub struct Actions {
    tt: i32,
}

#[derive(Clone, Debug)]
pub enum ActMsg {
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

    pub fn update(&mut self, message: ActMsg) -> Command<app::AppMsg> {
        {
            println!("{:?}", message);
        }

        Command::none()
    }

    pub fn view(&self) -> Element<app::AppMsg> {
        let right_menu_trees = vec![toggle_menu(), search_menu(), files_menu()];

        let left_menu_trees = vec![fetch_menu(), push_menu()];

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

fn new_menu_bar(
    menu_trees: Vec<MenuTree<app::AppMsg, iced::Renderer>>,
) -> MenuBar<app::AppMsg, iced::Renderer> {
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

fn toggle_menu<'a>() -> MenuTree<'a, app::AppMsg, iced::Renderer> {
    let main_button = labeled_button("Toggle", app::AppMsg::Actions(ActMsg::Toggle));
    MenuTree::new(main_button)
}

fn search_menu<'a>() -> MenuTree<'a, app::AppMsg, iced::Renderer> {
    let main_button = labeled_button("Search", app::AppMsg::Actions(ActMsg::Search));
    MenuTree::new(main_button)
}

fn files_menu<'a>() -> MenuTree<'a, app::AppMsg, iced::Renderer> {
    let main_button = labeled_button("Files", app::AppMsg::Actions(ActMsg::None));

    let children = vec![
        MenuTree::new(labeled_button(
            "New File",
            app::AppMsg::Actions(ActMsg::NewFile),
        )),
        MenuTree::new(labeled_button(
            "Open File",
            app::AppMsg::Actions(ActMsg::OpenFile),
        )),
        MenuTree::new(labeled_button(
            "Open Folder",
            app::AppMsg::Actions(ActMsg::OpenFolder),
        )),
        MenuTree::new(labeled_button(
            "Settings",
            app::AppMsg::Actions(ActMsg::Settings),
        )),
        MenuTree::new(labeled_button("Quit", app::AppMsg::Actions(ActMsg::Quit))),
    ];

    MenuTree::with_children(main_button, children)
}

fn fetch_menu<'a>() -> MenuTree<'a, app::AppMsg, iced::Renderer> {
    let main_button = labeled_button("Fetch", app::AppMsg::Actions(ActMsg::Fetch));
    MenuTree::new(main_button)
}

fn push_menu<'a>() -> MenuTree<'a, app::AppMsg, iced::Renderer> {
    let main_button = labeled_button("Push", app::AppMsg::Actions(ActMsg::Push));
    MenuTree::new(main_button)
}
