fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Person<'a> {
    name: &'a str,
    nickname: &'a str,
}

impl<'a> Person<'a> {
    fn greet(&self) {
        println!(
            "Hello, my name is {} and you can call me {}.",
            self.name, self.nickname
        );
    }
}

fn main() {
    let x = String::from("123");
    let y = String::from("4567");
    println!("{}", longest(&x, &y));

    let person = Person {
        name: &x,
        nickname: &y,
    };
    person.greet();
}
