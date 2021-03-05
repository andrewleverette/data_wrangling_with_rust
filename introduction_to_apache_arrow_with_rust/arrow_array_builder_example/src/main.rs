use arrow::array::Int32Builder;
use rand::distributions::{Distribution, Uniform};

fn main() {
    // 
    let range = Uniform::from(1..100);
    let mut rng = rand::thread_rng();
    
    // Initialize array builder
    let mut primitive_array_builder = Int32Builder::new(100);

    // Randomly gnerate data and append it to the array
    for _ in 0..100 {
        let value = range.sample(&mut rng);
        primitive_array_builder.append_value(value).unwrap();
    }

    // Consume builder and convert to array
    let primitive_array = primitive_array_builder.finish();

    println!("{:?}", primitive_array);
}
