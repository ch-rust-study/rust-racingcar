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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_make_cars() {
        let mut game = RacingGame::new();
        game.make_cars("foo, bar, baz");
        assert_eq!(game.cars.len(), 3);

        assert_eq!(game.cars[0].name, "foo");
        assert_eq!(game.cars[1].name, "bar");
        assert_eq!(game.cars[2].name, "baz");
    }

    #[test]
    fn get_winner_cars_test() {
        let car1 = Car {
            name: String::from("foo"),
            position: 3,
        };

        let car2 = Car {
            name: String::from("bar"),
            position: 3,
        };

        let car3 = Car {
            name: String::from("baz"),
            position: 2,
        };

        let mut game = RacingGame::new();
        game.cars = vec![car1, car2, car3];
        let winners = game.get_winner_cars();

        assert_eq!(winners.len(), 2);
        assert_eq!(winners[0].name, "foo");
        assert_eq!(winners[1].name, "bar");
    }
}
