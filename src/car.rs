pub struct Car {
    pub name: String,
    pub position: i32,
}

impl Car {
    pub fn new(name: &str) -> Car {
        Car {
            name: String::from(name),
            position: 0,
        }
    }

    pub fn run(&mut self) {
        self.position += 1;
    }

    pub fn log(&self) {
        println!(
            "{} : {}",
            self.name,
            "-".repeat(self.position.try_into().unwrap())
        );
    }
}
