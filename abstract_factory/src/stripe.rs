use crate::PaymentProvider;
pub struct Stripe;

impl PaymentProvider for Stripe {
    fn charge(&self, amount: f64) {
        println!("Stripe charge: ${}", amount);
    }

    fn refund(&self, transaction_id: &str) {
        println!("Stripe refund: {}", transaction_id);
    }

    fn validate(&self, payload: &str) -> bool {
        println!("Stripe webhook validation: {}", payload);
        true
    }
}
