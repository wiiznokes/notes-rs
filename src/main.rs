#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

#![feature(absolute_path)]

use iced::{Application, Settings};

use app::Notes;

mod app;

mod actions;
mod tree;
mod tab;

mod explorer;
mod icons;
mod notify;
mod button;
mod fs;
mod helper;

pub fn main() -> iced::Result {

    //env::set_var("RUST_BACKTRACE", "full");


    Notes::run(Settings::default())
}
