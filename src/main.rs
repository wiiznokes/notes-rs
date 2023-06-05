#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;

use iced::{Settings, Application};


mod app;
use app::{Notes};

mod actions;
mod dirs_tree;
mod onglets;

mod files_explorer;
mod icons;
mod notify;

pub fn main() -> color_eyre::Result<()> {

    //env::set_var("RUST_BACKTRACE", "full");

    let args = env::args();

   

    color_eyre::install()?;

    



   
    Notes::run(Settings::default())?;

    Ok(())
}
