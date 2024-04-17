mod view;

use view::input;


fn main() {
    let names = input::read_line("차 이름을 입력하세요");
    println!("{}", names);
}
