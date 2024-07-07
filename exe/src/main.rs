use std::{env, error::Error};

use data::parse_data;

mod data;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();

    let data: String = std::fs::read_to_string("data.json")?;
    // let data: String = std::fs::read_to_string(args[1])?;
    let data = parse_data(data.as_bytes())?;

    Ok(())
}
