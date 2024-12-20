/// The `Wrapper` struct is a custom generic type that takes another type `T`.
/// It works with types that are `Option<T>`, or other similar type constructors.

pub struct Wrapper<T> {
    value: Option<T>,
}

impl<T> Wrapper<T> {
    /// Creates a new `Wrapper` with an initial value.
    /// 
    /// # Arguments
    /// 
    /// * `value` - An option that holds the value of type `T`.
    /// 
    /// # Returns
    ///
    /// A new `Wrapper` instance containing the `Option<T>`.
    pub fn new(value: Option<T>) -> Self {
        Wrapper { value }
    }

    /// Maps the value inside the `Option<T>` to a new value of type `U`,
    /// applying the function `f` on the current value.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that transforms `T` into `U`.
    ///
    /// # Returns
    ///
    /// A new `Wrapper<U>` containing the transformed value.
    pub fn map<U, F>(self, f: F) -> Wrapper<U>
    where
        F: FnOnce(T) -> U,
    {
        Wrapper {
            value: self.value.map(f),
        }
    }

    /// Unwraps the value inside the `Wrapper` or returns a default value if `None`.
    ///
    /// # Arguments
    ///
    /// * `default` - The value to return in case the `Option<T>` is `None`.
    ///
    /// # Returns
    ///
    /// The value inside the `Wrapper` or the `default` if `None`.
    pub fn unwrap_or(self, default: T) -> T {
        self.value.unwrap_or(default)
    }

    /// Checks if the `Wrapper` contains a value.
    ///
    /// # Returns
    ///
    /// `true` if the `Wrapper` contains `Some(T)`, `false` if `None`.
    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }

    /// Retrieves the inner value, panicking if the value is `None`.
    ///
    /// # Panics
    ///
    /// This method will panic if the `Wrapper` contains `None`.
    pub fn unwrap(self) -> T {
        self.value.unwrap()
    }
}

impl<T> Wrapper<T>
where
    T: std::fmt::Debug,
{
    /// Prints the wrapped value.
    pub fn print(&self) {
        match &self.value {
            Some(val) => println!("{:?}", val),
            None => println!("None"),
        }
    }
}

/// The `run_example` function demonstrates how `Wrapper` works with higher-kinded types.
pub fn run_example() {
    println!("--- Example 1: Basic mapping ---");
    let wrapper = Wrapper::new(Some(10));
    let mapped = wrapper.map(|x| x * 2); // Map `10` to `20`
    mapped.print(); // Prints: 20

    println!("--- Example 2: Unwrap or default ---");
    let wrapper_none = Wrapper::new(None);
    let default_value = wrapper_none.unwrap_or(42); // Use default value `42`
    println!("Unwrapped or default: {}", default_value); // Prints: 42

    println!("--- Example 3: Check if some ---");
    let wrapper_some = Wrapper::new(Some(5));
    println!("Is Some? {}", wrapper_some.is_some()); // Prints: true

    println!("--- Example 4: Panic on unwrap ---");
    // Uncomment to see panic:
    //let wrapper_none = Wrapper::new(None);
    //let panicked_value = wrapper_none.unwrap(); // Will panic: "called `Option::unwrap()` on a `None` value"
}
