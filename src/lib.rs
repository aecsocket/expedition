#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

//! A simple and universal rich text styling library, designed to be easily adapted to any output
//! format.
//!
//! Inspired by [KyoriPowered/adventure](https://github.com/KyoriPowered/adventure).
//! 
//! In many cases when user messages are involved (such as in games or user-editable text fields),
//! you may wish to add some rich text styling options such as color, or decorations such as bold
//! or italic. Although many libraries such as termcolor or egui already support styling text
//! with various options, they all use their own formats for doing so. This crate aims to provide
//! a universal format for transmitting and storing rich text messages, supporting a subset of
//! common features that other libraries contain.
//!
//! # Usage
//! 
//! The entry point of the library is [`Text`]:
//! 
//! ```
//! use expedition::prelude::*;
//! 
//! let text = Text::new("Hello, ")
//!     .with(Text::new("world!"))
//! ```
//! 
//! See the documentation of [`Text`] for usage info.
//! 
//! ## Feature flags
#![cfg_attr(feature = "document_features", doc = document_features::document_features!())]
//!
//! [`Text`]: expedition::text::Text

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
