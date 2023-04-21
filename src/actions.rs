





use crate::theme::widget::{Element, Button, Row};

use iced::widget::{Space};

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

        Row::new()
            .push(Space::new(5, 0))
            .push(
                Row::new()
                    .push(Button::new("Toggle"))
            )
            .into()
        
        /* 
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

        */

        
       
        
    }
}