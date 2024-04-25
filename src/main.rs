mod car;
mod racing_game;

use racing_game::RacingGame;
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
