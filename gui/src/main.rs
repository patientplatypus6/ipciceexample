use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};
use std::collections::HashMap;
use std::process::Command as ProcessCommand;
use std::env;
use std::io;
use std::{thread, time};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use iced::widget::{button, container, row, column, text, text_input, Text};
use iced::{executor, Application, Command, Length, Settings, Subscription};
use iced::Color;
use self::theme::Theme;
use self::widget::Element;
use std::future::Future;
// use once_cell::sync::Lazy;

type Data = Vec<(String, String)>;
type Bootstrap = (IpcSender<Data>, IpcReceiver<Data>);

// static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Debug, Clone)]
struct State {
    input_value: String,
    addressclicked: i32,
    content: String,
    parent: Option<IpcSender<Data>>, 
    data: Option<Data>
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPress(String), 
    AddressChanged(String), 
    AddressEntered,
    IPC_DATA(Data, IpcSender<Data>), 
    IPC_SENT(), 
    URL_UPDATED()
}

struct App{
    state: State
}

mod widget {
    #![allow(dead_code)]
    use crate::theme::Theme;

    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
}

mod theme {
    use iced::widget::{button, container, text, text_input};
    use iced::{application, color, Color};

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Theme;

    impl application::StyleSheet for Theme {
        type Style = ();

        fn appearance(&self, _style: &Self::Style) -> application::Appearance {
            application::Appearance {
                background_color: color!(0x28, 0x28, 0x28),
                text_color: color!(0xeb, 0xdb, 0xb2),
            }
        }
    }

    impl text::StyleSheet for Theme {
        type Style = ();

        fn appearance(&self, _style: Self::Style) -> text::Appearance {
            text::Appearance {
                color: color!(0xeb, 0xdb, 0xb2).into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Container {
        #[default]
        Default,
        Bordered,
        WhiteBackground
    }

    impl container::StyleSheet for Theme {
        type Style = Container;

        fn appearance(&self, style: &Self::Style) -> container::Appearance {
            match style {
                Container::Default => container::Appearance::default(),
                Container::Bordered => container::Appearance {
                    border_color: color!(0x45, 0x85, 0x88),
                    border_width: 1.0,
                    border_radius: 4.0,
                    ..Default::default()
                },
                Container::WhiteBackground => container::Appearance{
                    background: Some(iced::Background::Color(Color::from_rgba(0.8, 0.2, 0.3, 1.0))),
                    ..Default::default()
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Button {
        #[default]
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Theme { 
        type Style = Button;

        fn active(&self, style: &Self::Style) -> button::Appearance {
            match style {
                Button::Primary => button::Appearance {
                    background: color!(0x28, 0x28, 0x28).into(),
                    border_radius: 4.0,
                    border_width: 1.0,
                    border_color: color!(0x45, 0x85, 0x88),
                    ..Default::default()
                },
                Button::Secondary => button::Appearance {
                    background: color!(0x3c, 0x38, 0x36).into(),
                    ..Default::default()
                },
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum TextInput {
        #[default]
        Default,
    }

    impl text_input::StyleSheet for Theme {
        type Style = TextInput;
        fn active(&self, style: &Self::Style) -> text_input::Appearance{
            text_input::Appearance {
                background:  iced::Background::Color(Color{ r: 1.0, g: 1.0, b: 1.0, a: 1.0,}),
                border_color: Color{ r: 0.0, g: 0.0, b: 0.0, a: 1.0,},
                border_radius: 3.0,
                border_width: 1.0
            }
        }
        fn focused(&self, style: &Self::Style) -> text_input::Appearance{
            text_input::Appearance {
                background:  iced::Background::Color(Color{ r: 1.0, g: 1.0, b: 1.0, a: 1.0,}),
                border_color: Color{ r: 0.0, g: 0.0, b: 0.0, a: 1.0,},
                border_radius: 3.0,
                border_width: 1.0
            }
        }   
        fn placeholder_color(&self, style: &Self::Style) -> iced::Color{
            Color{ r: 0.0, g: 0.0, b: 0.0, a: 1.0,}
        }  
        fn value_color(&self, style: &Self::Style) -> iced::Color{
            Color{ r: 0.0, g: 0.0, b: 0.0, a: 1.0,}
        }   
        fn selection_color(&self, style: &Self::Style) -> iced::Color{
            Color{ r: 1.0, g: 1.0, b: 1.0, a: 1.0,}
        }  
    }
}


impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (App{
            state:State{
                content: "default state".to_string(), 
                parent: None,           
                data: None,
                addressclicked: 0,      
                input_value:"input coordinates skroadryder".to_string()}
            },Command::perform(data_handler(), |(x,y)| Message::IPC_DATA(x,y)))
    }

    fn title(&self) -> String {
        "Custom Theme".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ButtonPress(m) => {
                println!("The value of the button press is {:?}", m);
                self.state.content = m;
                println!("The new value of the state is {:?}", self.state.content);
                match self.state.parent.clone() {
                    Some(x) => Command::perform(send_data(x),|_| Message::IPC_SENT()),
                    None    => Command::none()
                }
            }
            Message::IPC_DATA(x, y) => {
                self.state.data = Some(x);
                self.state.parent = Some(y);
                Command::none()
            }
            Message::IPC_SENT() => {
                println!("IPC_SENT!");
                Command::perform(data_handler(), |(x,y)| Message::IPC_DATA(x,y))
            }
            Message::URL_UPDATED() => {
                println!("URL Updated");
                Command::none()
            }
            Message::AddressChanged(m) => {
                println!("The value of m is {:?}", m);
                println!("The value of mstring is {:?}", m);
                println!("The value of the input value is {:?}", self.state.input_value);
                if self.state.addressclicked == 0 {
                    self.state.addressclicked = 1;
                    let lastchar = m.chars().last().unwrap().to_string();
                    println!("value of lastchar {:?}", lastchar);
                    self.state.input_value = lastchar
                }else{
                    self.state.input_value = m
                }
                Command::none()
            }
            Message::AddressEntered => {
                if !self.state.input_value.is_empty() {
                    println!("the input value is not empty and its value is {:?}", self.state.input_value.to_string());
                    println!("self.state.data {:?}", self.state.data);
                    println!("self.state.parent {:?}", self.state.parent);
                    let data = self.state.data.clone().unwrap();
                    let parent =  self.state.parent.clone().unwrap();
                    let newurl = self.state.input_value.to_string().clone();
                    Command::perform(update_url(data, parent, newurl),|_| Message::URL_UPDATED())
                }else{
                    println!("the input value is empty!");
                    Command::none()
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {

        let input = text_input(
            &self.state.input_value,
            &self.state.input_value,
            Message::AddressChanged,
        )
        .padding(15)
        .width(iced::Length::Units(1000))
        .size(30)
        .on_submit(Message::AddressEntered);

        container(
            column![
                container(
                    input
                )
                .padding(10),
                container(
                    row![
                        button(text("primary"))
                            .style(theme::Button::Primary)
                            .on_press(Message::ButtonPress("primary".to_string())),
                        button(text("secondary"))
                            .style(theme::Button::Secondary)
                            .on_press(Message::ButtonPress("secondary".to_string())),
                        container(text("hello"))
                            .style(theme::Container::WhiteBackground),
                        text("The value of the button pressed is : "),
                        text(&self.state.content)
                    ]
                    .spacing(10),
                )
                .padding(20)
                .style(theme::Container::Bordered)
            ]
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

pub async fn testtest(){
    println!("inside testtest");
}

pub async fn update_url(data: Data, parent: IpcSender<Data>, newurl: String){

    println!("value of data is: {:?}", data);
    println!("value of parent is: {:?}", parent); 
    println!("value of newurl: {:?}", newurl);

    let mut newdata = data.clone();

    for (i, el) in data.clone().iter().enumerate() {
        println!("The current element is {:?}", el);
        println!("The current index is {:?}", i);
        if el.0=="URL"{
            println!("value of el.1 {:?}", el.1);
            newdata[i].1 = newurl.clone();
        }
    }

    println!("value of newdata {:?}", newdata);
    parent.send(newdata);
}

pub async fn send_data(parent: IpcSender<Data>){
    parent.send(vec![("Dagne".to_string(), "8".to_string())]);
}

pub async fn data_handler()-> (Data,IpcSender<Data>){

    let args: Vec<String> = env::args().collect();
    println!("value of args {:?}", args);
    let (to_child, from_parent): (IpcSender<Data>, IpcReceiver<Data>) = ipc::channel().unwrap();
    let (to_parent, from_child): (IpcSender<Data>, IpcReceiver<Data>) = ipc::channel().unwrap();
    let bootstrap = IpcSender::connect(args[1].clone()).unwrap();
    bootstrap.send((to_child, from_child)).unwrap();

    let data = from_parent.recv().unwrap();
    (data, to_parent)
}


fn main() {
    #![allow(warnings, unused)]
    println!("Inside GUI process");
    App::run(Settings::default()).unwrap();
}
