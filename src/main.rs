#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;

use iced::{Settings, Application};


mod app;
use app::{Notes};

mod actions;
mod dirs_tree;
mod onglets;

mod file_system;


pub fn main() -> color_eyre::Result<()> {

    let args = env::args();

   

    color_eyre::install()?;

    



   
    Notes::run(Settings::default())?;

    Ok(())
}
