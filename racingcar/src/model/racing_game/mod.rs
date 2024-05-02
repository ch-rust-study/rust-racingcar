use rand::{thread_rng, Rng};

use super::car::Car;

pub struct ValidationError {
  pub message: String,
}

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


  pub fn create_cars(&mut self, car_names_input: &str) -> Result<(), ValidationError> {
    let car_names: Vec<&str> = car_names_input.split(',').collect();
    for name in car_names {
      // TODO: 매직 넘버 처리하기
      if name.len() < 1 || name.len() > 5 {
        return Err(ValidationError {
          message: "차의 이름은 한 글자 이상 다섯 글자 이하가 되어야 합니다.".to_string(),
        })
      }

      self.cars.push(Car::new(name.to_string()))
    }

    Ok(())
  }

  pub fn set_rounds_count(&mut self, count: i32) -> Result<(), ValidationError> {
    if count < 1 {
      return Err(ValidationError {
        message: "1회 이상 게임을 진행해야 합니다.".to_string(),
      })
    }
    self.rounds_count = count;
    Ok(())
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