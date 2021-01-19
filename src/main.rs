mod lib;

use std::fs::write;

use lib::Human;
use lib::Result;

fn main() -> Result<()> {
    let human = Human::new()?;
    write("datafile.csv", human.to_csv())?;
    Ok(())
}
