

use iced::executor;
use iced::{Application, Command, Element, Theme};

use iced::widget::{text, column, Column, Space};

use crate::actions::{Actions};


pub struct Notes {
    pub actions: Actions
}


#[derive(Debug, Clone)]
pub enum Message {
    Actions
}


impl Application for Notes {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {

        let mut app = Notes {
            actions: Actions {  }
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
            Message::Actions => {}
        }
        ret
    }

   
    fn view(&self) -> Element<Message> {


        


        column![
            Space::new(0, 5),

            self.actions.view(),

        ]
        .into()

        
    

    }

    
    
}

