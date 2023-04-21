

use iced::widget::{text, column, container};

use iced::{Element};

use iced::widget::{Row, Column, Text};


#[derive(Clone, Debug, Copy)]
pub struct Actions {


}


#[derive(Clone, Debug)]
pub enum Message {
    Toggle(bool),
    Settings,
    Push,
    Fetch,
    Edit
}




impl Actions {


    pub fn new () -> Actions {

        Actions {  }

    }


    pub fn view_fn(&self) -> Element<crate::app::Message> {

       
        Row::new()
            .push(Text::new("world"))
            .into()

        
    }
}