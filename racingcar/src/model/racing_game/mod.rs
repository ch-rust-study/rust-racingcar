use rand::{thread_rng, Rng};

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

  pub fn get_car_names(&self) -> Vec<String> {
    self.cars.iter().map(|car| car.get_name()).collect()
  }

  pub fn play_once(&mut self) {
    for car in self.cars.iter_mut() {
      let mut rng = thread_rng();
      let random_number = rng.gen_range(0..=9);
      let can_go = random_number >= 4;

      if can_go {
        car.go_forward();
      }
    }
  }

  pub fn play(&mut self, show_progress: fn(Vec<&Car>) -> ()) {
    for _ in 0..self.rounds_count {
      self.play_once();
      let mut car_refs: Vec<&Car> = Vec::new();
      for car in self.cars.iter() {
        car_refs.push(car)
      }
      show_progress(car_refs);
    }
  }

  pub fn get_result(&self) -> Vec<&Car> {
    let max_progress = self.cars.iter().map(|car| car.get_progress()).max();

    match max_progress {
      Some(max) => {
        let result: Vec<&Car> = self.cars.iter().filter(|car| car.get_progress() == max).collect();
        result
      },
      None => Vec::new()
    }
  }  
}