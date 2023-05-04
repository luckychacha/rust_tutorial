pub struct User {
    name: String,
    age: u32,
    weight: f32,
}
// test
impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        Self { name, age, weight }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight
    }
}

pub fn print_info() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name(), bob.age());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weight() {
        let bob = User::new(String::from("Bob"), 32, 155.2);
        assert_eq!(bob.weight(), 155.2);
    }

    #[test]
    fn test_set_age() {
        let mut bob = User::new(String::from("Bob"), 32, 155.2);
        assert_eq!(bob.age(), 32);
        bob.set_age(33);
        assert_eq!(bob.age(), 33);
    }
}

// fn main() {

// }

//
