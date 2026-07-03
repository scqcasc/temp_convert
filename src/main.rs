use clap::Parser;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// metric is the default (as it should be)
    /// use --imperial to override default to imperial measuring
    #[arg(short, long)]
    imperial: bool,
}

fn main() {
    let args = Args::parse();
    println!("=== Celsius to Fahrenheit Converter ===");

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
        match input.parse::<f64>() {
            Ok(celsius) => {
                // 5. Calculate and display the result
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("{:.2}°C is equal to {:.2}°F", celsius, fahrenheit);
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
