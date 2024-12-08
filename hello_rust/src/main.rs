fn greeting(name: &str) -> String {
    let format_greeting: String = format!("Hello, {}!", name);
    return format_greeting;
}

fn main() {
    let name: &str = "Chris";
    let greeting_message: String = greeting(name);
    println!("{}", greeting_message);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        let expected: &str = "Hello, Chris!";
        let actual: String = greeting("Chris");
        assert_eq!(expected, actual);
    }
}