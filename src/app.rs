#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]




use std::env;
use std::path::Path;

use iced::{executor};
use iced::{Application, Command};



use crate::actions::{self, Actions};
use crate::dirs_tree::{self, DirsTree};
use crate::file_system;
use crate::onglets::{self, Onglets};


use iced::widget::{Column, Row};
use iced::{ Element};

use iced::widget::{Space};


use crate::file_system::{FileNode, Node, DirNode};

pub struct Notes {

    
    pub actions: Actions,
    pub dirs_tree: DirsTree,
    pub onglets: Onglets,

    pub file_system: Option<Node>,

}


#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<Node, String>),
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

        let app = Notes {
            actions: Actions::new(),
            dirs_tree: DirsTree::new(),
            onglets: Onglets::new(),
            file_system: None,
        };
        
        
        let mut args = env::args();
        // prog name
        args.next();
        
        let arg = args.next();

        let command = if let Some(dir_path) = arg {
            Command::perform(load(dir_path), Message::Loaded)
        } else {
            Command::none()
        };

        (app, command)
    }

    fn title(&self) -> String {
        String::from("Notes")
    }


    fn update(&mut self, message: Message) -> Command<Self::Message> {

        
        match message {

            Message::Loaded(res) => {
                match res {
                    Ok(Node::Dir(dir_node)) => {
                        dir_node.print_dir_node(0);
                        self.file_system = Some(Node::Dir(dir_node));
                    },
                    Err(error) => {
                        println!("{error}");
                    },
                    _ => { panic!() }
                }
                Command::none() 
            }

            Message::Actions(sub_message) => {

                self.actions.update(sub_message)

            },
            Message::DirsTree(sub_message) => self.dirs_tree.update(sub_message, &mut self.file_system),
            Message::Onglets(sub_message) => self.onglets.update(sub_message),

            _ => Command::none()
        }

    }

   
    fn view(&self) -> Element<Message> {

        
        
        Column::new()
            .push(Space::new(0, 5))
            .push(self.actions.view())
            .push(
                Row::new()
                    .push(self.dirs_tree.view(&self.file_system))
                    .push(self.onglets.view(&self))
            )
            .into()


    }

    
    
}

async fn load(path_str: String) -> Result<Node, String> {

    let path = Path::new(&path_str);

    match file_system::create_dir_node(path) {
        Ok(dir_node) => {
            
            dir_node.print_dir_node(4);
            
            Ok(Node::Dir(dir_node))
        }
        Err(error) => Err(error) 
    }

}