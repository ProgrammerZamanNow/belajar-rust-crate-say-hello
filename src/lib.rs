pub fn say_hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn say_hello_to_everyone() -> String {
    "Hello, everyone!".to_string()
}

pub fn say_goodbye(name: &str) -> String {
    format!("Goodbye, {}!", name)
}

pub fn say_goodbye_to_everyone() -> String {
    "Goodbye, everyone!".to_string()
}