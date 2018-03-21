/// The right-aligned UI component.
///
/// Renders current temperature & humidity + prints the `E` letter when retrieves an error from the
/// Weather Actor.

use chrono::prelude::*;
use pwr_hd44780::BufferedLcd;
use pwr_hd44780::Hd44780;
use std::cell::RefCell;
use std::rc::Rc;
use super::State;
use utils::UnitResult;

// X-coordinate of the
pub(super) const LEFT_PADDING: usize = 10;
pub(super) const ERROR_LEFT_PADDING: usize = 19;

pub struct Component {
    lcd: Rc<RefCell<BufferedLcd>>,
    now: DateTime<Local>,
    state: Option<State>,
}

impl Component {
    pub fn new(lcd: Rc<RefCell<BufferedLcd>>) -> Component {
        Component {
            lcd,
            now: Local::now(),
            state: None,
        }
    }
}

impl super::Component for Component {
    fn update(&mut self, state: State) -> UnitResult {
        self.now = Local::now();
        self.state = Some(state);

        Ok(())
    }

    fn render(&mut self) -> UnitResult {
        let (mut lcd, now) = (self.lcd.borrow_mut(), self.now);

        match self.state {
            // -- if state's known -- //
            Some(
                State {
                    weather: Some(ref weather),
                    weather_updating: _,
                }
            ) => {
                // print temperature, if present
                lcd.move_at(0, LEFT_PADDING + 1)?;

                if let Some(x) = weather.temperature {
                    lcd.print(format!("{:.0} C", x))?;
                } else {
                    lcd.print("? C")?;
                }

                // print pressure, if present
                lcd.move_at(1, LEFT_PADDING + 1)?;

                if let Some(x) = weather.pressure {
                    lcd.print(format!("{:.0} hPa", x / 100.0))?;
                } else {
                    lcd.print("? hPa")?;
                }

                // print blinking status
                if now.timestamp_subsec_millis() % 1000 < 500 {
                    if weather.status.is_failed() {
                        lcd.print_at(0, ERROR_LEFT_PADDING, "E")?;
                    }
                }
            }

            // -- if no data is available yet --
            _ => (),
        }

        Ok(())
    }
}