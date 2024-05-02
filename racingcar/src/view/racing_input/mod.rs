use super::input;

pub fn request_car_names() -> String {
  input::read_line("차 이름을 입력하세요")
}

pub fn request_rounds_count() -> i32 {
  let rounds_count = input::read_line("경기 횟수를 입력하세요");

  match rounds_count.trim().parse() {
      Ok(count) => return count,
      Err(e) => {
        print!("{}", e.to_string());
        return request_rounds_count()
      }
  }
}