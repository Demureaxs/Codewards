pub fn run_hello() {
    hello("johN");
    hello("alice");
    hello("");
    codewars_hello("");
}

fn hello(name: &str) -> String {
    let mut greeting = String::from("Hello, ");
    if name.is_empty() {
        greeting.push_str("World!")
    } else {
        let (first, rest) = name.split_at(1);
        let cased = first.to_uppercase() + &rest.to_lowercase() + "!";
        greeting.push_str(&cased);
    }

    return greeting;
}

fn codewars_hello(name: &str) -> String {
    if name.is_empty() {
        return "Hello, World!".to_string();
    };
    format!(
        "Hello, {}{}!",
        name[..1].to_uppercase(),
        name[1..].to_lowercase()
    )
}
