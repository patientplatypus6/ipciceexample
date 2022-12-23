use std::process::Command;


pub struct Primary {
  pub x: i32, 
  pub y: String,
  pub z: i8
}

impl Primary{
  pub fn test(&self){
    println!("The value of the variables in Primary are :");
    println!("x = {}, y = {}, z = {}", self.x, self.y, self.z);
  }

  #[tokio::main]
  pub async fn getrequest(url: String) -> Result<String, Box<dyn std::error::Error>> {

    println!("Fetching {:?}...", url);
    let res = reqwest::get(url).await?;

    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    Command::new("sh")
        .arg("-C")
        .arg("/Users/peterweyand/Code/rustprojects/project1.1/project1_1/src/test.sh")
        .spawn()
        .expect("sh command failed to start");
        
    Ok(body)
  }
}



