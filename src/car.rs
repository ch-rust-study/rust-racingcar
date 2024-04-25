use rand::Rng;

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
        if is_run() {
            self.position += 1;
        }
    }

    pub fn log(&self) {
        println!(
            "{} : {}",
            self.name,
            "-".repeat(self.position.try_into().unwrap())
        );
    }
}

fn is_run() -> bool {
    return rand::thread_rng().gen_range(0..=9) >= 4;
}
