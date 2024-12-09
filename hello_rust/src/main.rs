fn greeting(name: &str) -> Result<String, String> {
    // Check if the name is empty. If so, raise an error message.
    if name.is_empty() {
        return Err("ERROR: Name is required to have at least one character.".to_string());
    } else if name.len() > 20 {
        return Err("ERROR: Name cannot be longer than 20 characters.".to_string());
    }

    Ok(format!("Hello, {}!", name))
}

fn string_slice(str_to_slice: &str, start: usize, end: usize) -> String {
    // Check if the start index is greater than the end index.
    if start > end {
        return "ERROR: Your start index must be less than your end index.".to_string();
    } else if end > str_to_slice.len() {
        return "ERROR: Your start end index must be less than or equal to the length of the string.".to_string();
    }
    str_to_slice[start..end].to_string()
}

fn main() {
    // Create an array of names.
    let names: [&str; 3]= [ 
        "Chris", 
        "", 
        "ChrisChrisChrisChrisChris" 
    ];

    let mut counter: usize = 0;

    // Iterate over the array of names.
    for name in names.iter() {
        // Call the greeting function and print the result.
        match greeting(name) {
            Ok(message) => println!("{}", message),
            Err(error) => println!("{}", error)
        }

        // Call the string_slice function and print the result.
        let end= name.len();
        let result = string_slice(name, counter, end);
        println!("{}", result);
        counter += 1;
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

    let mut x = 5;

    // Immutable borrow
    let y = &x; // y is an immutable reference to x.
    // let z = &mut x; // This will cause a compile-time error if y is still in scope.
    println!("y: {}", y); 

    // Mutable borrow
    let z = &mut x; // z is a mutable reference to x. This is allowed since y has been used with the println! statement.

    *z += 1; // Dereference z and increment the value.
    println!("z: {}", z);
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

    #[test]
    fn test_string_slice() {
        let variable: &str = "Hello, world!";
        let start = 7;
        let end = 13;
        let expected: &str = "world!";
        let actual: String = string_slice(variable, start, end);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_string_slice_start_is_greater_than_end() {
        let variable: &str = "Hello, world!";
        let start = 6;
        let end = 5;
        let expected: &str = "ERROR: Your start index must be less than your end index.";
        let actual: String = string_slice(variable, start, end);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_string_slice_end_is_greater_than_length() {
        let variable: &str = "Hello, world!";
        let start = 7;
        let end = 20;
        let expected: &str = "ERROR: Your start end index must be less than or equal to the length of the string.";
        let actual: String = string_slice(variable, start, end);
        assert_eq!(expected, actual);
    }
}