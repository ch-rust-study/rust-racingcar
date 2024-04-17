mod view;
mod model;

use view::input;
use model::car::Car;


fn main() {
    let names = input::read_line("차 이름을 입력하세요");
    let car = Car::new(names);
    println!("{}", car.get_name());
}
