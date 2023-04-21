#![allow(dead_code)]
#![allow(unused_variables)]


use iced::{executor};
use iced::{Application, Command};



use crate::actions::{self, Actions};
use crate::dirs_tree::{self, DirsTree};
use crate::onglets::{self, Onglets};

use crate::theme::{self};
use crate::theme::widget::{Element, Column, Row};


use iced::widget::{Space};



pub struct Notes {

    
    pub actions: Actions,
    pub dirs_tree: DirsTree,
    pub onglets: Onglets,

    pub test: i32,
}


#[derive(Debug, Clone)]
pub enum Message {
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
            test: 0
        };


        let command = Command::none();

      

        (app, command)
    }

    fn title(&self) -> String {
        String::from("Notes")
    }


    fn update(&mut self, message: Message) -> Command<Self::Message> {

        
        match message {
            Message::Actions(sub_message) => {

               

                // call first function with mutable reference to actions
                let command = self.actions.update(sub_message, &mut self.test);
                
                
                // actions is no longer borrowed mutably at this point

                Command::none()

            },
            Message::DirsTree(sub_message) => self.dirs_tree.update(sub_message),
            Message::Onglets(sub_message) => self.onglets.update(sub_message),
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

