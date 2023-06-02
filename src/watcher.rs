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
enum State<'a> {
    Starting,
    Waiting(&'a mpsc::Receiver<Message>),
    Watching(&'a  mpsc::Receiver<Message>, PathBuf),
}

pub fn start_watcher() -> Subscription<Message> {
    struct Connect;

    subscription::channel(
        std::any::TypeId::of::<Connect>(),
        100,
        |mut output| async move {
            let mut state = State::Starting;

            loop {
                match &mut state {
                    State::Starting => {
                        println!("Start");

                        let (sender, receiver) = mpsc::channel(100);

                        output.send(Message::Waiting(sender))
                            .await
                            .expect("error: can't send msg to app");


                        state = State::Waiting(&receiver);
                    }

                    State::Waiting(receiver) => {

                        let input = receiver.select_next_some().await;

                        match input {
                            Message::Watch(path) => {
                                state = State::Watching(receiver, path);
                            },
                            Message::Stop => todo!(),

                            Message::StopWatch => panic!("the watcher is waiting but stopwatch is called"),

                            _ => panic!("should not append")
                        }

                    }

                    State::Watching(receiver, path) => {

                        let mut inotify = Inotify::init().expect("Failed to initialize inotify");

                        inotify
                            .add_watch(
                                path,
                                WatchMask::MODIFY
                                    | WatchMask::CREATE
                                    | WatchMask::DELETE
                                    | WatchMask::DELETE_SELF,
                            )
                            .expect("Failed to add inotify watch");

                        let mut buffer = [0u8; 4096];
                        
                        let handle = thread::spawn(async move || {

                            loop {
                                let events = inotify
                                    .read_events_blocking(&mut buffer)
                                    .expect("Failed to read inotify events");


                                for event in events {
                                    if event.mask.contains(EventMask::CREATE) {
                                        if event.mask.contains(EventMask::ISDIR) {
                                            println!("Directory created: {:?}", event.name);
                                            output.send(Message::CreateDir())
                                                .await
                                                .expect("error: can't send msg to app");
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
                        

                        
                        let input = receiver.select_next_some().await;

                        handle.join();

                        match input {
                            Message::Watch(path) => {
                                state = State::Watching(receiver, path);
                            },
                            Message::Stop => todo!(),

                            Message::StopWatch => todo!(),

                            _ => panic!("should not append")
                        }
                    }
                    

                    // state: Start, Wait(receiver), Watching(receiver, path)

                    // utiliser le receiver pour
                    // - start le process, avec un path
                    // - stop le watch
                    //
                    // utiliser l'output pour envoyer des message, quand inotify est notifier
                    // si state = Watching et que un nouveau path est envoyé.
                    // -> join le watch
                    // -> remove le watch
                    // -> change le state en Watching(avec le new path)
                    // -> faire un tour de boucle



                    

                    State::Connected(re) => {
                        let input = re.select_next_some().await;

                        let mut inotify = Inotify::init().expect("Failed to initialize inotify");

                        inotify
                            .add_watch(
                                "/home/lenaic/Documents/notes-rs/aaa_test/",
                                WatchMask::MODIFY
                                    | WatchMask::CREATE
                                    | WatchMask::DELETE
                                    | WatchMask::DELETE_SELF,
                            )
                            .expect("Failed to add inotify watch");

                        let mut buffer = [0u8; 4096];
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

                        println!("Receive msg from app: {:?}", input);

                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }
        },
    )
}
