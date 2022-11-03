use roller::roller;

mod argument_parser;
mod arguments;
mod result_printer;

fn main() {
    let args = arguments::args();
    let rolls = argument_parser::parse_arguments(args);
    let results = roller::roll(&rolls);
    result_printer::print(&results);
}
