

use iced::widget::{text, column, container, row, button};

use iced::{Element, Alignment, Rectangle, Length};

use iced::widget::{Row, Column, Text, Space};

use iced::theme::{self, Theme};

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


    pub fn view(&self) -> Element<crate::app::Message> {

        row![

            Space::new(5, 0),

            row![
                button("Toggle"),
                button("Settings"),
            ]
            .spacing(10)
            .width(Length::Fill),

            row![
                button("Push"),
                button("Fetch"),
                button("Edit"),
            ]
            .spacing(10)
            .width(Length::Shrink),

            Space::new(5, 0),

        ]
        .into()

   

        
       
        
    }
}