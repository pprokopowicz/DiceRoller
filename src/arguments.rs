use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// List of rolls to be thrown.
    ///
    /// Format examples:
    ///
    /// d20 - 1 throw of 20-sided dice.
    ///
    /// 5d6 - 5 throws of 6-sided dice.
    ///
    /// 3d6 d10 2d20 - 3 throws of 6-sided dice, 1 roll of 10-sided dice, 2 rolls of 20-sided dice.
    pub rolls: Vec<String>,
}

pub fn args() -> Args {
    Args::parse()
}
