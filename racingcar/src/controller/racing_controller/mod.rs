use crate::{model::racing_game::RacingGame, view::{racing_input, racing_output}};

pub struct RacingController {
  racing_game: RacingGame
}

impl RacingController {
  pub fn new() -> RacingController {
    RacingController {
      racing_game: RacingGame::new(),
    }
  }

  fn set_car_names(&mut self) {
    let names = racing_input::requestCarNames();
    self.racing_game.create_cars(names.trim());
  }

  fn set_rounds_count(&mut self) {
    let rounds_count = racing_input::requestRoundsCount();
    self.racing_game.set_rounds_count(rounds_count);
  }

  fn show_racing_result(&mut self) {
    racing_output::show_racing_result_title();
    self.racing_game.play(racing_output::show_racing_progress);
    let result = self.racing_game.get_result();
    racing_output::show_racing_winners(result);
  }

  pub fn run(&mut self) {
    self.set_car_names();
    self.set_rounds_count();
    self.show_racing_result();
  }
}