#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(absolute_path)]

use iced::{Application, Settings};

mod explorer;
mod helpers;
mod tabs;
mod top_bar;
mod widgets;

mod app;
use app::{Notes, State};

pub fn main() -> iced::Result {
    //env::set_var("RUST_BACKTRACE", "full");

    State::run(Settings::default())
}
