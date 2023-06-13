#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(absolute_path)]

use iced::{Application, Settings};

use app::Notes;

mod app;

mod actions;
mod tab;
mod tree;

mod button;
mod explorer;
mod fs;
mod helper;
mod icons;
mod notify;

pub fn main() -> iced::Result {
    //env::set_var("RUST_BACKTRACE", "full");

    Notes::run(Settings::default())
}
