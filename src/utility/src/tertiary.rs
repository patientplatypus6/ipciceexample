



pub struct Tertiary {
  pub x: i32,
  pub y: String,
  pub z: i8
}

impl Tertiary {
  pub fn test(&self){
    println!("The value of the variables in Tertiary are :");
    println!("x = {}, y = {}, z = {}", self.x, self.y, self.z);
  }
}
