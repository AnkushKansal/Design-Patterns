use crate::PaymentProvider;

pub struct Paypal;

impl PaymentProvider for Paypal {
    fn charge(&self, amount: f64) {
        println!("PayPal charge: ${}", amount);
    }

    fn refund(&self, transaction_id: &str) {
        println!("PayPal refund: {}", transaction_id);
    }

    fn validate(&self, payload: &str) -> bool {
        println!("PayPal webhook validation: {}", payload);
        true
    }
}
