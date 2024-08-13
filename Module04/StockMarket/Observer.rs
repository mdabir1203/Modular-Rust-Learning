use std::cell::RefCell;
use std::rc::Rc;

struct WeatherStation {fatal: refusing to merge unrelated histories

    observers: Vec<Rc<RefCell<dyn Observer>>>,
}

impl WeatherStation {
    fn new() -> Self {
        WeatherStation {
            observers: Vec::new(),
        }
    }

    fn add_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn set_measurements(&self, temperature: f32, humidity: f32) {
        for observer in &self.observers {
            observer.borrow_mut().update(temperature, humidity);
        }
    }
}

trait Observer {
    fn update(&self, temperature: f32, humidity: f32);
}

struct DisplayDevice {
    name: String,
}

impl Observer for DisplayDevice {
    fn update(&self, temperature: f32, humidity: f32) {
        println!(
            "{}: Temperature = {}°C, Humidity = {}%",
            self.name, temperature, humidity
        );
    }
}

fn main() {
    let mut weather_station = WeatherStation::new();
    let display1 = Rc::new(RefCell::new(DisplayDevice {
        name: String::from("Display 1"),
    }));
    let display2 = Rc::new(RefCell::new(DisplayDevice {
        name: String::from("Display 2"),
    }));

    weather_station.add_observer(display1.clone());
    weather_station.add_observer(display2.clone());

    weather_station.set_measurements(25.5, 60.0);
    weather_station.set_measurements(26.0, 58.5);
}