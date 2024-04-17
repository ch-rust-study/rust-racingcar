pub struct Car {
  name: String,
  progress: i32,
}

impl Car {
  pub fn new(name: String) -> Car {
    // TODO: validation 필요
    Car {
      name,
      progress: 0,
    }
  }

  pub fn get_name(self) -> String {
    self.name
  }
}