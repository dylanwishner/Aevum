mod error;
mod timer;
mod util;

use anyhow::Result;
use std::io::{stdin, Read};
use timer::Timer;

fn main() {
    let (clock, timer_value) = get_values().expect("Error reading input");
    let _timer = Timer::new(clock, timer_value);
}

fn get_values() -> Result<(String, String)> {
    let mut clock = String::new();
    println!("Clock speed: ");
    stdin().read_line(&mut clock)?;

    let mut timer_value = String::new();
    println!("Timer period: ");
    stdin().read_line(&mut timer_value)?;

    Ok((clock, timer_value))
}
