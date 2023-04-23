#![allow(dead_code)]
#![allow(unused_variables)]


use std::path::PathBuf;

use iced::{Command};
use iced::{Length, Element};




use iced::widget::{Column, Text, Container};

use crate::app::{self};

use crate::file_system;


#[derive(Clone, Debug)]
pub struct DirsTree {


}


#[derive(Clone, Debug)]
pub enum Message {
    Open,
    Move,
    Remove,

    InputChanged,
    Rename,
    NewFile,
    NewDir,
    Cut,
    Copy,
    Paste
}




impl DirsTree {


    pub fn new () -> DirsTree {

        DirsTree {  }

    }


    pub fn update(&mut self, message: Message) -> iced::Command<app::Message> {
        
        match message {
            Message::InputChanged => {},

            _ => { }
        }


        Command::none()
    }

   

    pub fn view(&self, files: &Option<file_system::DirNode>) -> Element<app::Message> {

        let tree: Column<app::Message> = Column::new()
            .padding(10)
            .push(Text::new("v D projet1"))
            .push(Text::new("    F main.rs"))
            .push(Text::new("v D projet2"))
            .push(Text::new("    v D src"))
            .push(Text::new("        F test.c"))
            .push(Text::new("        > D privateProject"))
            .push(Text::new("F file.md"))
            .push(Text::new("F file.txt"));
        
        
       
        let content = Container::new(tree)
            .height(Length::Fill)
            .style(iced::theme::Container::Box);

        Container::new(content)
            .height(Length::Fill)
            .padding(10).into()
        
    }


    
}



//fn view_tree(files: file_system::DirNode) -> Element<app::Message> {

//}