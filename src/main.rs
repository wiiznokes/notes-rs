
use iced::{Settings, Application};


mod app;
use app::{Notes};

mod actions;


pub fn main() -> color_eyre::Result<()> {


    color_eyre::install()?;

   
    Notes::run(Settings::default())?;

    Ok(())
}
