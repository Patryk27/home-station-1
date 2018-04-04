use std::time::Instant;

#[derive(Clone, Debug, Default)]
pub struct State {
    pub status: Status,

    // PM10 level, unit: uG
    pub pm10: Option<f32>,

    // PM2.5 level, unit: uG
    pub pm25: Option<f32>,

    // temperature, unit: Celsius degrees
    pub temperature: Option<f32>,

    // pressure, unit: Pa
    pub pressure: Option<f32>,

    // humidity, unit: % (percentage)
    pub humidity: Option<f32>,

    // air quality index, CAQI
    pub air_quality_index: Option<f32>,
}

#[derive(Clone, Debug, Message, PartialEq)]
pub enum Status {
    Disabled,

    Uninitialized,
    RefreshRequested,

    Working {
        since: Instant,
    },

    Ready {
        since: Instant,
    },

    Failed {
        since: Instant,
    },
}

impl Status {
    pub fn is_failed(&self) -> bool {
        match *self {
            Status::Failed { since: _ } => true,
            _ => false,
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Uninitialized
    }
}