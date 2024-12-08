fn greeting(name: &str) -> Result<String, String> {
    // Check if the name is empty. If so, raise an error message.
    if name.is_empty() {
        return Err("ERROR: Name is required to have at least one character.".to_string());
    } else if name.len() > 20 {
        return Err("ERROR: Name cannot be longer than 20 characters.".to_string());
    }

    return Ok(format!("Hello, {}!", name));
}

fn main() {
    // Create an array of names.
    let names: [&str; 3]= [ 
        "Chris", 
        "", 
        "ChrisChrisChrisChrisChris" 
    ];

    // Iterate over the array of names.
    for name in names.iter() {
        // Call the greeting function and print the result.
        match greeting(name) {
            Ok(message) => println!("{}", message),
            Err(error) => println!("{}", error)
        }
    }

    // Ownership and Borrowing
    let s1 = String::from("hello");
    let len = s1.len(); // Immutably borrowed.
    // s1 is still valid here, because we have not mutated it.
    println!("The length of '{}' is {}.", s1, len);
    
    // Mutable borrowing
    let mut s2 = String::from("hello");

    s2.push_str(", world!"); // Mutably borrowed.
    println!("{}", s2);
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
        let expected: &str = "ERROR: Name is required to have at least one character.";
        let actual:Result<String, String> = greeting("");
        assert_eq!(expected, actual.err().unwrap());
    }

    #[test]
    fn test_greeting_long_name() {
        let expected: &str = "ERROR: Name cannot be longer than 20 characters.";
        let actual:Result<String, String> = greeting("ChrisChrisChrisChrisChris");
        assert_eq!(expected, actual.err().unwrap());
    }
}