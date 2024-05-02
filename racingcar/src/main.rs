mod view;
mod model;
mod controller;

use controller::racing_controller::RacingController;

fn main() {
    let mut racing_controller = RacingController::new();
    racing_controller.run();    
}
