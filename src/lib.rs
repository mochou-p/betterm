// mochou-p/betterm/src/lib.rs

#![warn(missing_docs)]

//! the better terminal crate

/// ANSI, LUT, RGB colors
pub mod color;
/// reset the effects of [`color`]
pub mod reset;
/// convenience mechanism for colorful printing
pub mod styled_printer;
/// set and get terminal state, events
pub mod terminal;

