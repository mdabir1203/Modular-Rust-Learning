

// Payment trait definition
pub trait Payment {
    fn process(&self, amount: f64) -> Result<String, String>;
    fn refund(&self, amount: f64) -> Result<String, String>;
}

// CreditCard payment method
pub struct CreditCard {
    card_number: String,
    expiry_date: String,
    cvv: String,
}

// Business logic here
impl Payment for CreditCard {
    fn process(&self, amount: f64) -> Result<String, String> {
        if amount > 0.0 && amount < 10000.0 {
            Ok(format!("Processed ${:.2} via Credit Card ending in {}", amount, &self.card_number[self.card_number.len()-4..]))
        } else {
            Err("Invalid amount for credit card transaction".to_string())
        }
    }

    fn refund(&self, amount: f64) -> Result<String, String> {
        if amount > 0.0 {
            Ok(format!("Refunded ${:.2} to Credit Card ending in {}", amount, &self.card_number[self.card_number.len()-4..]))
        } else {
            Err("Invalid refund amount".to_string())
        }
    }
}

// Stripe payment method
pub struct Stripe {
    email: String,
}

impl Payment for Stripe {
    fn process(&self, amount: f64) -> Result<String, String> {
        if amount > 0.0 && amount < 5000.0 {
            Ok(format!("Processed ${:.2} via Stripe account {}", amount, self.email))
        } else {
            Err("Invalid amount for Stripe transaction".to_string())
        }
    }

    fn refund(&self, amount: f64) -> Result<String, String> {
        if amount > 0.0 {
            Ok(format!("Refunded ${:.2} to Stripe account {}", amount, self.email))
        } else {
            Err("Invalid refund amount".to_string())
        }
    }
}

// PaymentType enum for factory method
pub enum PaymentType {
    CreditCard(String, String, String),
    Stripe(String),
}

// PaymentFactory implementation
pub struct PaymentFactory;

impl PaymentFactory {
    fn create_payment(&self, payment_type: PaymentType) -> Box<dyn Payment> {
        match payment_type {
            PaymentType::CreditCard(number, expiry, cvv) => {
                Box::new(CreditCard {
                    card_number: number,
                    expiry_date: expiry,
                    cvv,
                })
            },
            PaymentType::Stripe(email) => {
                Box::new(Stripe { email })
            },
        }
    }
}

// PaymentProcessor implementation
pub struct PaymentProcessor {
    factory: PaymentFactory,
}

impl PaymentProcessor {
    fn new() -> Self {
        PaymentProcessor {
            factory: PaymentFactory,
        }
    }

    pub fn process_payment(&self, payment_type: PaymentType, amount: f64) -> Result<String, String> {
        let payment = self.factory.create_payment(payment_type);
        payment.process(amount)
    }

    pub fn process_refund(&self, payment_type: PaymentType, amount: f64) -> Result<String, String> {
        let payment = self.factory.create_payment(payment_type);
        payment.refund(amount)
    }
}



fn main() {
    let processor = PaymentProcessor::new();

    println!("Payment Processing Demo\n");

    // Successful credit card payment
    match processor.process_payment(
        PaymentType::CreditCard(
            "1234567890123456".to_string(),
            "12/25".to_string(),
            "123".to_string()
        ),
        100.50
    ) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Successful Stripe payment
    match processor.process_payment(
        PaymentType::Stripe("user@example.com".to_string()),
        75.25
    ) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Credit card payment exceeding limit
    match processor.process_payment(
        PaymentType::CreditCard(
            "9876543210987654".to_string(),
            "06/24".to_string(),
            "456".to_string()
        ),
        15000.00
    ) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Successful refund
    match processor.process_refund(
        PaymentType::Stripe("refund.user@example.com".to_string()),
        50.00
    ) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nDemo completed.");
}