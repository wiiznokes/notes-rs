

use iced::{executor};
use iced::{Application, Command};



use crate::actions::{Actions};

use crate::dirs_tree::{DirsTree};

use crate::onglets::{Onglets};

use crate::theme::{self};

use crate::theme::widget::{Element, Column, Row};


use iced::widget::{Space};



pub struct Notes {
    pub actions: Actions,


    pub dirs_tree: DirsTree,

    pub onglets: Onglets,
}


#[derive(Debug, Clone)]
pub enum Message {
    Actions,
    DirsTree,
    Onglets
}








impl Application for Notes {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = theme::Theme;

    
  

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {

        let mut app = Notes {
            actions: Actions::new(),
            dirs_tree: DirsTree::new(),
            onglets: Onglets::new(),

        };


        let mut command = Command::none();

      

        (app, command)
    }

    fn title(&self) -> String {
        String::from("Notes")
    }


    fn update(&mut self, message: Message) -> iced::Command<Self::Message> {

        let mut ret = Command::none();
        match message {
            Message::Actions => {},
            Message::DirsTree => {},
            Message::Onglets => {}
        }
        ret
    }

   
    fn view(&self) -> Element<Message, iced::Renderer<theme::Theme>> {

        
        
        Column::new()
            .push(Space::new(0, 5))
            .push(self.actions.view())
            .push(
                Row::new()
                    .push(self.dirs_tree.view())
                    .push(self.onglets.view())
            )
            .into()


    
        
    

    }

    
    
}

