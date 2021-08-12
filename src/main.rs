mod error;
mod timer;
mod util;

use anyhow::Result;
use std::io::stdin;
use timer::{TimeUnit, Timer};

use crate::util::is_valid_period;

fn main() {
    let (clock, period, unit) = get_values().expect("Error reading input");
    let _timer = Timer::new(clock, period, unit);
}

fn get_values() -> Result<(f32, f32, TimeUnit)> {
    let mut clock = String::from("abc");

    println!("Clock speed: ");
    while !clock.chars().all(|c| c.is_numeric()) {
        clock.clear();
        stdin().read_line(&mut clock)?;
        clock.pop(); // Pop the newline character
    }

    let mut period_str = String::new();

    println!("Timer period: ");
    while is_valid_period(&period_str).is_none() {
        period_str.clear();
        stdin().read_line(&mut period_str)?;
        period_str.pop(); // Pop the newline character
    }

    let (period, unit) = is_valid_period(&period_str).unwrap();
    Ok((clock.parse::<f32>().unwrap(), period, unit))
}
