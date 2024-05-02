mod view;
mod model;

use model::racing_game::RacingGame;
use view::racing_output::show_racing_winners;

use crate::view::{racing_input::{requestCarNames, requestRoundsCount}, racing_output::{show_racing_progress, show_racing_result_title}};


fn main() {
    let mut racing_game = RacingGame::new();

    let names = requestCarNames();
    racing_game.create_cars(names.trim());

    let rounds_count = requestRoundsCount();
    racing_game.set_rounds_count(rounds_count);
    
    show_racing_result_title();
    racing_game.play(show_racing_progress);
    let result = racing_game.get_result();
    show_racing_winners(result);
}
