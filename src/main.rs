use rand::Rng;
use std::io::{self};

fn input_string(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("[ERROR] unknown error");

    input.trim().to_string()
}

fn input_number(prompt: &str) -> i32 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("[ERROR] unknown error");

    match input.trim().parse::<i32>() {
        Ok(number) => number,
        Err(_) => panic!("[ERROR] 숫자를 입력하세요"),
    }
}

fn is_run() -> bool {
    return rand::thread_rng().gen_range(0..=9) >= 4;
}

#[derive(Clone)]
struct Car {
    name: String,
    position: i32,
}

impl Car {
    fn new(name: &str) -> Car {
        Car {
            name: String::from(name),
            position: 0,
        }
    }

    fn run(&mut self) {
        if is_run() {
            self.position += 1;
        }
    }

    fn log(&self) {
        println!(
            "{} : {}",
            self.name,
            "-".repeat(self.position.try_into().unwrap())
        );
    }
}

struct RacingGame {
    cars: Vec<Car>,
}

impl RacingGame {
    fn new() -> RacingGame {
        RacingGame { cars: Vec::new() }
    }

    fn make_cars(&mut self, cars_input: &str) {
        self.cars = cars_input
            .split(',')
            .map(|name| Car::new(name.trim()))
            .collect()
    }

    fn start(&mut self, count: i32) {
        println!("실행 결과\n");
        for _i in 1..=count {
            for car in &mut self.cars {
                car.run();
                car.log();
            }
            println!("");
        }
    }

    fn get_winner_cars(&self) -> Vec<Car> {
        let max_position = self.cars.iter().map(|car| car.position).max();

        self.cars
            .iter()
            .filter(|car| car.position == max_position.unwrap())
            .cloned()
            .collect()
    }
}

fn main() {
    let mut game = RacingGame::new();
    game.make_cars(&input_string(
        "경주할 자동차 이름을 입력하세요.(이름은 쉼표(,) 기준으로 구분",
    ));
    game.start(input_number("시도할 회수는 몇회인가요?"));

    println!(
        "최종 우승자: {}",
        game.get_winner_cars()
            .iter()
            .map(|car| car.name.as_str())
            .collect::<Vec<&str>>()
            .join(",")
    );
}
