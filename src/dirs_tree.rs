

use iced::widget::{text, column, container, row, button};

use iced::{Element, Alignment, Rectangle, Length, Padding};

use iced::widget::{Row, Column, Text, Space};

use iced::theme::{self, Theme};

#[derive(Clone, Debug, Copy)]
pub struct DirsTree {


}


#[derive(Clone, Debug)]
pub enum Message {
    Open,
    Move,
    Remove,
    Rename,
    NewFile,
    NewDir,
    Cut,
    Copy,
    Paste
}




impl DirsTree {


    pub fn new () -> DirsTree {

        DirsTree {  }

    }


   

    pub fn view(&self) -> Element<crate::app::Message> {


        Column::new()
            .padding(10)
            .push(text("v D projet1"))
            .push(text("    F main.rs"))
            .push(text("v D projet2"))
            .push(text("    v D src"))
            .push(text("        F test.c"))
            .push(text("        > D privateProject"))
            .push(text("F file.md"))
            .push(text("F file.txt"))
            .into()
       
        
    }
}