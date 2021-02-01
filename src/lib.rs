use std::{io, str::FromStr, time::SystemTime};

pub use anyhow::{anyhow, Context, Result};

/// Holds the values for Human
pub struct Human {
    name: String,
    sex: char,
    birth_day: u16,
    birth_month: u16,
    birth_year: u16,
}

impl Human {
    /// Create new instance of Human by getting the relevant information from stdin.
    /// Can fail if the sex is not entered correctly or birth day, month or year are not numbers
    pub fn new() -> Result<Self> {
        let human = Human {
            name: get_read_line("Zadaj meno:", &|meno: String| meno.split_whitespace().count() > 1)?,
            sex: get_read_line("Zadaj pohlavie (M/Z):")?, // TODO: Move to Into trait
            birth_day: get_read_line("Zadaj den narodania") // FIXME: rozdielny format
                .context("Zly format")?, // TODO: Move Error into generic function get_read_line()
            birth_month: get_read_line("Zadaj mesiac narodenia")
                .context("Zly format")?,
            birth_year: get_read_line("Zadaj rok narodenia")
                .context("Zly format")?,
        };
        // current year from EPOCH time
        let curr_year = SystemTime::now() // TODO: calculate real age with chrono
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs()
            / 31556926 // calculates year
            + 1970; // since EPOCH started in 1970
        if curr_year - (human.birth_year as u64) < 18 {
            return Err(anyhow!("Neplnolety, nemozeme pokracovat"));
        }
        Ok(human)
    }
    /// Converts the fields name, sex, birth_day, birth_month, birth_year to comma separated String
    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{}\n",
            self.name, self.sex, self.birth_day, self.birth_month, self.birth_year
        )
    }
}

/// Shows the prompt with a new newline and then requests user input from stdin.
// String, char, u8
// 
fn get_read_line<T, U>(prompt: &str, check: U) -> Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: Sync,
    <T as FromStr>::Err: Send,
    <T as FromStr>::Err: std::error::Error + 'static,
    U: Fn(T) -> bool,
{
    println!("{}", prompt);
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).unwrap();
    Ok(buff.trim().parse()?)
}