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
    // Create a variable for a valid name, empty name, and a long name.
    let name: &str = "Chris";
    let empty_name: &str = "";
    let long_name: &str = "ChrisChrisChrisChrisChris";

    // Create a counter variable
    let mut counter: i32 = 0;

    // Loop through the names and print the greeting message.
    loop {
        // Check if the counter is 0, 1, or 2.
        if counter == 0 {
            println!("{}", greeting(name).unwrap());
        } else if counter == 1 {
            println!("{}", greeting(empty_name).err().unwrap());
        } else if counter == 2 {
            println!("{}", greeting(long_name).err().unwrap());
        } else {
            // Break the loop if the counter is greater than 2.
            break;
        }

        // Increment the counter by 1.
        counter += 1;
    }
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