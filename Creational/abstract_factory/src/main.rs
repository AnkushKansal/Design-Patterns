use std::sync::Arc;
use std::thread;

use abstract_factory::factory::{PaymentFactory, PaypalFactory, StripeFactory};

fn main() {
    println!("Abstract Factory Design Pattern!");

    println!("Enter the payment provider (stripe/paypal):");

    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    let factory: Arc<dyn PaymentFactory> = match user_input.trim() {
        "stripe" => Arc::new(StripeFactory),
        "paypal" => Arc::new(PaypalFactory),
        _ => {
            eprintln!("Invalid payment provider");
            return;
        }
    };

    let mut handles = vec![];

    for i in 0..4 {
        let factory = Arc::clone(&factory);

        handles.push(thread::spawn(move || {
            let provider = factory.create_provider();

            provider.charge((i + 1) as f64 * 100.0);
            provider.refund("TX123");
            provider.validate("EVENT");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
