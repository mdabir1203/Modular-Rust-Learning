use std::fmt;

trait CPU {
    fn manufacture(&self)-> String;
    fn customize(&mut self, company: &str);
}

#[derive(Default)]
struct Keyboard {
    company: String
}

impl CPU for Keyboard {
    fn manufacture(&self) -> String {
        format!("Creating a {} Keyboard", self.company)
    }

    fn customize(&mut self, company: &str) {
        self.company = company.to_string();
    }
}

impl fmt::Display for Keyboard {
    fn fmt(&self, fan: &mut fmt::Formatter) -> fmt::Result {
        write!(fan, "Keyboard ({})")
    }
}

// Similar impl for Mouse and Soundbox

enum CpuType {
    Keyboard,
    Mouse,
    Soundbox
}
struct CpuFactory;

impl CpuFactory {
    fn create_cpu(&self, cpu_factory: CpuFactory) -> Box<dyn CPU> {
        match cpu_factory {
            CpuType::Keyboard => Box::new(Keyboard::default()),
            CpuType::Mouse => Box::new(Mouse::default()),
            CpuType::Soundbox => Box::new(Soundbox::default())
        }
    }
}

fn main() {
    let factory = CpuFactory;

    let mut Keyboard = factory.create_cpu(CpuType::Keyboard);
    Keyboard.customize("Dell");
    println!("{}", Keyboard.manufacture());

    let mut Mouse = factory.create_cpu(CpuType::Mouse);
    Mouse.customize("Logitech");
    println!("{}", Mouse.manufacture());

    let mut Soundbox = factory.create_cpu(CpuType::Soundbox);
    Soundbox.customize("Sony");
    println!("{}", Soundbox.manufacture());
}