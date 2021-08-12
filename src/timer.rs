use crate::{
    timer::TimeUnit::*,
    util::{parse_f32, split_str_num},
};

pub struct Timer {
    pub clock: f32,
    pub timer_period: f32,
    pub time_unit: TimeUnit,
}

impl Timer {
    pub fn new(clock: String, timer_values: String) -> Option<Self> {
        let clock = parse_f32(clock).expect("Could not turn clock frequency into f32");

        // Make sure that the timer period is a valid ascii string with no special charcters and
        // make sure the user's timer period begins with a numerical value before parsing it
        if timer_values.chars().all(|c| c.is_ascii_alphanumeric())
            && timer_values.len() > 0
            && timer_values.chars().nth(0).unwrap().is_numeric()
        {
            let (timer_period, time_unit_str) =
                split_str_num(&timer_values).expect("Could not parse input");

            let time_unit = match &time_unit_str[..] {
                "ms" => Milliseconds,
                "us" => Microseconds,
                "ns" => Nanoseconds,
                "s" => Seconds,
                _ => Error,
            };

            Some(Self {
                clock,
                timer_period: timer_period.parse::<f32>().unwrap(),
                time_unit,
            })
        } else {
            None
        }
    }

    pub fn _calculate_values(&self) -> (f32, f32) {
        (0f32, 0f32)
    }
}

pub enum TimeUnit {
    Milliseconds,
    Microseconds,
    Nanoseconds,
    Seconds,
    Error,
}
