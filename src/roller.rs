use rand::Rng;

#[derive(Debug)]
pub struct Roll {
    dice: u32,
    amount: u32,
}

impl Roll {
    pub fn new(dice: u32, amount: u32) -> Self {
        Roll { dice, amount }
    }
}

#[derive(Debug)]
pub struct RollResult {
    pub dice: u32,
    pub throws: Vec<u32>,
}

impl RollResult {
    fn new(dice: u32, throws: Vec<u32>) -> Self {
        RollResult { dice, throws }
    }

    pub fn sum(&self) -> u32 {
        self.throws
            .clone()
            .into_iter()
            .reduce(|a, b| a + b)
            .unwrap_or(0)
    }
}

pub fn roll(rolls: &Vec<Roll>) -> Vec<RollResult> {
    let mut rng = rand::thread_rng();

    rolls
        .into_iter()
        .map(|roll| {
            let rolls = (0..roll.amount)
                .into_iter()
                .map(|_| rng.gen_range(1..=roll.dice))
                .collect::<Vec<u32>>();

            RollResult::new(roll.dice, rolls)
        })
        .collect::<Vec<RollResult>>()
}
