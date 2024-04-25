use super::car::Car;

pub struct RacingGame {
  cars: Vec<Car>,
  rounds_count: i32,
}

impl RacingGame {
  pub fn new() -> RacingGame {
    RacingGame {
      cars: Vec::new(),
      rounds_count: 0,
    }
  }

  pub fn create_cars(&mut self, car_names_input: &str) {
    // TODO: validation 
    let car_names: Vec<&str> = car_names_input.split(',').collect();
    for name in car_names {
      // TODO: validation
      self.cars.push(Car::new(name.to_string()))
    }
  }

  pub fn set_rounds_count(&mut self, count: i32) {
    self.rounds_count = count
  }

  pub fn get_car_names(self) -> Vec<String> {
    self.cars.iter().map(|car| car.get_name()).collect()
  }
}