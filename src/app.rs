#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use std::env;
use std::path::{Path, PathBuf};

use iced::futures::channel::mpsc::Sender;
use iced::{executor, Subscription};
use iced::{Application, Command};

use crate::actions::{self, Actions};
use crate::dirs_tree::{self, DirsTree};
use crate::onglets::{self, Onglets};
use crate::{explorer, notify};

use iced::widget::{Column, Row};
use iced::Element;

use iced::widget::Space;

use crate::explorer::{Dir, Explorer, File, Node};

pub struct Notes {
    pub actions: Actions,
    pub dirs_tree: DirsTree,
    pub onglets: Onglets,

    pub explorer: Option<Explorer>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<Explorer, String>),

    Explorer(explorer::Message),
    Actions(actions::Message),
    DirsTree(dirs_tree::Message),
    Onglets(onglets::Message),
}

impl Application for Notes {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let mut args = env::args();
    

        let root_path = args.nth(1).map(PathBuf::from);

        let root_path_clone = root_path.clone();

        let app = Notes {
            actions: Actions::new(),
            dirs_tree: DirsTree::new(),
            onglets: Onglets::new(),
            explorer: None,
        };

        let command = if let Some(path) = root_path_clone {
            Command::perform(load(path), Message::Loaded)
        } else {
            Command::none()
        };

        (app, command)
    }

    fn title(&self) -> String {
        String::from("Notes")
    }

    fn subscription(&self) -> Subscription<Message> {
        // todo: when we start the app without a path, we will never handle the Waiting call
        notify::start_watcher().map(|msg| Message::Explorer(explorer::Message::Watcher(msg)))
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Explorer(msg) => {
                if let Some(ref mut explorer) = self.explorer {
                    return explorer.handle_message(msg).map(Message::Explorer);
                }
            }
            Message::Loaded(res) => match res {
                Ok(explorer) => {
                    self.explorer = Some(explorer);
                }
                Err(error) => {
                    println!("{error}");
                }
            },

            Message::Actions(msg) => return self.actions.update(msg),
            Message::DirsTree(msg) => return self.dirs_tree.update(msg, &mut self.explorer),
            Message::Onglets(msg) => return self.onglets.update(msg),
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .push(Space::new(0, 5))
            .push(self.actions.view())
            .push(
                Row::new()
                    .push(self.dirs_tree.view(&self.explorer))
                    .push(self.onglets.view(self)),
            )
            .into()
    }
}

use std::path;

async fn load(path: PathBuf) -> Result<Explorer, String> {
    Explorer::new(path)
}
