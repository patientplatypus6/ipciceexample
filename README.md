# Working Project (Skroderider)

# update v0.0.2

I now have both the webview and the gui opening when running the top level `run.sh` file. They both can access the top level IPC_channel and pass data back and forth.

# update v0.0.1

So far what this repo does is that it has a top level cargo that holds the IPC channel data in the function -

```
fn set_data_vec() -> Data {
    vec![("Peter".to_string(), "36".to_string())]
}
```

In the gui folder this updates this initializes the IPCSender channel, reads the `[("Peter".to_string(), "36".to_string())]` from the parent and then sends `vec![("Dagne".to_string(), "8".to_string())]` back up to the parent on a button push using the function - 

```
pub async fn send_data(parent: IpcSender<Data>){
    parent.send(vec![("Dagne".to_string(), "8".to_string())]);
}
```

The `State` Struct holds `parent`, an IPCSender field, as an Option type (to initialize to `None` as IPCSender does not have the `Default` trait)

```
#[derive(Debug, Clone)]
struct State {
    content: String,
    parent: Option<IpcSender<Data>>
}
```

I think this is a good pattern to use in order to hold state at top level. In this way multiple processes can be spun up and the GUI doesn't "own" state, except locally. Holding `parent: Option<IpcSender<Data>>` within a Struct can be used in other functions as a pattern as well. 

What I intend to do further is include a process that uses a webview and then pass the data from this process to the top level and back down to the gui and back again. 