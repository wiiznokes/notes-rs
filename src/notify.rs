use futures::{
    channel::mpsc::Sender,
    SinkExt, StreamExt,
};
use iced::{Subscription, subscription};
use iced::futures::channel::mpsc;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::path::PathBuf;



#[derive(Clone, Debug)]
pub enum Message {
    Waiting(mpsc::Sender<Message>),

    Watch(PathBuf),
    StopWatch(PathBuf),
    Stop,

    Event(Event),
}


pub fn start_watcher() -> Subscription<Message> {
    struct Connect;

    subscription::channel(
        std::any::TypeId::of::<Connect>(),
        100,
        |mut output| async move {


            let (app_sender, mut app_receiver) = mpsc::channel(100);

            output.send(Message::Waiting(app_sender))
                .await
                .expect("error: can't send msg to app");



            let mut watcher = async_watcher(output).unwrap();


            loop {
                
                if let Some(res) = app_receiver.next().await {
                    println!("receive from app: {:?}", res);
                    
                    match res {
                        Message::Watch(path) => {
                            watcher.watch(path.as_path(), RecursiveMode::NonRecursive).unwrap();
                        },
                        Message::StopWatch(path) => todo!(),
                        Message::Stop => todo!(),
                        _ => panic!()
                    }
                }
            }
        },
    )
}



fn async_watcher(mut output: Sender<Message>) -> notify::Result<RecommendedWatcher> {

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(move |res: Result<Event, notify::Error>| {
        futures::executor::block_on(async {

            output.send(Message::Event(res.unwrap())).await.unwrap();
        })
    }, Config::default())?;

    Ok(watcher)
}