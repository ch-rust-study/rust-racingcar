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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn car_run() {
        let mut foo = Car::new("foo");
        let old_position = foo.position;
        foo.run();
        assert_eq!(old_position + 1, foo.position);
    }
}
