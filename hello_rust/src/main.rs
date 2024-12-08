fn greeting(name: &str) -> Result<String, String> {
    // Check if the name is empty. If so, raise an error message.
    if name.is_empty() {
        return Err("Name is required to have at least one character.".to_string());
    } else if name.len() > 20 {
        return Err("Name cannot be longer than 20 characters.".to_string());
    }

    return Ok(format!("Hello, {}!", name));
}

fn main() {
    let name: &str = "Chris";
    let greeting_message: Result<String, String> = greeting(name);
    println!("{}", greeting_message.unwrap());

    // Name is empty, so this will raise an error.
    let name: &str = "";
    let greeting_message: Result<String, String> = greeting(name);
    println!("{}", greeting_message.err().unwrap());

    // Name is too long, so this will raise an error.
    let name: &str = "ChrisChrisChrisChrisChris";
    let greeting_message: Result<String, String> = greeting(name);
    println!("{}", greeting_message.err().unwrap());

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        let expected: &str = "Hello, Chris!";
        let actual:Result<String, String> = greeting("Chris");
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_greeting_empty_name() {
        let expected: &str = "Name is required to have at least one character.";
        let actual:Result<String, String> = greeting("");
        assert_eq!(expected, actual.err().unwrap());
    }

    #[test]
    fn test_greeting_long_name() {
        let expected: &str = "Name cannot be longer than 20 characters.";
        let actual:Result<String, String> = greeting("ChrisChrisChrisChrisChris");
        assert_eq!(expected, actual.err().unwrap());
    }
}