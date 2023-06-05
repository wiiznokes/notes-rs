#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;

use iced::{Settings, Application};


mod app;
use app::{Notes};

mod actions;
mod tree;
mod onglets;

mod explorer;
mod icons;
mod notify;
mod button;

pub fn main() {

    //env::set_var("RUST_BACKTRACE", "full");

    let args = env::args();

   



    



   
    Notes::run(Settings::default()).unwrap();

}
