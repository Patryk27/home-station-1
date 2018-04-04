extern crate pwr_airly;

use actix::{AsyncContext, Context};
use pwr_airly::AirlyClient;
pub use self::config::Configuration;
pub use self::structs::*;
use std::time::Duration;
use std::time::Instant;

mod actor;
mod config;
pub mod messages;
pub mod structs;

pub struct Actor {
    config: Configuration,
    airly: AirlyClient,
    state: State,
}

impl Actor {
    pub fn new(config: Configuration) -> Actor {
        let mut state = State::default();

        if !config.is_set() {
            state.status = Status::Disabled;
        }

        let key = config.key.clone();

        Actor {
            config,
            state,
            airly: AirlyClient::new(key),
        }
    }

    fn process(&mut self, ctx: &mut Context<Self>) {
        let state = &mut self.state;

        match state.status {
            // -- uninitialized -- //
            Status::Uninitialized => {
                info!("Weather actor is uninitialized - requesting a refresh.");

                state.status = Status::RefreshRequested;
            }

            // -- refresh requested -- //
            Status::RefreshRequested => {
                info!("Weather actor is now refreshing.");
                info!("Changing status to: working.");

                info!("-> [in] sensor_id = {}", self.config.sensor_id);

                state.status = Status::Working {
                    since: Instant::now(),
                };

                match self.airly.get_sensor_measurements(self.config.sensor_id) {
                    Ok(measurements) => {
                        let current = measurements.current;

                        state.pm10 = current.pm10;
                        state.pm25 = current.pm25;
                        state.temperature = current.temperature;
                        state.pressure = current.pressure;
                        state.humidity = current.humidity;
                        state.air_quality_index = current.air_quality_index;

                        info!("-> [out] measurements.current = {:?}", current);
                        info!("Changing status to: ready.");

                        state.status = Status::Ready {
                            since: Instant::now(),
                        };
                    }

                    Err(err) => {
                        error!("Weather actor failed to refresh: {:?}", err);
                        error!("Changing status to: failed.");

                        state.status = Status::Failed {
                            since: Instant::now(),
                        };
                    }
                }
            }

            // -- ready -- //
            Status::Ready { since } => {
                if since.elapsed().as_secs() >= 5 * 60 {
                    info!("Weather actor contains old data - requesting a refresh.");

                    state.status = Status::RefreshRequested;
                }
            }

            // -- failed -- //
            Status::Failed { since } => {
                if since.elapsed().as_secs() >= 60 {
                    warn!("Weather actor has been in a failed state for some time - requesting a refresh.");

                    state.status = Status::RefreshRequested;
                }
            }

            _ => (),
        }

        ctx.run_later(Duration::from_millis(1000), |act, ctx| {
            act.process(ctx);
        });
    }
}