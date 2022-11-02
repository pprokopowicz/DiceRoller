use roller::roller::{roll, Roll};

mod result_printer;

fn main() {
    let rolls = vec![
        Roll::new(6, 10),
        Roll::new(100, 1),
        Roll::new(20, 5)
    ];
    let results = roll(&rolls);
    result_printer::print(&results);
}
