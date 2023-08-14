#![feature(iter_intersperse)]
#![forbid(missing_docs)]

//! hi

#[cfg(feature = "egui")]
pub mod egui;
#[cfg(feature = "termcolor")]
pub mod term;
pub mod text;
pub mod util;

/// The essential types for using the library.
pub mod prelude {
    pub use ecolor::Color32;

    #[cfg(feature = "egui")]
    pub use crate::egui::StyleToFormat;
    pub use crate::text::{Styleable, Text, TextBuilder, TextStyle};
    pub use crate::util::{StackFlattener, TextFlattener};
}
