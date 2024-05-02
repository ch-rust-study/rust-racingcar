use super::input;

pub fn requestCarNames() -> String {
  input::read_line("차 이름을 입력하세요")
}

pub fn requestRoundsCount() -> i32 {
  let rounds_count = input::read_line("경기 횟수를 입력하세요");

  match rounds_count.trim().parse() {
      Ok(count) => return count,
      // TODO: handle error
      Err(e) => {
        print!("{}", e.to_string());
        return requestRoundsCount()
      }
  }
}