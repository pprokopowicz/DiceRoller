use roller::roller::{roll, Roll};

fn main() {
    let results = roll(&vec![Roll::new(6, 3)]);
    for result in &results {
        println!("Result: {:?}\nSum: {}", result.throws, result.sum());
    }
}
