extern crate pwr_hd44780;

use pwr_hd44780::Hd44780;
use std::cell::RefCell;
use std::error;
use std::rc::Rc;
use super::Actor;
use super::components;
use ui::{Actors, State};
use ui::components::Component;
use ui::Configuration;

impl Actor {
    pub fn new(config: Configuration) -> Result<Actor, Box<error::Error>> {
        // create the LCD
        let mut lcd = create_lcd(
            &config
        )?;

        // initialize the LCD
        initialize_lcd(
            &mut lcd
        )?;

        // create the reference-counted pointer to the LCD
        let lcd_rc = Rc::new(
            RefCell::new(lcd)
        );

        // initialize the components
        let components = create_components(
            lcd_rc.clone()
        );

        Ok(
            Actor {
                actors: Actors {
                    weather: config.weather,
                },

                state: State {
                    weather: None,
                    weather_updating: false,
                },

                lcd: lcd_rc,
                components,
            }
        )
    }
}

/// Constructs a HD44780's LCD instance basing on given configuration.
fn create_lcd(config: &Configuration) -> Result<pwr_hd44780::BufferedLcd, Box<error::Error>> {
    // create the bus
    let lcd_bus = pwr_hd44780::I2CBus::new(
        config.lcd.i2c_device.clone(),
        config.lcd.i2c_address.clone(),
    ).expect("Failed to initialize the HD44780's bus - invalid I2C path / address?");

    // create the direct frontend
    let direct_lcd = pwr_hd44780::DirectLcd::new(
        Box::new(lcd_bus),
        20, 4,
    ).expect("Failed to create the HD44780's instance - invalid I2C path / address?");

    // create the buffered frontend
    pwr_hd44780::BufferedLcd::new(
        Box::new(direct_lcd)
    )
}

/// Initializes the previously created HD44780's LCD instance
/// (initially clears it & creates appropriate custom characters)
fn initialize_lcd(lcd: &mut pwr_hd44780::BufferedLcd) -> Result<(), Box<error::Error>> {
    lcd.clear()
        .expect("HD44780's clear() failed - unstable I2C connection?");

    // create the pipe character: |
    // the LCD's default one is a little shorter than LCD's line height which yields not-so-pretty
    // UI
    lcd.create_char(0, [
        0b00000100,
        0b00000100,
        0b00000100,
        0b00000100,
        0b00000100,
        0b00000100,
        0b00000100,
        0b00000100,
    ])?;

    // create a character used to concatenate the | and - characters in the UI
    lcd.create_char(1, [
        0b00000100,
        0b00000100,
        0b00000100,
        0b11111111,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
    ])?;

    Ok(())
}

/// The UI is composed of 4 sub-components (called just `components`); this function instantiates
/// them all.
fn create_components(lcd_rc: Rc<RefCell<pwr_hd44780::BufferedLcd>>) -> Vec<Box<Component>> {
    vec![
        Box::new(components::Bottom::new(lcd_rc.clone())),
        Box::new(components::Left::new(lcd_rc.clone())),
        Box::new(components::Right::new(lcd_rc.clone())),
        Box::new(components::Separators::new(lcd_rc.clone())),
    ]
}