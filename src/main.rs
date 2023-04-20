use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

use iced::widget::{button, column, text, container};



pub fn main() -> iced::Result {
    
    let settings: iced::Settings<(i32, i32)> = Settings { 
        flags: (0,0),
        ..Settings::default()
    };
    Counter::run(settings)
}


#[derive(Debug, Clone, Copy)]
struct State {
    // The counter value
    value: i32,
    test: i32
}

enum Counter {
    Active(State)
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed(State),
    DecrementPressed(State),
    Test(TestMessage)
}


#[derive(Debug, Clone)]
enum TestMessage {
    TestPressed(State)
}

impl Application for Counter {
    type Executor = executor::Default;
    type Flags = (i32, i32);
    type Theme = Theme;
    type Message = Message;
   
    

    fn new(_flags: Self::Flags) -> (Counter, Command<Self::Message>) {
        println!("new!");
        let s = State {value: _flags.0, test: _flags.1 };

        (Counter::Active(s), Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Message)-> iced::Command<Message> {
        match message {
            Message::IncrementPressed(mut state) => {
                state.value += 1;
            }
            Message::DecrementPressed(mut state) => {
                state.value -= 1;
            }
            Message::Test(TestMessage::TestPressed(mut state)) => {
                state.test += 1;
            }
        }
        println!("after update!");
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // We use a column: a simple vertical layout
        
        match self {
            Counter::Active(mut s) => {

                println!("view!, {}", s.value);

                column![
                // The increment button. We tell it to produce an
                // `IncrementPressed` message when pressed
                button("+").on_press(Message::IncrementPressed(s)),

                // We show the value of the counter here
                text(s.value).size(50),
                // The decrement button. We tell it to produce a
                // `DecrementPressed` message when pressed
                button("-").on_press(Message::DecrementPressed(s)),
            ].into()
            }
        }

        
    }   
}



fn test_view<'a>(state: State)-> Element<'a, TestMessage>  {

    println!("test_view!");
    container(
        text(state.test).size(50)
    ).into()
    
}

