pub trait Area {
    fn area(&self) -> f64;
}

pub trait Perimeter {
    fn perimeter(&self) -> f64;
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Perimeter for Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

pub fn calculate_shape_properties<T>(shape: &T)
where
    T: Area + Perimeter,
{
    println!(
        "Area: {}, Perimeter: {}",
        shape.area(),
        shape.perimeter()
    );
}

pub fn run_example() {
    let rect = Rectangle { width: 5.0, height: 3.0 };
    calculate_shape_properties(&rect);
}
