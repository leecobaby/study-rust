//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        if c1 == PrimaryColor::Red && c2 == PrimaryColor::Yellow {
            SecondaryColor::Orange
        } else if c1 == PrimaryColor::Red && c2 == PrimaryColor::Blue {
            SecondaryColor::Purple
        } else if c1 == PrimaryColor::Yellow && c2 == PrimaryColor::Blue {
            SecondaryColor::Green
        } else {
            panic!("Can't mix {:?} and {:?}", c1, c2);
        }
    }
}

// use art::PrimaryColor;
// use art::SecondaryColor;
// use art::mix;
fn main() {
    let red = kinds::PrimaryColor::Red;
    let yellow = kinds::PrimaryColor::Yellow;
    utils::mix(red, yellow);
}
