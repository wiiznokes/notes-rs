use inotify::{
    Inotify,
    WatchMask, 
    EventMask,
    Event
};

use std::ffi::OsStr;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;

pub fn start_watch(path: &Path) -> Result<(), String> {
    println!("start watch {}", path.display());

      // Create an inotify instance
    let mut inotify = Inotify::init().unwrap();

    // Add a watch to the directory
    let watch_descriptor = inotify.add_watch(
        &path, WatchMask::CREATE | WatchMask::MODIFY)
        .unwrap();
    

    // Spawn a thread to listen for inotify events
    thread::spawn(move || {
        let mut buffer = [0u8; 4096];
        loop {
            let events = inotify.read_events_blocking(&mut buffer).unwrap();

            for event in events {
                match event.mask {
                    EventMask::CREATE | EventMask::MODIFY => {
                        // Handle the event, e.g., print the file name
                        let file_name = event.name.unwrap();
                        println!("New file created/modified: {}", file_name.to_string_lossy());
                    }
                    _ => (),
                }
            }
        }
    });


    Ok(())
}




fn handle_event() {

}