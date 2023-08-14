#![feature(iter_intersperse)]

//include!("../README.md");

#[cfg(feature = "egui")]
pub mod egui;
#[cfg(feature = "termcolor")]
pub mod term;
pub mod text;
pub mod util;

pub mod prelude {
    pub use ecolor::Color32;

    pub use crate::text::{StyleState, Styleable, Text, TextBuilder, TextStyle};
    pub use crate::util::{StackFlattener, TextFlattener};
}
