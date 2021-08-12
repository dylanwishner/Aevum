use std::num::ParseFloatError;

use anyhow::Result;

pub fn _print_match(arr: u64, psc: u64) {
    println!("Prescaler: {}\tAuto-Reload: {}", psc, arr);
}

// Splits the timer period string into two parts: one for the numerical value, one for the unit of
// time. Ignores whitespace. For example: '5ms' would be split into ('5', 'ms').
pub fn split_str_num(s: &str) -> Result<(String, String)> {
    let mut num_end_index = 0;
    for (index, char) in s.chars().enumerate() {
        if !char.is_numeric() {
            num_end_index = index - 1;
        }
    }

    Ok((
        s[..num_end_index].trim().to_owned(),
        s[num_end_index..].trim().to_owned(),
    ))
}

pub fn parse_f32(n: String) -> Result<f32, ParseFloatError> {
    n.parse::<f32>()
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
