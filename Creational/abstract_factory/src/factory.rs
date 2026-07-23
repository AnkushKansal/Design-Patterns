use crate::PaymentProvider;
use crate::paypal::Paypal;
use crate::stripe::Stripe;

//Factory for Factory
pub trait PaymentFactory: Send + Sync {
    fn create_provider(&self) -> Box<dyn PaymentProvider>;
}

pub struct StripeFactory;

impl PaymentFactory for StripeFactory {
    fn create_provider(&self) -> Box<dyn PaymentProvider> {
        Box::new(Stripe)
    }
}

pub struct PaypalFactory;

impl PaymentFactory for PaypalFactory {
    fn create_provider(&self) -> Box<dyn PaymentProvider> {
        Box::new(Paypal)
    }
}
