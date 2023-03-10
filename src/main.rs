use std::process::Command;
use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender, IpcReceiver};
use std::env;
use serde::*;
use std::{thread, time};
use utility::primary::*;

type Data = Vec<(String, String)>;
type Bootstrap = (IpcSender<Data>, IpcReceiver<Data>);

fn test_utility(){
    println!("inside test_utility functions");
    println!("now testing Primary::getrequest");
    Primary::getrequest("https://www.google.com".to_string());
}

fn set_data_vec() -> Data {
    vec![
        ("URL".to_string(), "https://www.google.com".to_string()),
        ("Peter".to_string(), "36".to_string())
    ]
}

fn set_data_vec_test() -> Data {
    vec![
        ("test".to_string(), "test".to_string())
    ]
}

fn server_handler(){
    println!("insider server_handler");
    let data = set_data_vec();

    let (server0, server_name0) = IpcOneShotServer::<Bootstrap>::new().unwrap();
    let guiserver = spawn_server(
        "/Users/peterweyand/Code/rustprojects/project1_2/src/rungui.sh".to_string(),
        &server_name0
    );
    // let webserver = spawn_server(
    //     "/Users/peterweyand/Code/rustprojects/project1_2/src/runwebview.sh".to_string(),
    //     &server_name0
    // );
    let (_receiver, (sender, receiver)): (IpcReceiver<Bootstrap>, Bootstrap) = server0.accept().unwrap();
    println!("value of sender outside loop {:?}", sender);
    sender.send(data);

    loop {
        match receiver.try_recv() {
            Ok(res) => {
                println!("Received data in main...{:?}", res);
                println!("Retransmitting to nodes....");
                println!("value of sender in loop {:?}", sender);
                // let data = set_data_vec_test();
                sender.send(res);
            },
            Err(_) => {
                println!("Still waiting in main...");
                std::thread::sleep_ms(1000);
            }
        }
    }
}

fn main() {
    #[allow(warnings, unused)]
    println!("Inside process_handler");
    test_utility();
    server_handler();
}

pub fn spawn_server(address:String, server_name0: &str) -> std::process::Child {
    Command::new(address) 
        .arg(server_name0)
        .spawn()
        .expect("failed to execute server process")
}
