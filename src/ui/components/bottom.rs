/// The bottom-aligned UI component.
///
/// Renders periodically changing lines describing current weather status (like the PM10 level).

extern crate core;

use pwr_hd44780::BufferedLcd;
use pwr_hd44780::Hd44780;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use super::State;
use utils::UnitResult;

pub struct Component {
    lcd: Rc<RefCell<BufferedLcd>>,

    // List of lines displayed at the bottom of the screen
    lines: Vec<String>,

    // Latest date when lines were updated
    lines_updated_at: Instant,

    // Current line's index
    current_line_idx: usize,
}

impl Component {
    pub fn new(lcd: Rc<RefCell<BufferedLcd>>) -> Component {
        Component {
            lcd,

            lines: vec![],
            lines_updated_at: Instant::now(),

            current_line_idx: 0,
        }
    }
}

impl super::Component for Component {
    fn update(&mut self, state: State) -> UnitResult {
        let lines = &mut self.lines;

        // -- update lines -- //
        lines.clear();

        if let Some(weather) = state.weather {
            // match PM10
            if let Some(x) = weather.pm10 {
                lines.push(
                    // translation: `PM10 level: ...`
                    format!("Poz. PM10: {:.0} uG", x)
                );
            }

            // match PM2.5
            if let Some(x) = weather.pm25 {
                lines.push(
                    // translation: `PM2.5 level: ...`
                    format!("Poz. PM2.5: {:.0} uG", x)
                );
            }

            // match humidity
            if let Some(x) = weather.humidity {
                lines.push(
                    // translation: `Humidity: ...`
                    format!("Wilgotnosc: {:.0}%", x)
                );
            }

            // match air quality index
            if let Some(x) = weather.air_quality_index {
                let y = 100 - (x as i32);

                let txt = match y {
                    0 ... 15 => ":-(((",
                    15 ... 30 => ":-((",
                    30 ... 40 => ":-(",
                    40 ... 60 => ":-|",
                    60 ... 70 => ":-)",
                    70 ... 85 => ":-))",
                    85 ... 100 => ":-)))",
                    _ => "?",
                };

                lines.push(
                    // translation: `Overall state: ...`
                    format!("Ogolny stan: {:.0}", y)
                );

                lines.push(
                    // translation: `Overall state: ...`
                    format!("Ogolny stan: {}", txt)
                );
            }
        }

        // if no lines are present, conveniently render just "-", to notify user that there's no
        // data to render
        if lines.is_empty() {
            lines.push("-".to_string());
        }

        // if appropriate time's elapsed, increase the current line index
        if self.lines_updated_at.elapsed().as_secs() >= 3 {
            self.current_line_idx = (self.current_line_idx + 1) % lines.len();
            self.lines_updated_at = Instant::now();
        }

        Ok(())
    }

    fn render(&mut self) -> UnitResult {
        let mut lcd = self.lcd.borrow_mut();

        // fetch current line
        let line = self.lines[self.current_line_idx].clone();

        // determine a nice, centered `x` coordinate
        let x = (lcd.width() - line.len()) / 2;

        lcd.print_at(3, x, line)
    }
}