// mochou-p/betterm/src/styled_printer.rs

use std::io::{self, Write};
use crate::color::{Color, fg, bg, underline};


/// accumulates styled contents (colors and text) for later printing.
/// all methods return the [`StyledPrinter`], so you can chain them
#[derive(Default, Clone)]
pub struct StyledPrinter {
    commands: Vec<Command>,
    stop:     Option<usize>
}

#[derive(Clone)]
enum Command {
    Text(String),
    FgColor(String),
    BgColor(String),
    UnderlineColor(String),
    PopFg,
    PopBg,
    PopUnderline,
    ResetAll,
    ResetFg,
    ResetBg,
    ResetUnderline
}

impl StyledPrinter {
    /// append text to the styled contents
    pub fn text(mut self, text: impl AsRef<str>) -> Self {
        self.commands.push(Command::Text(String::from(text.as_ref())));
        self
    }

    /// push to the foreground color stack
    pub fn push_fg(mut self, color: impl Color) -> Self {
        self.commands.push(Command::FgColor(fg(color)));
        self
    }

    /// push to the background color stack
    pub fn push_bg(mut self, color: impl Color) -> Self {
        self.commands.push(Command::BgColor(bg(color)));
        self
    }

    /// push to the underline color stack
    pub fn push_underline(mut self, color: impl Color) -> Self {
        self.commands.push(Command::UnderlineColor(underline(color)));
        self
    }

    /// pop the foreground color stack
    pub fn pop_fg(mut self) -> Self {
        self.commands.push(Command::PopFg);
        self
    }

    /// pop the background color stack
    pub fn pop_bg(mut self) -> Self {
        self.commands.push(Command::PopBg);
        self
    }

    /// pop the underline color stack
    pub fn pop_underline(mut self) -> Self {
        self.commands.push(Command::PopUnderline);
        self
    }

    /// everything inside the closure uses this foreground color
    pub fn with_fg(self, color: impl Color, f: impl FnOnce(Self) -> Self) -> Self {
        f(self.push_fg(color)).pop_fg()
    }

    /// everything inside the closure uses this background color
    pub fn with_bg(self, color: impl Color, f: impl FnOnce(Self) -> Self) -> Self {
        f(self.push_bg(color)).pop_bg()
    }

    /// everything inside the closure uses this underline color
    pub fn with_underline(self, color: impl Color, f: impl FnOnce(Self) -> Self) -> Self {
        f(self.push_underline(color)).pop_underline()
    }

    /// use this foregroud color for the text
    pub fn fg(self, color: impl Color, text: impl AsRef<str>) -> Self {
        self.with_fg(color, |stpr| stpr.text(text))
    }

    /// use this backgroud color for the text
    pub fn bg(self, color: impl Color, text: impl AsRef<str>) -> Self {
        self.with_bg(color, |stpr| stpr.text(text))
    }

    /// use this underline color for the text
    pub fn underline(self, color: impl Color, text: impl AsRef<str>) -> Self {
        self.with_underline(color, |stpr| stpr.text(text))
    }

    /// pops all pushed colors
    pub fn reset_all(mut self) -> Self {
        self.commands.push(Command::ResetAll);
        self
    }

    /// completely returns the terminal style to default
    pub fn reset_fg(mut self) -> Self {
        self.commands.push(Command::ResetFg);
        self
    }

    /// pops all pushed background colors
    pub fn reset_bg(mut self) -> Self {
        self.commands.push(Command::ResetBg);
        self
    }

    /// pops all pushed underline colors
    pub fn reset_underline(mut self) -> Self {
        self.commands.push(Command::ResetUnderline);
        self
    }

    /// removes all accumulated contents
    pub fn clear(mut self) -> Self {
        self.commands.clear();
        self
    }

    /// set an optional horizontal limit for the next prints (assumes no newlines and no cursor movement)
    pub fn stop_after(mut self, n: Option<usize>) -> Self {
        self.stop = n;
        self
    }

    /// print the accumulated styled contents (write but not flush)
    pub fn write_all(self, output: &mut impl Write) -> io::Result<Self> {
        let mut            count = 0;
        let mut        fg_colors = vec![];
        let mut        bg_colors = vec![];
        let mut underline_colors = vec![];

        for command in self.commands.iter() {
            match command {
                Command::Text(text) => {
                    let len = text.chars().count();
                    count += len;

                    if let Some(stop) = self.stop {
                        if count > stop {
                            output.write_all(&text.as_bytes()[..len - (count - stop)])?;
                            break;
                        } else if count == stop {
                            output.write_all(text.as_bytes())?;
                            break;
                        }
                    }

                    output.write_all(text.as_bytes())?;
                },
                Command::FgColor(color) => {
                    output.write_all(color.as_bytes())?;
                    fg_colors.push(color);
                },
                Command::BgColor(color) => {
                    output.write_all(color.as_bytes())?;
                    bg_colors.push(color);
                },
                Command::UnderlineColor(color) => {
                    output.write_all(color.as_bytes())?;
                    underline_colors.push(color);
                },
                Command::PopFg => {
                    if fg_colors.is_empty() { continue; }
                    fg_colors.pop();

                    if let Some(last) = fg_colors.last() {
                        output.write_all(last.as_bytes())?;
                    } else {
                        output.write_all(b"\x1b[39m")?;
                    }
                },
                Command::PopBg => {
                    if bg_colors.is_empty() { continue; }
                    bg_colors.pop();

                    if let Some(last) = bg_colors.last() {
                        output.write_all(last.as_bytes())?;
                    } else {
                        output.write_all(b"\x1b[49m")?;
                    }
                },
                Command::PopUnderline => {
                    if underline_colors.is_empty() { continue; }
                    underline_colors.pop();

                    if let Some(last) = underline_colors.last() {
                        output.write_all(last.as_bytes())?;
                    } else {
                        output.write_all(b"\x1b[59m")?;
                    }
                },
                Command::ResetAll => {
                    output.write_all(b"\x1b[0m")?;
                    fg_colors       .clear();
                    bg_colors       .clear();
                    underline_colors.clear();
                },
                Command::ResetFg => {
                    output.write_all(b"\x1b[39m")?;
                    fg_colors.clear();
                },
                Command::ResetBg => {
                    output.write_all(b"\x1b[49m")?;
                    bg_colors.clear();
                },
                Command::ResetUnderline => {
                    output.write_all(b"\x1b[59m")?;
                    underline_colors.clear();
                }
            }
        }

        output.write_all(b"\x1b[0m")?;
        Ok(self)
    }
}

