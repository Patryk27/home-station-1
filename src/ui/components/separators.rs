/// The UI's separator renderer.
///
/// It renders lines which separate all other components (how unexpected!).

use pwr_hd44780::BufferedLcd;
use pwr_hd44780::Hd44780;
use std::cell::RefCell;
use std::rc::Rc;
use super::right;
use utils::UnitResult;

pub struct Component {
    lcd: Rc<RefCell<BufferedLcd>>,
}

impl Component {
    pub fn new(lcd: Rc<RefCell<BufferedLcd>>) -> Component {
        Component {
            lcd,
        }
    }
}

impl super::Component for Component {
    fn render(&mut self) -> UnitResult {
        let mut lcd = self.lcd.borrow_mut();

        // display a vertical line between left & right components
        for y in 0..2 {
            lcd.print_char_at(y, right::LEFT_PADDING - 1, 0)?;
        }

        // display a horizontal line between left+right & bottom components
        for x in 0..20 {
            lcd.print_at(2, x, "-")?;
        }

        // display a special, superior character at the connection of both lines
        lcd.print_char_at(2, right::LEFT_PADDING - 1, 1)?;

        Ok(())
    }
}