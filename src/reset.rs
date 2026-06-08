// mochou-p/betterm/src/reset.rs

use std::fmt::{self, Display, Formatter};


/// get [`ResetAll`]
pub fn       all() -> ResetAll       { ResetAll       }
/// get [`ResetFg`]
pub fn        fg() -> ResetFg        { ResetFg        }
/// get [`ResetBg`]
pub fn        bg() -> ResetBg        { ResetBg        }
/// get [`ResetUnderline`]
pub fn underline() -> ResetUnderline { ResetUnderline }

/// reset everything
#[derive(Clone, Copy, Debug)]
pub struct ResetAll;

/// reset the foreground color
#[derive(Clone, Copy, Debug)]
pub struct ResetFg;

/// reset the background color
#[derive(Clone, Copy, Debug)]
pub struct ResetBg;

/// reset the underline color
#[derive(Clone, Copy, Debug)]
pub struct ResetUnderline;

impl Display for ResetAll {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[0m")
    }
}

impl Display for ResetFg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[39m")
    }
}

impl Display for ResetBg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[49m")
    }
}

impl Display for ResetUnderline {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[59m")
    }
}
