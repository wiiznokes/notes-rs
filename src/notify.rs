#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use iced::futures::channel::mpsc;
use iced::{subscription, Subscription};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum NtfMsg {
    Waiting(Sender<NtfMsg>),

    Watch(PathBuf),
    StopWatch(PathBuf),
    Stop,

    Event(Event),
}

pub fn start_watcher() -> Subscription<NtfMsg> {
    struct Connect;

    subscription::channel(
        std::any::TypeId::of::<Connect>(),
        100,
        |mut output| async move {
            let (app_sender, mut app_receiver) = mpsc::channel(100);

            output
                .send(NtfMsg::Waiting(app_sender))
                .await
                .expect("error: can't send msg to app");

            let mut watcher = async_watcher(output).unwrap();

            println!("start watching");
            loop {
                if let Some(res) = app_receiver.next().await {
                    println!("receive from app: {:?}", res);

                    match res {
                        NtfMsg::Watch(path) => {
                            watcher
                                .watch(path.as_path(), RecursiveMode::NonRecursive)
                                .unwrap();
                        }
                        NtfMsg::StopWatch(path) => todo!(),
                        NtfMsg::Stop => todo!(),
                        _ => panic!(),
                    }
                }
            }
        },
    )
}

fn async_watcher(mut output: Sender<NtfMsg>) -> notify::Result<RecommendedWatcher> {
    let watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            futures::executor::block_on(async {
                output.send(NtfMsg::Event(res.unwrap())).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok(watcher)
}
