use std::{convert::TryInto, io, str::FromStr, time::SystemTime};

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
        let curr_year = SystemTime::now() // TODO: calculate real age with chrono
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs()
            / 31556926 // calculates year
            + 1970; // since EPOCH started in 1970
        let human = Human {
            name: get_read_line(
                "Zadaj meno:",
                &|meno: &String| meno.split_whitespace().count() >= 2,
                "Musíš zadať aj prvé aj druhé meno",
            ),
            sex: get_read_line(
                "Zadaj pohlavie (M/Z):",
                &|pohlavie: &char| *pohlavie == 'M' || *pohlavie == 'Z',
                "Musíš zadať veľké M alebo Z",
            ),
            birth_day: get_read_line(
                "Zadaj den narodania",
                &|den: &u16| *den <= 31 && *den >= 1,
                "Deň musí byť medzi 1 a 31",
            ),
            birth_month: get_read_line(
                "Zadaj mesiac narodenia",
                &|mesiac: &u16| *mesiac <= 12 && *mesiac >= 1,
                "Mesiac musí byť medzi 1 a 12",
            ),
            birth_year: get_read_line(
                "Zadaj rok narodenia",
                &|rok: &u16| *rok > 1900 && *rok <= curr_year.try_into().unwrap(),
                &format!("Rok musí byť medzi 1901 a {}", curr_year),
            ),
        };
        // current year from EPOCH time
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

// get_read_line::<String>(prompt: &str, check: &dyn Fn(&T) -> bool) -> String
// get_read_line::<u16>(prompt: &str, check: &dyn Fn(&T) -> bool) -> y16
// get_read_line::<har>(prompt: &str, check: &dyn Fn(&T) -> bool) -> char

fn get_read_line<T>(prompt: &str, check: &dyn Fn(&T) -> bool, err_msg: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Sync,
    <T as FromStr>::Err: Send,
    <T as FromStr>::Err: std::error::Error + 'static,
{
    loop {
        println!("{}", prompt);
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).unwrap();
        let buff = buff.trim().parse(); // Ok(T), Err(T)
        if let Ok(buff) = buff {
            if check(&buff) {
                return buff;
            }
        }
        println!("{}\n", err_msg);
    }
}
