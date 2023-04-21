

use iced::widget::{text, column, container, row, button};

use iced::{Element, Alignment, Rectangle, Length, Padding};

use iced::widget::{Row, Column, Text, Space};

use iced::theme::{self, Theme};

#[derive(Clone, Debug, Copy)]
pub struct Onglets {


}


#[derive(Clone, Debug)]
pub enum Message {
    Close
}




impl Onglets {


    pub fn new () -> Onglets {

        Onglets {  }

    }


   

    pub fn view(&self) -> Element<crate::app::Message> {


        text("hello").into()
       
        
    }
}