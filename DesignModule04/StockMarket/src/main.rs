use std::collections::HashMap;
use std::rc::Rc;
use std::borrow::Cow;

// Observer trait
trait StockObserver {
    fn update(&self, symbol: Cow<str>, price: f64);
}

// Subject (Observable)
struct StockMarket {
    prices: HashMap<String, f64>,
    observers: Vec<Rc<dyn StockObserver>>,
}

impl StockMarket {
    fn new() -> Self {
        StockMarket {
            prices: HashMap::with_capacity(10),
            observers: Vec::with_capacity(10),
        }
    }

    fn add_observer(&mut self, observer: Rc<dyn StockObserver>) {
        self.observers.push(observer);
    }

    fn set_price(&mut self, symbol: &str, price: f64) {
        self.prices.insert(symbol.to_string(), price);
        self.notify_observers(symbol, price);
    }

    fn notify_observers(&self, symbol: &str, price: f64) {
        let symbol = Cow::Borrowed(symbol);
        for observer in &self.observers {
            observer.update(symbol.clone(), price);
        }
    }
}

// Concrete Observers
struct PriceDisplay {
    name: String,
}

impl StockObserver for PriceDisplay {
    fn update(&self, symbol: Cow<str>, price: f64) {
        println!("{}: {} stock updated to ${:.2}", self.name, symbol, price);
    }
}

struct AlertSystem {
    threshold: f64,
}

impl StockObserver for AlertSystem {
    fn update(&self, symbol: Cow<str>, price: f64) {
        if price > self.threshold {
            println!("ALERT: {} stock price ${:.2} exceeds threshold ${:.2}", symbol, price, self.threshold);
        }
    }
}

fn main() {
    let mut stock_market = StockMarket::new();

    let display1 = Rc::new(PriceDisplay {
        name: String::from("Display 1"),
    });
    let display2 = Rc::new(PriceDisplay {
        name: String::from("Display 2"),
    });
    let alert_system = Rc::new(AlertSystem {
        threshold: 100.0,
    });

    stock_market.add_observer(display1);
    stock_market.add_observer(display2);
    stock_market.add_observer(alert_system);

    // Simulate stock price changes
    stock_market.set_price("AAPL", 15.0);
    stock_market.set_price("GOOGL", 1330.0);
    stock_market.set_price("MSFT", 95.0);
    stock_market.set_price("MSFT", 120.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_market() {
        let mut stock_market = StockMarket::new();

        let display1 = Rc::new(PriceDisplay {
            name: String::from("Display 1"),
        });
        let display2 = Rc::new(PriceDisplay {
            name: String::from("Display 2"),
        });
        let alert_system = Rc::new(AlertSystem {
            threshold: 100.0,
        });

        stock_market.add_observer(display1);
        stock_market.add_observer(display2);
        stock_market.add_observer(alert_system);

        stock_market.set_price("AAPL", 15.0);
        stock_market.set_price("GOOGL", 1330.0);
        stock_market.set_price("MSFT", 95.0);
        stock_market.set_price("MSFT", 120.0);
    }

    #[test]
    fn test_price_display() {
        let display = PriceDisplay {
            name: String::from("Display 1"),
        };
        display.update(Cow::Borrowed("AAPL"), 15.0);
    }

    #[test]
    fn test_alert_system() {
        let alert = AlertSystem {
            threshold: 100.0,
        };
        alert.update(Cow::Borrowed("AAPL"), 105.0);
    }

    #[test]
    fn test_stock_observer() {
        let display = PriceDisplay {
            name: String::from("Display 1"),
        };
        let observer: &dyn StockObserver = &display;
        observer.update(Cow::Borrowed("AAPL"), 15.0);
    }

    #[test]
    fn test_stock_observer_vec() {
        let display1 = Rc::new(PriceDisplay {
            name: String::from("Display 1"),
        });
        let display2 = Rc::new(PriceDisplay {
            name: String::from("Display 2"),
        });

        let mut observers: Vec<Rc<dyn StockObserver>> = Vec::new();
        observers.push(display1);
        observers.push(display2);

        for observer in &observers {
            observer.update(Cow::Borrowed("AAPL"), 15.0);
        }
    }
}