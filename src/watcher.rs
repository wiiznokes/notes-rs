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
use std::time::Duration;

pub fn start_watch(path: &Path) -> Result<(), String> {
    println!("start watch {}", path.display());

    let mut inotify = Inotify::init()
        .expect("Failed to initialize inotify");



        inotify
        .add_watch(
            path,
            WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE | WatchMask::DELETE_SELF,
        )
        .expect("Failed to add inotify watch");

    // Spawn a thread to listen for inotify events
    thread::spawn(move || {

        let mut buffer = [0u8; 4096];
        loop {
            let events = inotify
                .read_events_blocking(&mut buffer)
                .expect("Failed to read inotify events");

            for event in events {
                if event.mask.contains(EventMask::CREATE) {
                    if event.mask.contains(EventMask::ISDIR) {
                        println!("Directory created: {:?}", event.name);
                    } else {
                        println!("File created: {:?}", event.name);
                    }
                } else if event.mask.contains(EventMask::DELETE) {
                    if event.mask.contains(EventMask::ISDIR) {
                        println!("Directory deleted: {:?}", event.name);
                    } else {
                        println!("File deleted: {:?}", event.name);
                    }
                } else if event.mask.contains(EventMask::DELETE_SELF) {
                    if event.mask.contains(EventMask::ISDIR) {
                        println!("Directory self deleted: {:?}", event.name);
                    } else {
                        println!("File self deleted: {:?}", event.name);
                    }
                } else if event.mask.contains(EventMask::MODIFY) {
                    if event.mask.contains(EventMask::ISDIR) {
                        println!("Directory modified: {:?}", event.name);
                    } else {
                        println!("File modified: {:?}", event.name);
                    }
                }
            }
        }
    });


    Ok(())
}


