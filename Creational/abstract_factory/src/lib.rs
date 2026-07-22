pub trait PaymentProvider {
    fn charge(&self, amount: f64);
    fn refund(&self, transaction_id: &str);
    fn validate(&self, payload: &str) -> bool;
}

pub mod factory;
pub mod paypal;
pub mod stripe;
