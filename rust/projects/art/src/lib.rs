//! # Art
//!
//! Some art stuff

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// Primary color
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    /// Secondary Color
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors
    pub fn mix(c1: PrimaryColor, c2:PrimaryColor) -> SecondaryColor{
        // --snip--
    }
}
