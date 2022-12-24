

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
  

    let mut newstate = State{
        data: None,
        parent: None
    };

    let mut prevstate = State{
        data: None,
        parent: None
    };

    event_loop.run(move |event, x, control_flow| {
      *control_flow = ControlFlow::Wait;
      match event {
        Event::NewEvents(StartCause::Init) => {
            newstate = data_handler(prevstate.clone());
        },
        Event::MainEventsCleared => {
            if Some(newstate.data.clone()) != Some(prevstate.data.clone()) {  
                println!("newstate.data.clone() {:?}", newstate.data.clone());
                let newstatedata = newstate.data.clone().unwrap();
                let newstateparent = newstate.parent.clone().unwrap();
                println!("value of newstatedata: {:?}", newstatedata);
                state_updated(newstatedata, newstateparent);
                prevstate = newstate.clone();
            }
        },
        Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } => *control_flow = ControlFlow::Exit,
        _ => (),
      }
    });
  }

  pub fn state_updated(newstatedata: Data, parent: IpcSender<Data>){
    println!("inside state_updated");
    println!("inside state_updated and prevstatedata {:?}", newstatedata);
    println!("inside state_updated and parent {:?}", parent);
    parent.send(newstatedata);
    // unwrappedparent.send(vec![("Dagne".to_string(), "8".to_string())]);
  }

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