use std::time::Instant;
use std::fmt::Debug;

pub struct DataProcessor<T>
where
    T: Clone + Debug,
{
    data: Vec<T>,
}

impl<T> DataProcessor<T>
where
    T: Clone + Debug,
{
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    pub fn process_data<F>(&self, op: F)
    where
        F: Fn(&T),
    {
        for item in &self.data {
            op(item);
        }
    }

    pub fn map_data<F, U>(&self, f: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        self.data.iter().map(f).collect()
    }

    pub fn aggregate<F>(&self, f: F) -> Option<T>
    where
        F: Fn(&T, &T) -> T,
    {
        if self.data.is_empty() {
            return None;
        }
        let mut result = self.data[0].clone();
        for item in &self.data[1..] {
            result = f(&result, item);
        }
        Some(result)
    }

    pub fn time_process_data<F>(&self, op: F)
    where
        F: Fn(&T),
    {
        let start = Instant::now();
        self.process_data(op);
        let duration = start.elapsed();
        println!("Processing took: {:?}", duration);
    }

    pub fn time_map_data<F, U>(&self, f: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        let start = Instant::now();
        let result = self.map_data(f);
        let duration = start.elapsed();
        println!("Mapping took: {:?}", duration);
        result
    }
}

pub fn run_example() {
    let processor = DataProcessor::new(vec![1, 2, 3]);

    // Processing data
    processor.time_process_data(|item| println!("{:?}", item));

    // Mapping data
    let mapped = processor.time_map_data(|x| x * 2);
    println!("Mapped Data: {:?}", mapped);

    // Aggregating data (sum)
    let sum = processor.aggregate(|a, b| a + b);
    println!("Sum of Data: {:?}", sum);

    // Edge case: Aggregating empty data
    let empty_processor: DataProcessor<i32> = DataProcessor::new(vec![]);
    let empty_sum = empty_processor.aggregate(|a, b| a + b);
    println!("Sum of Empty Data: {:?}", empty_sum);
}
