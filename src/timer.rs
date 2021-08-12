pub struct Timer {
    pub clock: f32,
    pub period: f32,
    pub unit: TimeUnit,
}

impl Timer {
    pub fn new(clock: f32, period: f32, unit: TimeUnit) -> Self {
        Self {
            clock,
            period,
            unit,
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
}
