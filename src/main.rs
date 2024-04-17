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
        self.position += 1;
    }
}

fn main() {
    let mut cars = input_string("경주할 자동차 이름을 입력하세요.(이름은 쉼표(,) 기준으로 구분");
    let mut count = input_number("시도할 회수는 몇회인가요?");

    let cars: Vec<Car> = cars.split(",").map(|name| Car::new(name.trim())).collect();

    for i in 1..=count {
        for car in cars {
            car.run();
        }
    }
}
