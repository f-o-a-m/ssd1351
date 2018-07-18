//! Raw mode for coercion into richer driver types
//!
//! A display driver instance without high level functionality used as a return type from the
//! builder. Used as a source to coerce the driver into richer modes like
//! [`GraphicsMode`](../graphics/index.html) and [`TerminalMode`](../terminal/index.html).

use interface::DisplayInterface;
use display::Display;

use mode::displaymode::DisplayModeTrait;

/// Raw display mode
pub struct RawMode<DI>
where
    DI: DisplayInterface,
{
    // TODO remove pub! - testing only!
    pub display: Display<DI>,
}

impl<DI> DisplayModeTrait<DI> for RawMode<DI>
where
    DI: DisplayInterface,
{
    /// Create new RawMode instance
    fn new(display: Display<DI>) -> Self {
        RawMode { display }
    }

    /// Release all resources used by RawMode
    fn release(self) -> Display<DI> {
        self.display
    }
}

impl<DI: DisplayInterface> RawMode<DI> {
    /// Create a new raw display mode
    pub fn new(display: Display<DI>) -> Self {
        RawMode { display }
    }
}
