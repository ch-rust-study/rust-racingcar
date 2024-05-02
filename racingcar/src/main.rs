mod view;
mod model;

use view::input;
use model::racing_game::RacingGame;

use crate::view::racing_output::{show_racing_progress, show_racing_result, show_racing_result_title};


fn main() {
    let mut racing_game = RacingGame::new();

    let names = input::read_line("차 이름을 입력하세요");
    racing_game.create_cars(names.trim());

    let rounds_count = input::read_line("경기 횟수를 입력하세요");

    match rounds_count.trim().parse() {
        Ok(count) => racing_game.set_rounds_count(count),
        // TODO: handle error
        Err(e) => panic!("hey!! {}", e)
    }
    

    let car_names_result = racing_game.get_car_names().join(" ");
    
    println!("{}", car_names_result);

    
    show_racing_result_title();
    racing_game.play(show_racing_progress);
    let result = racing_game.get_result();
    show_racing_result(result);
}
