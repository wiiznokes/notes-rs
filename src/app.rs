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
use crate::{explorer, notify};
use crate::onglets::{self, Onglets};

use iced::widget::{Column, Row};
use iced::Element;

use iced::widget::Space;

use crate::explorer::{Dir, File, Node};

pub struct Notes {
    explorer_path: Option<PathBuf>,

    pub actions: Actions,
    pub dirs_tree: DirsTree,
    pub onglets: Onglets,

    pub explorer: Option<Node>,

    watcher: Option<Sender<notify::Message>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<Node, String>),
    Actions(actions::Message),
    DirsTree(dirs_tree::Message),
    Onglets(onglets::Message),
    Watcher(notify::Message)
}

impl Application for Notes {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {

        let mut args = env::args();
        // prog name
        args.next();

        let root_path = if let Some(path) = args.next() {
            Some(PathBuf::from(path))
        } else {
            None
        };


        let root_path_clone = root_path.clone();
    

        let app = Notes {
            actions: Actions::new(),
            dirs_tree: DirsTree::new(),
            onglets: Onglets::new(),
            explorer: None,
            watcher: None,
            explorer_path: root_path,
        };

        

        let command = if let Some(path) = root_path_clone {
            Command::perform(load(path.clone()), Message::Loaded)
        } else {
            Command::none()
        };

        (app, command)
    }

    fn title(&self) -> String {
        String::from("Notes")
    }

    
    fn subscription(&self) -> Subscription<Message> {
        println!("subscription (in app)");
        notify::start_watcher().map(Message::Watcher)
    }
  

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Loaded(res) => {
                match res {
                    Ok(Node::Dir(dir_node)) => {
                        self.explorer = Some(Node::Dir(dir_node));
                    }
                    Err(error) => {
                        println!("{error}");
                    }
                    _ => {
                        panic!()
                    }
                }
                Command::none()
            }

            Message::Actions(sub_message) => self.actions.update(sub_message),
            Message::DirsTree(sub_message) => {
                self.dirs_tree.update(sub_message, &mut self.explorer, &mut self.watcher)
            }
            Message::Onglets(sub_message) => self.onglets.update(sub_message),

            Message::Watcher(sub_msg) => {
                println!("receive msg from watcher: {:?}", sub_msg);

                match sub_msg {
                    notify::Message::Waiting(mut sender) => {
                        if let Some(path) = &self.explorer_path {
                            let msg_to_send = notify::Message::Watch(path.clone());
                            sender.try_send(msg_to_send)
                                .expect("error tring to send to watcher");
                        }
                        
                        self.watcher = Some(sender);
                    }
                    _ => {}
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .push(Space::new(0, 5))
            .push(self.actions.view())
            .push(
                Row::new()
                    .push(self.dirs_tree.view(&self.explorer))
                    .push(self.onglets.view(&self)),
            )
            .into()
    }
}


use std::path;

async fn load(path: PathBuf) -> Result<Node, String> {


    match explorer::init_explorer(path) {
        Ok(dir_node) => {
            //println!("{:?}", dir_node);
            
            Ok(Node::Dir(dir_node))
        }
        Err(error) => Err(error),
    }
}
