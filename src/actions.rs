use crate::app::Notes;

use iced::widget::{text};



pub struct Actions {


}


#[derive(Clone, Debug)]
pub enum Message {
    Toggle(bool),
    Settings,
    Push,
    Fetch,
    Edit
}




impl Actions {


    pub fn update(&mut msg: Message) -> iced::Command<crate::app::Message> {
        
    }


    pub fn view<'a>() -> iced::Element<'a, crate::app::Message> {

        text("hello").into()


        let tasks: Element<_> = if filtered_tasks.count() > 0 {
            column(
                tasks
                    .iter()
                    .enumerate()
                    .filter(|(_, task)| filter.matches(task))
                    .map(|(i, task)| {
                        task.view(i).map(move |message| {
                            Message::TaskMessage(i, message)
                        })
                    })
                    .collect(),
            )
            .spacing(10)
            .into()
        } else {
            empty_message(match filter {
                Filter::All => "You have not created a task yet...",
                Filter::Active => "All your tasks are done! :D",
                Filter::Completed => {
                    "You have not completed a task yet..."
                }
            })
        };
    }
}