use crate::arguments;
use roller::roller::Roll;

static ERR_MSG: &str = "Bad argument format. To find out more run 'roller --help";

pub fn parse_arguments(args: arguments::Args) -> Vec<Roll> {
    args
        .rolls
        .into_iter()
        .map(|roll| {
            let values = roll.split("d").collect::<Vec<&str>>();

            if values.len() != 2 { panic!("{}", ERR_MSG) }
            let throws: u32 = {
                let throws = values[0];
                if throws.is_empty() { 1 } else { throws.parse().expect(ERR_MSG) }
            };
            let dice: u32= values[1].parse().expect(ERR_MSG);

            Roll::new(dice, throws)
        })
        .collect::<Vec<Roll>>()
}
