#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::cell::RefCell;
use std::env;
use std::path;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use iced::futures::channel::mpsc::Sender;
use iced::widget::Space;
use iced::widget::Text;
use iced::widget::{Column, Row};
use iced::Element;
use iced::{executor, Subscription};
use iced::{Application, Command};

use crate::explorer::file_struct::XplMsg;
use crate::explorer::file_struct::{Dir, Explorer, File, Node, PathId};
use crate::explorer::notify::NtfMsg;
use crate::explorer::{file_struct, notify};
use crate::helpers::fs;
use crate::tabs::tab::{self, Tab};
use crate::top_bar::actions::{self, Actions};
use crate::widgets::tree::{self, Tree};

pub enum State {
    Waiting,
    Ready(Notes),
}

pub struct Notes {
    pub actions: Actions,
    pub dirs_tree: Tree,
    pub tab: Tab,

    pub explorer: Option<Explorer>,
    pub watcher: Rc<RefCell<Sender<notify::NtfMsg>>>,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    Explorer(file_struct::XplMsg),
    Actions(actions::ActMsg),
    DirsTree(tree::TreeMsg),
    Tab(tab::TabMsg),
}

impl Application for State {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (State::Waiting, Command::none())
    }

    fn title(&self) -> String {
        String::from("Notes")
    }

    fn update(&mut self, message: AppMsg) -> Command<Self::Message> {
        match self {
            State::Waiting => {
                if let AppMsg::Explorer(XplMsg::Watcher(notify::NtfMsg::Waiting(watcher))) = message
                {
                    let watcher_ref_cell = RefCell::new(watcher);
                    let watcher_rc = Rc::new(watcher_ref_cell);

                    let explorer = if let Some(path_id) =
                        fs::get_absolute(env::args().nth(1).map(PathBuf::from))
                    {
                        if path_id.is_dir {
                            Some(Explorer::new(path_id.path, Rc::clone(&watcher_rc)).unwrap())
                        } else {
                            println!("todo: open file");
                            None
                        }
                    } else {
                        None
                    };

                    *self = State::Ready(Notes {
                        actions: Actions::new(),
                        dirs_tree: Tree::new(),
                        tab: Tab::new(),
                        explorer,
                        watcher: watcher_rc,
                    });
                }
            }
            State::Ready(notes) => match message {
                AppMsg::Explorer(msg) => {
                    if let Some(ref mut explorer) = notes.explorer {
                        if let Some(res) = explorer.handle_message(msg) {
                            match res {
                                file_struct::XplResult::RootHasBeenRemoved => notes.explorer = None,
                            }
                        }
                    }
                }

                AppMsg::Actions(msg) => return notes.actions.update(msg),
                AppMsg::DirsTree(msg) => return notes.dirs_tree.update(msg, &mut notes.explorer),
                AppMsg::Tab(msg) => return notes.tab.update(msg),
            },
        };

        Command::none()
    }

    fn view(&self) -> Element<AppMsg> {
        match self {
            State::Waiting => Text::new("loading...").into(),
            State::Ready(notes) => Column::new()
                .push(Space::new(0, 5))
                .push(notes.actions.view())
                .push(
                    Row::new()
                        .push(notes.dirs_tree.view(&notes.explorer, false))
                        .push(notes.tab.view(notes)),
                )
                .into(),
        }
    }

    fn subscription(&self) -> Subscription<AppMsg> {
        // todo: when we start the app without a path, we will never handle the Waiting call
        notify::start_watcher().map(|msg| AppMsg::Explorer(file_struct::XplMsg::Watcher(msg)))
    }
}
