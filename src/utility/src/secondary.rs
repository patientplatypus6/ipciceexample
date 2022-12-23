




pub struct Secondary {
  pub x: i32,
  pub y: String,
  pub z: i8
}

impl Secondary {
  #![allow(warnings, unused)]

  pub fn test(&self){
    println!("The value of the variables in Secondary are :");
    println!("x = {}, y = {}, z = {}", self.x, self.y, self.z);
  }
}
