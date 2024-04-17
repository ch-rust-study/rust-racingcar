use std::io::stdin;

pub fn read_line(question: &str) -> String {
  println!("{}", question);

  let mut buffer = String::new();
  stdin().read_line(&mut buffer).unwrap();

  return buffer;
}