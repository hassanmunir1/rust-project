use rand::Rng;
use std::io::{self, Write};

fn get_guess() -> u32 {
    loop {
        print!("Please enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        match guess.trim().parse() {
            Ok(num) if num >= 1 && num <= 100 => return num,
            Ok(_) => println!("Please enter a number between 1 and 100."),
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}

fn main() {
    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let guess = get_guess();

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small! Try again."),
            std::cmp::Ordering::Greater => println!("Too large! Try again."),
            std::cmp::Ordering::Equal => {
                println!("Congratulations! You guessed the correct number: {}", secret_number);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_guess() {
        assert_eq!(get_guess(), 50);
    }

    #[test]
    fn test_invalid_guess() {
        // Simulate user input for invalid guess
        let mut input = "0\n".as_bytes();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        assert_eq!(buffer.as_bytes(), input);

        // Call the function to validate input
        assert_eq!(get_guess(), 50);
    }

    #[test]
    fn test_out_of_range_guess() {
        // Simulate user input for out of range guess
        let mut input = "101\n".as_bytes();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        assert_eq!(buffer.as_bytes(), input);

        // Call the function to validate input
        assert_eq!(get_guess(), 50);
    }
}





