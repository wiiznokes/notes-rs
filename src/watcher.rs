use iced::futures::SinkExt;
use iced::{subscription, Subscription};
use inotify::{Event, EventMask, Inotify, WatchMask};

use std::path::Path;
use std::rc::Rc;

use iced::futures::channel::mpsc;

use std::thread;
use std::time::Duration;
use std::{ffi::OsStr, path::PathBuf};

use iced::futures;

use futures::stream::StreamExt;

use std::fmt;

use tokio::task;

use crate::files_explorer::{DirNode, FileNode, Node};

#[derive(Clone, Debug)]
pub enum Message {
    Waiting(mpsc::Sender<Message>),

    Watch(PathBuf),
    StopWatch,
    Stop,

    CreateFile(),
    CreateDir(),
    DeleteFile(PathBuf),
    DeleteDir(PathBuf),
    ModifyFile(PathBuf),
    ModifyDir(PathBuf),
    MoveFromFile(PathBuf),
    MoveFromDir(PathBuf),
    MoveToFile(PathBuf),
    MoveToDir(PathBuf),
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
enum State {
    Waiting,
    Watching(PathBuf),
}
pub fn start_watcher() -> Subscription<Message> {
    struct Connect;

    subscription::channel(
        std::any::TypeId::of::<Connect>(),
        100,
        |mut output| async move {


            let (sender, mut receiver) = mpsc::channel(100);

            output.send(Message::Waiting(sender))
                .await
                .expect("error: can't send msg to app");

            let mut state = State::Waiting;

            loop {
                match &mut state {
            
                    State::Waiting => {

                        let input = receiver.select_next_some().await;

                        match input {
                            Message::Watch(path) => {
                                state = State::Watching(path);
                            },
                            Message::Stop => todo!(),

                            Message::StopWatch => panic!("the watcher is waiting but stopwatch is called"),

                            _ => panic!("should not append")
                        }


                    }

                    State::Watching(path) => {
                        
                        
                        let mut inotify = Inotify::init().expect("Failed to initialize inotify");

                        inotify
                            .add_watch(
                                path.clone(),
                                WatchMask::MODIFY
                                    | WatchMask::CREATE
                                    | WatchMask::DELETE
                                    | WatchMask::DELETE_SELF,
                            )
                            .expect("Failed to add inotify watch");

                        let mut buffer = [0u8; 4096];

                        
                        
                        let handle = thread::spawn( move || {

                            loop {
                                let events = inotify
                                    .read_events_blocking(&mut buffer)
                                    .expect("Failed to read inotify events");


                                for event in events {
                                    if event.mask.contains(EventMask::CREATE) {
                                        if event.mask.contains(EventMask::ISDIR) {
                                            println!("Directory created: {:?}", event.name);
                                            //output.send(Message::CreateDir())
                                                ;
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
                        
                        

                        println!("before receiver.select_next_some().await, {}", path.clone().to_string_lossy());
                        
                        let input = receiver.select_next_some().await;

                        println!("after receiver.select_next_some().await {:?}", input);

                        handle.join();

                        match input {
                            Message::Watch(path) => {
                                state = State::Watching(path);
                            },
                            Message::Stop => todo!(),

                            Message::StopWatch => todo!(),

                            _ => panic!("should not append")
                        }
                    }
                    
                }
            }
        },
    )
}
