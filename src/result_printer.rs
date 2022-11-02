use std::vec;

use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};
use roller::roller::RollResult;

pub fn print(results: &Vec<RollResult>) {
    let table = results
        .iter()
        .map(|result| {
            let dice = format!("d{}", result.dice);
            let throws = result
                .throws
                .iter()
                .map(|throw| throw.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            vec![
                dice.cell().justify(Justify::Center),
                throws.cell().justify(Justify::Center),
                result.sum().cell().justify(Justify::Center),
            ]
        })
        .collect::<Vec<Vec<CellStruct>>>()
        .table()
        .title(vec![
            "Dice".cell().bold(true).justify(Justify::Center),
            "Throws".cell().bold(true).justify(Justify::Center),
            "Sum".cell().bold(true).justify(Justify::Center),
        ]);

    _ = print_stdout(table);
}
