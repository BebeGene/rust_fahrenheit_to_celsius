// This program prompts the user to enter a temperature in Fahrenheit and converts it to Celsius.

use std::io;

fn main() {
    // Prompt user to enter a temperature in Fahrenheit
    println!("Enter temp in F:");

    // Create a mutable 'String' variable to hold user input
    let mut input = String::new();

    // Read user's input from standard input (keyboard) and store it in 'input'
    // read_line appends a newline character, so we trim it later
    io::stdin()
        .read_line(&mut input) // Read the input line
        .expect("Failed to read input"); // If an error occurs, print this message and exit

    // Try to parse the input string into a floating-point number (f64)
    // Use 'if let' to handle the 'Result' returned by 'parse'
    if let Ok(f) = input.trim().parse::<f64>() {
        // If parsing is successful, 'f' contains the Fahrenheit value

        // Perform conversion directly within the println! macro
        // Formula: (Fahrenheit - 32) * 5/9 to convert to Celsius
        println!(
            "{:.2}°F is equal to {:.2}°C", // Format Fahrenheit and Celsius with 2 decimal places
            f,                             // Fahrenheit value
            (f - 32.0) * 5.0 / 9.0         // Conversion formula to Celsius
        );
    } else {
        // If parsing fails, print this error message
        println!("Enter a valid number.");
    }
}
