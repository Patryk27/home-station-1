#![feature(duration_extras)]

#[macro_use]
extern crate actix;
extern crate chrono;
extern crate config;
extern crate futures;
#[macro_use]
extern crate log;
extern crate pwr_airly;
extern crate pwr_hd44780;
extern crate simple_logger;

use actix::{Arbiter, System};

mod ui;
mod utils;
mod weather;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    info!("HomeStation, v. 0.0.1");
    info!("(C) 2018, by Patryk Wychowaniec");
    info!("");

    // -- create system -- //
    info!("[1/3] Creating system...");

    let system = System::new("home-station");

    // -- load configuration -- //
    info!("[2/3] Loading configuration...");

    let config = &mut config::Config::default();
    config.merge(config::File::with_name("config")).unwrap();

    // -- spawn actors -- //
    info!("[3/3] Spawning actors...");

    let weather_config = weather::Configuration::new(config);

    let weather = Arbiter::start(|_| {
        weather::Actor::new(
            weather_config
        )
    });

    let ui_config = ui::Configuration::new(weather, config);

    let _ui = Arbiter::start(|_| {
        ui::Actor::new(ui_config).unwrap()
    });

    // -- and, eventually, run the application! --
    info!("");
    info!("Ready - executing application!");
    info!("");

    system.run();
}
