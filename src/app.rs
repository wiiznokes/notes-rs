#![allow(dead_code)]
#![allow(unused_variables)]


use std::env;
use std::path::Path;

use iced::{executor};
use iced::{Application, Command};



use crate::actions::{self, Actions};
use crate::dirs_tree::{self, DirsTree};
use crate::file_system;
use crate::onglets::{self, Onglets};

use crate::theme::{self};
use crate::theme::widget::{Element, Column, Row};


use iced::widget::{Space};




pub struct Notes {

    
    pub actions: Actions,
    pub dirs_tree: DirsTree,
    pub onglets: Onglets,

    pub file_system: Option<file_system::DirNode>,

    pub test: i32,
}


#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<file_system::DirNode, String>),
    Actions(actions::Message),
    DirsTree(dirs_tree::Message),
    Onglets(onglets::Message),
}








impl Application for Notes {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = theme::Theme;

    
  

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {

        let app = Notes {
            actions: Actions::new(),
            dirs_tree: DirsTree::new(),
            onglets: Onglets::new(),
            file_system: None,
            test: 0
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
                    Ok(dir_node) => { 
                        file_system::print_dir_node(&dir_node, 0);
                        self.file_system = Some(dir_node);
                    },
                    Err(error) => {
                        println!("{error}");
                    }
                }
                Command::none() 
            }

            Message::Actions(sub_message) => {

                self.actions.update(sub_message, &mut self.test)

            },
            Message::DirsTree(sub_message) => self.dirs_tree.update(sub_message),
            Message::Onglets(sub_message) => self.onglets.update(sub_message),

            _ => Command::none()
        }

    }

   
    fn view(&self) -> Element<Message, iced::Renderer<theme::Theme>> {

        
        
        Column::new()
            .push(Space::new(0, 5))
            .push(self.actions.view())
            .push(
                Row::new()
                    .push(self.dirs_tree.view())
                    .push(self.onglets.view(&self))
            )
            .into()


    }

    
    
}

async fn load(path_str: String) -> Result<file_system::DirNode, String> {

    let path = Path::new(&path_str);

    match file_system::create_dir_node(path) {
        Ok(dir_node) => {
            
            file_system::print_dir_node(&dir_node, 4);
            
            Ok(dir_node)
        }
        Err(error) => Err(error) 
    }

}