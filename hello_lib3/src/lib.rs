pub struct Greeter {
    greeting: String,
}

impl Greeter {
    pub fn new(greeting: &str) -> Greeter {
        Greeter { greeting: greeting.to_string(), }
    }

    pub fn greet(&self, thing: &str) -> std::string::String {
        format!("{} {}", &self.greeting, thing)
    }
}

#[cfg(test)]
mod test {
    use super::Greeter;

    #[test]
    fn test_greet() {
        let hello = Greeter::new("Hi");
        assert_eq!("Hi Rust", hello.greet("Rust"));
    }
}

