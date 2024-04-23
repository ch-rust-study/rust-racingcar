mod view;
mod model;

use view::input;
use model::racing_game::RacingGame;


fn main() {
    let mut racing_game = RacingGame::new();

    let names = input::read_line("차 이름을 입력하세요");
    racing_game.create_cars(names);

    let car_names_result = racing_game.get_car_names().join(" ");
    println!("{}", car_names_result);
}
