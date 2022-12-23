

use std::env;
use std::thread;
use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};

type Data = Vec<(String, String)>;

#[derive(Debug, Clone)]
pub struct State {
    data: Option<Data>,
    parent: Option<IpcSender<Data>>
}

fn main() -> wry::Result<()> {



    use wry::{
      application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
      },
      webview::WebViewBuilder,
    };
  
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
      .with_title("Hello World")
      .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
      .with_url("https://tauri.studio")?
      .build()?;
  

    let mut state = State{
        data: None,
        parent: None
    };

    event_loop.run(move |event, x, control_flow| {
      *control_flow = ControlFlow::Wait;
      match event {
        Event::NewEvents(StartCause::Init) => {
            let state = data_handler(state.clone());
            println!("value of returnstate in NewEvents: {:?}, ", state);
        },
        Event::MainEventsCleared => {
            // control_flow_events(state.clone());
        },
        Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } => *control_flow = ControlFlow::Exit,
        _ => (),
      }
    });
  }

//   pub async fn control_flow_events(state: State){
//     // loop{
//     //     println!("inside control_flow_events");
//     //     thread::sleep_ms(1000);
//     // };
//     println!("The value of state: {:?}", state);
//   }

  pub fn data_handler(state: State) -> State {
    println!("inside function test");
    println!("value of state: {:?}", state);
    let args: Vec<String> = env::args().collect();
    println!("value of args {:?}", args);
    let (to_child, from_parent): (IpcSender<Data>, IpcReceiver<Data>) = ipc::channel().unwrap();
    let (to_parent, from_child): (IpcSender<Data>, IpcReceiver<Data>) = ipc::channel().unwrap();
    let bootstrap = IpcSender::connect(args[1].clone()).unwrap();
    bootstrap.send((to_child, from_child)).unwrap();

    let data = from_parent.recv().unwrap();
    let state = State{
        data: Some(data),
        parent: Some(to_parent)
    };

    state

  } 