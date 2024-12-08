fn greeting(name: &str) -> String {
    // Check if the name is empty. If so, raise an error message.
    if name.is_empty() {
        panic!("Name is required to have at least one character.");
    }

    let format_greeting: String = format!("Hello, {}!", name);
    return format_greeting;
}

fn main() {
    let name: &str = "Chris";
    let greeting_message: String = greeting(name);
    println!("{}", greeting_message);

    // Name is empty, so this will raise an error.
    let name: &str = "";
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

    #[test]
    #[should_panic(expected = "Name is required to have at least one character.")]
    fn test_greeting_empty() {
        greeting("");
    }
}