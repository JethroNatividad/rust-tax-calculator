// write a simple program to compute tax and amount. Only add the tax if the state is Winsconsin or WI.

// Inputs: Order amount, and state.
// Process: add 5.5% tax if state is Winsconsin or WI, if not, then leave it.
// Outputs: (Winsconsin) Subtotal, Tax, Total | (Other states) Total.

fn round_decimal(number: f64, place: i64) -> f64 {
    let multiplier = 10_f64.powi(place as i32);
    (number * multiplier).round() / multiplier
}

fn calculate_tax(order_amount: f64, tax_percentage: f64) -> (f64, f64) {
    // calculate the tax
    // round to 2 digits
    // return (tax, total)
    (1.0, 1.0)

}

#[cfg(test)]
mod tests {
    use super::*;

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


fn main() {
    // states = [{names: ["winsconsin", "wi"], tax_percentage: 5.5}]
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
