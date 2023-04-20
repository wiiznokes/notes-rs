

use iced::executor;
use iced::{Application, Command, Element, Theme};

use iced::widget::{text, column};




pub struct Notes {

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

        let mut my_vector: Vec<Element<'_, _, _>> = Vec::new();

        my_vector.push(text("yo").into());

        my_vector.push(crate::actions::Actions::view().into());

        column(my_vector).into()
        

    }

    
    
}

