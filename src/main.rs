#![allow(dead_code)]
#![allow(unused_variables)]

use iced::{Settings, Application};


mod app;
use app::{Notes};

mod actions;
mod dirs_tree;
mod onglets;
mod theme;

pub fn main() -> color_eyre::Result<()> {


    color_eyre::install()?;

   
    Notes::run(Settings::default())?;

    Ok(())
}
