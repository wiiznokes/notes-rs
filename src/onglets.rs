use iced::Length;


use crate::theme::widget::{Element, Column, Row, Text};

use crate::app::{self};

use crate::theme:: {self};

use iced::widget::{Space};





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


   

    pub fn view(&self) -> Element<app::Message, iced::Renderer<theme::Theme>> {


        Text::new("hello").into()
       
        
    }
}