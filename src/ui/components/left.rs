/// The left-aligned UI component.
///
/// Renders current time & date.

use chrono::prelude::*;
use pwr_hd44780::BufferedLcd;
use pwr_hd44780::Hd44780;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use super::State;
use utils::UnitResult;

pub struct Component {
    lcd: Rc<RefCell<BufferedLcd>>,
    now: DateTime<Local>,

    // Map mapping month number into its name
    month_names: HashMap<u32, &'static str>,
}

impl Component {
    pub fn new(lcd: Rc<RefCell<BufferedLcd>>) -> Component {
        Component {
            lcd,
            now: Local::now(),
            month_names: get_month_names(),
        }
    }

    fn render_time(&self) -> UnitResult {
        let (mut lcd, now) = (self.lcd.borrow_mut(), self.now);

        let separator = if now.second() % 2 == 0 { ':' } else { ' ' };

        lcd.print_at(
            0, 1,
            format!(
                "{:02}{}{:02}",
                now.hour(), separator, now.minute(),
            ),
        )
    }

    fn render_date(&self) -> UnitResult {
        let (mut lcd, now) = (self.lcd.borrow_mut(), self.now);

        lcd.print_at(
            1, 1,
            format!(
                "{:02} {}",
                now.day(), self.month_names[&now.month()]
            ),
        )
    }
}

impl super::Component for Component {
    fn update(&mut self, _: State) -> UnitResult {
        self.now = Local::now();

        Ok(())
    }

    fn render(&mut self) -> UnitResult {
        self.render_time()?;
        self.render_date()?;

        Ok(())
    }
}

/// Generate a map mapping month number into its name.
///
/// By `month name` I currently mean its roman numeral, but it's still easier to generate statically
/// rather than build arabic <-> roman converter.
fn get_month_names() -> HashMap<u32, &'static str> {
    let mut month_names = HashMap::new();

    month_names.insert(1, "I");
    month_names.insert(2, "II");
    month_names.insert(3, "III");

    month_names.insert(4, "IV");
    month_names.insert(5, "V");
    month_names.insert(6, "VI");

    month_names.insert(7, "VII");
    month_names.insert(8, "VIII");
    month_names.insert(9, "IX");

    month_names.insert(10, "X");
    month_names.insert(11, "XI");
    month_names.insert(12, "XII");

    month_names
}