#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::env;
use std::path;
use std::path::{Path, PathBuf};

use iced::futures::channel::mpsc::Sender;
use iced::widget::Space;
use iced::widget::{Column, Row};
use iced::Element;
use iced::{executor, Subscription};
use iced::{Application, Command};

use crate::explorer::file_struct::{Dir, Explorer, File, Node, PathId};
use crate::explorer::{file_struct, notify};
use crate::helpers::fs;
use crate::tabs::tab::{self, Tab};
use crate::top_bar::actions::{self, Actions};
use crate::widgets::tree::{self, Tree};

pub struct Notes {
    pub actions: Actions,
    pub dirs_tree: Tree,
    pub tab: Tab,

    pub explorer: Option<Explorer>,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    Loaded(Result<Explorer, String>),

    Explorer(file_struct::XplMsg),
    Actions(actions::ActMsg),
    DirsTree(tree::TreeMsg),
    Tab(tab::TabMsg),
}

impl Application for Notes {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let app = Notes {
            actions: Actions::new(),
            dirs_tree: Tree::new(),
            tab: Tab::new(),
            explorer: None,
        };

        let command = match fs::get_absolute(env::args().nth(1).map(PathBuf::from)) {
            Some(path_id) => {
                if path_id.is_dir {
                    Command::perform(load(path_id.path), AppMsg::Loaded)
                } else {
                    println!("todo: open file");
                    Command::none()
                }
            }
            None => Command::none(),
        };

        (app, command)
    }

    fn title(&self) -> String {
        String::from("Notes")
    }

    fn update(&mut self, message: AppMsg) -> Command<Self::Message> {
        match message {
            AppMsg::Explorer(msg) => {
                if let Some(ref mut explorer) = self.explorer {
                    match explorer.handle_message(msg) {
                        Some(res) => {
                            match res {
                                file_struct::XplResult::RootHasBeenRemoved => {
                                    self.explorer = None
                                },
                            }
                        },
                        None => {},
                    }
                }
            }
            AppMsg::Loaded(res) => match res {
                Ok(explorer) => {
                    self.explorer = Some(explorer);
                }
                Err(error) => {
                    println!("{error}");
                }
            },

            AppMsg::Actions(msg) => return self.actions.update(msg),
            AppMsg::DirsTree(msg) => return self.dirs_tree.update(msg, &mut self.explorer),
            AppMsg::Tab(msg) => return self.tab.update(msg),
        }
        Command::none()
    }

    fn view(&self) -> Element<AppMsg> {
        Column::new()
            .push(Space::new(0, 5))
            .push(self.actions.view())
            .push(
                Row::new()
                    .push(self.dirs_tree.view(&self.explorer, false))
                    .push(self.tab.view(self)),
            )
            .into()
    }

    fn subscription(&self) -> Subscription<AppMsg> {
        // todo: when we start the app without a path, we will never handle the Waiting call
        notify::start_watcher().map(|msg| AppMsg::Explorer(file_struct::XplMsg::Watcher(msg)))
    }
}

async fn load(path: PathBuf) -> Result<Explorer, String> {
    Explorer::new(path)
}
