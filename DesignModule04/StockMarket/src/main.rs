use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

// Observer trait
trait StockObserver {
    fn update(&self, symbol: &str, price: f64);
}

// Subject (Observable)
struct StockMarket {
    prices: HashMap<String, f64>,
    observers: Vec<Rc<RefCell<dyn StockObserver>>>,
}

impl StockMarket {
    fn new() -> Self {
        StockMarket {
            prices: HashMap::new(),
            observers: Vec::new(),
        }
    }

    fn add_observer(&mut self, observer: Rc<RefCell<dyn StockObserver>>) {
        self.observers.push(observer);
    }

    fn set_price(&mut self, symbol: &str, price: f64) {
        self.prices.insert(symbol.to_string(), price);
        self.notify_observers(symbol, price);
    }

    fn notify_observers(&self, symbol: &str, price: f64) {
        for observer in &self.observers {
            observer.borrow().update(symbol, price);
        }
    }
}

// Concrete Observers
struct PriceDisplay {
    name: String,
}

impl StockObserver for PriceDisplay {
    fn update(&self, symbol: &str, price: f64) {
        println!("{}: {} stock updated to ${:.2}", self.name, symbol, price);
    }
}

struct AlertSystem {
    threshold: f64,
}

impl StockObserver for AlertSystem {
    fn update(&self, symbol: &str, price: f64) {
        if price > self.threshold {
            println!("ALERT: {} stock price ${:.2} exceeds threshold ${:.2}", symbol, price, self.threshold);
        }
    }
}

fn main() {
    let mut stock_market = StockMarket::new();

    let display1 = Rc::new(RefCell::new(PriceDisplay {
        name: String::from("Display 1"),
    }));
    let display2 = Rc::new(RefCell::new(PriceDisplay {
        name: String::from("Display 2"),
    }));
    let alert_system = Rc::new(RefCell::new(AlertSystem {
        threshold: 100.0,
    }));

    stock_market.add_observer(display1);
    stock_market.add_observer(display2);
    stock_market.add_observer(alert_system);

    // Simulate stock price changes
    stock_market.set_price("AAPL", 15.0);
    stock_market.set_price("GOOGL", 1330.0);
    stock_market.set_price("MSFT", 95.0);
    stock_market.set_price("MSFT", 120.0);
}