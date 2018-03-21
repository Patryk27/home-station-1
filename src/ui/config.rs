extern crate actix;

use actix::{Addr, Syn};
use config::Config;
use weather;

/// This struct defines a required configuration for the UI Actor.
pub struct Configuration {
    // UI Actor is dependent on the Weather Actor
    pub weather: Addr<Syn, weather::Actor>,

    // ... a LCD configuration won't hurt too
    pub lcd: LcdConfiguration,
}

pub struct LcdConfiguration {
    pub i2c_device: String,
    pub i2c_address: u16,
}

impl Configuration {
    pub fn new(weather: Addr<Syn, weather::Actor>, config: &mut Config) -> Configuration {
        Configuration {
            weather,

            lcd: LcdConfiguration {
                i2c_device: config.get("devices.lcd.i2c.device").unwrap(),
                i2c_address: config.get("devices.lcd.i2c.address").unwrap(),
            },
        }
    }
}