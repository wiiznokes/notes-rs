use crate::app::Notes;

use iced::widget::{text};



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


    //pub fn update(&mut msg: Message) -> iced::Command<crate::app::Message> {

    // }


    pub fn view<'a>() -> iced::Element<'a, crate::app::Message> {

        text("hello").into()
    }
}