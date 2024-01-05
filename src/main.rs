use std::io;
use std::io::Write;
// write a simple program to compute tax and amount. Only add the tax if the state is Winsconsin or WI.

// Inputs: Order amount, and state.
// Process: add 5.5% tax if state is Winsconsin or WI, if not, then leave it.
// Outputs: (Winsconsin) Subtotal, Tax, Total | (Other states) Total.

fn round_decimal(number: f64, place: i32) -> f64 {
    let multiplier: f64 = 10_f64.powi(place);
    (number * multiplier).round() / multiplier
}

fn calculate_tax(order_amount: f64, tax_percentage: f64) -> (f64, f64) {
    let tax = order_amount * (tax_percentage / 100.0);
    let total = order_amount + tax;
    (round_decimal(tax, 2), round_decimal(total, 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_tax() {
        assert_eq!(calculate_tax(10.0, 5.5), (0.55, 10.55));

        // Check for zero order amount, expecting zero tax and total
        assert_eq!(calculate_tax(0.0, 5.0), (0.0, 0.0));
    
        // Check for negative tax percentage, expecting negative tax and total
        assert_eq!(calculate_tax(200.0, -8.0), (-16.0, 184.0));
    
        // Check for large order amount and tax percentage
        assert_eq!(calculate_tax(999999.99, 20.0), (200000.0, 1199999.99));
    }

    #[test]
    fn test_round_decimal() {
        // Test rounding to 2 decimal places
        assert_eq!(round_decimal(3.14159, 2), 3.14);
        assert_eq!(round_decimal(6.666666, 2), 6.67);

        // Test rounding to 4 decimal places
        assert_eq!(round_decimal(2.718281828, 4), 2.7183);
        assert_eq!(round_decimal(9.999999999, 4), 10.0);

        // Test rounding to 0 decimal places
        assert_eq!(round_decimal(123.456789, 0), 123.0);
        assert_eq!(round_decimal(0.987654321, 0), 1.0);
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

struct State {
    names: Vec<&str>,
    tax_percentage: f32,
}

fn main() {
    let states = vec![
        State {
            names: vec!["wisconsin", "wi"],
            tax_percentage: 5.5,
        }
    ];
    // Prompt for order_amount with message "What is the order amount? "
    // Prompt for state with message "What is the state? "
    // if state is in states
        // get the tax and total
        // print "The subtotal is ${}."
        // print "The tax is ${}."
        // print "The total is ${}."
    // else
        // print "The total is $10.00."
}
