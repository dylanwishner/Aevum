use crate::{error::AevumError, timer::TimeUnit};

pub fn _print_match(arr: u64, psc: u64) {
    println!("Prescaler: {}\tAuto-Reload: {}", psc, arr);
}

pub fn is_valid_period(period: &str) -> Option<(f32, TimeUnit)> {
    if period.chars().all(|c| c.is_ascii_alphanumeric())
        && period.len() > 0
        && period.chars().nth(0).unwrap().is_numeric()
    {
        let split_period = split_str_num(&period);

        if split_period.is_some() {
            let (time, unit) = split_period.unwrap();

            // Make sure the time is a valid f32 and the unit is a valid time unit
            let parsed_time = time.parse::<f32>();
            let parsed_unit = match_unit(unit);

            if parsed_time.is_ok() && parsed_unit.is_ok() {
                Some((parsed_time.unwrap(), parsed_unit.unwrap()))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

// Splits the timer period string into two parts: one for the numerical value, one for the unit of
// time. Ignores whitespace. For example: '5ms' would be split into ('5', 'ms').
fn split_str_num(s: &str) -> Option<(String, String)> {
    let mut num_end_index = 0;
    for (index, char) in s.chars().enumerate() {
        if !char.is_numeric() {
            num_end_index = index - 1;
        }
    }

    Some((
        s[..num_end_index].trim().to_owned(),
        s[num_end_index..].trim().to_owned(),
    ))
}

fn match_unit(unit: String) -> Result<TimeUnit, AevumError> {
    match &unit[..] {
        "ms" => Ok(TimeUnit::Milliseconds),
        "us" => Ok(TimeUnit::Microseconds),
        "ns" => Ok(TimeUnit::Nanoseconds),
        "s" => Ok(TimeUnit::Seconds),
        _ => Err(AevumError),
    }
}

#[cfg(test)]
mod test {
    use crate::util::split_str_num;

    #[test]
    fn test() {
        let s = "5ms";
        assert_eq!(
            split_str_num(s).unwrap(),
            ("5".to_string(), "ms".to_string())
        );
    }

    #[test]
    fn test_none() {
        let s = "5 ms";
        assert_eq!(
            split_str_num(s).unwrap(),
            ("5".to_string(), "ms".to_string())
        );
    }
}
