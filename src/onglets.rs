use iced::Length;


use crate::theme::widget::{Element, Column, Row, Text, Container};

use crate::app::{self};

use crate::theme:: {self};

use iced::widget::{Space};

use iced::alignment;




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


        let onglets = Text::new("hello")
            .height(Length::Fill)
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center);
       


        let content = Container::new(onglets)
            .style(theme::Container::Bordered)
            .height(Length::Fill)
            .width(Length::Fill);

        Container::new(content)
            .padding(10)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
        
    }
}