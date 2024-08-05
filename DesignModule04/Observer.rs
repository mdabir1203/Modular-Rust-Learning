use std::cell::RefCell;

fn main()
{
    let mut weather_station = WeatherStation::new();
    let display1 = Rc::new(RefCell::new(DisplayDevice{
        name: String::from("Display 1"),
    }));
    let display2 = Rc::new(RefCell::new(DisplayDevice{
        name: String::from("Display 2"),
    }));

    weather_station.add_observer(display1);
    weather_station.add_observer(display2);

    weather_station.set_measurements(25.5, 60.0);
    weather_station.set_measurements(26.0, 58.5);

}