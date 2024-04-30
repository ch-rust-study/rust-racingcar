use crate::car::Car;
use rand::Rng;

pub struct RacingGame {
    cars: Vec<Car>,
}

fn is_run() -> bool {
    return rand::thread_rng().gen_range(0..=9) >= 4;
}

impl RacingGame {
    pub fn new() -> RacingGame {
        RacingGame { cars: Vec::new() }
    }

    pub fn make_cars(&mut self, cars_input: &str) {
        self.cars = cars_input
            .split(',')
            .map(|name| Car::new(name.trim()))
            .collect()
    }

    pub fn start(&mut self, count: i32) {
        println!("실행 결과\n");
        for _i in 1..=count {
            for car in &mut self.cars {
                if is_run() {
                    car.run();
                }
                car.log();
            }
            println!("");
        }
    }

    pub fn get_winner_cars(&self) -> Vec<&Car> {
        let max_position = self.cars.iter().map(|car| car.position).max();

        self.cars
            .iter()
            .filter(|car| car.position == max_position.unwrap())
            .collect()
    }
}
