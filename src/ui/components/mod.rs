/// The UI is composed of four sub-components (called just `components`).
/// For convenience, they were named after their positions.

pub use self::bottom::Component as Bottom;
pub use self::left::Component as Left;
pub use self::right::Component as Right;
pub use self::separators::Component as Separators;
use super::State;
use utils::UnitResult;

mod bottom;
mod left;
mod right;
mod separators;

/// Defines a generic UI sub-component, for convenience named just `component`.
pub trait Component {
    fn update(&mut self, _state: State) -> UnitResult {
        Ok(()) // nottin' here
    }

    fn render(&mut self) -> UnitResult {
        Ok(()) // nottin' here
    }
}