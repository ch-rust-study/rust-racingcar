use super::car::Car;

pub struct RacingGame {
  cars: Vec<Car>,
}

impl RacingGame {
  pub fn new() -> RacingGame {
    RacingGame {
      cars: Vec::new()
    }
  }

  pub fn create_cars(&mut self, car_names_input: String) {
    // TODO: validation 
    let car_names: Vec<&str> = car_names_input.split(',').collect();
    for name in car_names {
      // TODO: validation
      self.cars.push(Car::new(name.to_string()))
    }
  }

  pub fn get_car_names(self) -> Vec<String> {
    self.cars.iter().map(|car| car.get_name()).collect()
  }
}