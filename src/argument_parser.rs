use std::process;

use crate::arguments;
use roller::roller::Roll;

static ERR_MSG: &str = "Bad argument format. To find out more run 'roller --help";

pub fn parse_arguments(args: arguments::Args) -> Vec<Roll> {
    if args.rolls.is_empty() {
        exit()
    }
    args.rolls
        .into_iter()
        .map(|roll| {
            let values = roll.split("d").collect::<Vec<&str>>();

            if values.len() != 2 {
                exit()
            }
            let throws: u32 = {
                let throws = values[0];
                if throws.is_empty() {
                    1
                } else {
                    match throws.parse() {
                        Ok(val) => val,
                        Err(_) => exit()
                    }
                }
            };
            let dice: u32 = match values[1].parse() {
                Ok(val) => val,
                Err(_) => exit()
            };

            Roll::new(dice, throws)
        })
        .collect::<Vec<Roll>>()
}

fn exit() -> ! {
    eprintln!("{}", ERR_MSG);
    process::exit(1);
}
