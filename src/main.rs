use clap::Parser;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// overrides default to imperial measuring
    #[arg(short, long)]
    imperial: bool,
}

fn main() {
    let args = Args::parse();
    println!("=== Temperature Converter ===");

    loop {
        let standard = if args.imperial {
            "Fahrenheit"
        } else {
            "Celcius"
        };

        let message = format!("\nEnter temperature in {standard} (or type 'q' to quit): ");

        // 1. Prompt the user for input
        println!("{}", message);

        // Ensure the prompt prints immediately since print! doesn't auto-flush the buffer
        io::stdout().flush().unwrap();

        // 2. Read the user's input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // 3. Clean up the input string
        let input = input.trim();

        // Check if the user wants to exit
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("q") {
            println!("Goodbye!");
            break;
        }

        // 4. Parse the input into a floating-point number
        let char1 = if args.imperial { "F" } else { "C" };
        let char2 = if args.imperial { "C" } else { "F" };
        match input.parse::<f64>() {
            Ok(input_value) => {
                // 5. Calculate and display the result
                let out_value = if args.imperial {
                    fahrenheit_to_celcius(input_value)
                } else {
                    celsius_to_fahrenheit(input_value)
                };
                println!(
                    "{:.2}°{char1} is equal to {:.2}°{char2}",
                    input_value, out_value
                );
            }
            Err(_) => {
                println!("❌ Error: Please enter a valid number or 'exit'.");
            }
        }
    }
}

/// Converts Celsius to Fahrenheit using the formula: (C * 9/5) + 32
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

/// Converts Fahrenheit to Celcius using the formula (F - 32) * 5/9
fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
