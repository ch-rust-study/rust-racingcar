use crate::model::car::Car;

pub fn show_racing_progress(cars: Vec<&Car>) {
  print!("\n실행결과\n\n");

  for car in cars.iter() {
    print!("{}: {}\n", car.get_name(), "-".repeat(car.get_progress() as usize))
  }

  print!("\n----------\n")
}

pub fn show_racing_result(result: Vec<&Car>) {
  print!("최종 우승자 : {}",
  result
    .iter()
    .map(|car| car.get_name())
    .collect::<Vec<String>>()
    .join(",")
  ) 
}