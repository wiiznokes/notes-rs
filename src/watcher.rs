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
    Connected(mpsc::Sender<Message>),
    Watch(PathBuf),
    RemoveFile(PathBuf),
    CreateFile(PathBuf),
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
enum State {
    Disconnected,
    Connected(mpsc::Receiver<Message>),
}



pub fn start_watch() -> Subscription<Message> {
    println!("start_watch");

    struct Connect;

    subscription::channel(
        std::any::TypeId::of::<Connect>(),
        100,
        |mut output| async move {
            println!("subscription::channel");

            let mut state = State::Disconnected;

            loop {
                match &mut state {
                    State::Disconnected => {
                        println!("Disconnected");

                        let (sender, receiver) = mpsc::channel(100);

                        let res = output.send(Message::Connected(sender)).await;

                        res.expect("error: can't send Connected to app");

                        state = State::Connected(receiver);
                    }

                    State::Connected(re) => {
                        let input = re.select_next_some().await;

                        println!("Receive msg from app: {:?}", input);

                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }
        },
    )
}
