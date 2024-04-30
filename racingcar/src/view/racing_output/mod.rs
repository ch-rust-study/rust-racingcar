use crate::model::car::Car;

pub fn show_racing_progress(cars: Vec<&Car>) {
  for car in cars.iter() {
    print!("{}: {}\n", car.get_name(), "-".repeat(car.get_progress() as usize))
  }

  print!("\n----------\n")
}