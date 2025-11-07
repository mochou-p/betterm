// #![warn(missing_docs)]

//! [betterm](crate) is a better terminal crate
//!
//! its simple!
//! ```rust
//! use betterm::*;
//!
//! println!(
//!     "{}{}Hello world!{}",
//!     color::FG_GREEN, style::INTENSITY_BOLD,
//!     RESET_ALL
//! );
//! ```

/// reset the effects of [color] and [style]
pub const RESET_ALL: &str = "\x1b[0m";

/// characters like newline, tabs, ...
pub mod basic {
    /// used as the start of ANSI codes
    pub const ESCAPE:          char = '\x1b';

    // pub const BELL:            char = '\a';

    // pub const BACKSPACE:       char = '\b';

    /// new line (behaves differently in raw mode)
    pub const LINE_FEED:       char = '\n';

    /// move the cursor to the first column
    pub const CARRIAGE_RETURN: char = '\r';

    /// snap cursor to the next tabstop
    pub const HORIZONTAL_TAB:  char = '\t';

    // pub const   VERTICAL_TAB:  char = '\v';
}

/// erase text in the terminal
pub mod clear {
    /// clear every line under cursor, and the current line right of cursor
    pub const       SCREEN_UNDER_CURSOR:  &str = "\x1b[0J";

    /// clear every line above cursor, and the current line left of cursor
    pub const       SCREEN_ABOVE_CURSOR:  &str = "\x1b[1J";

    /// clear every line
    pub const WHOLE_SCREEN:               &str = "\x1b[2J";

    /// clear the scrollback, a.k.a. the visual history of the terminal
    pub const SCROLLBACK_BUFFER:          &str = "\x1b[3J";

    /// clear the current line right of cursor
    pub const       LINE_RIGHT_OF_CURSOR: &str = "\x1b[0K";

    /// clear the current line left of cursor
    pub const       LINE_LEFT_OF_CURSOR:  &str = "\x1b[1K";

    /// clear the current line
    pub const WHOLE_LINE:                 &str = "\x1b[2K";
}

/// ANSI, 256 lookup table, RGB
pub mod color {
    use std::fmt;

    /// reset the effects of FG_*
    pub const UNSET_FG:                 &str  = "\x1b[39m";

    /// reset the effects of BG_*
    pub const UNSET_BG:                 &str  = "\x1b[49m";

    /// reset the effects of UNDERLINE_*
    pub const UNSET_UNDERLINE:          &str  = "\x1b[59m";

    pub const FG_BLACK:                 &str  = "\x1b[30m";
    pub const FG_RED:                   &str  = "\x1b[31m";
    pub const FG_GREEN:                 &str  = "\x1b[32m";
    pub const FG_YELLOW:                &str  = "\x1b[33m";
    pub const FG_BLUE:                  &str  = "\x1b[34m";
    pub const FG_MAGENTA:               &str  = "\x1b[35m";
    pub const FG_CYAN:                  &str  = "\x1b[36m";
    pub const FG_WHITE:                 &str  = "\x1b[37m";

    pub const BG_BLACK:                 &str  = "\x1b[40m";
    pub const BG_RED:                   &str  = "\x1b[41m";
    pub const BG_GREEN:                 &str  = "\x1b[42m";
    pub const BG_YELLOW:                &str  = "\x1b[43m";
    pub const BG_BLUE:                  &str  = "\x1b[44m";
    pub const BG_MAGENTA:               &str  = "\x1b[45m";
    pub const BG_CYAN:                  &str  = "\x1b[46m";
    pub const BG_WHITE:                 &str  = "\x1b[47m";

    pub const UNDERLINE_BLACK:          &str  = "\x1b[50m";
    pub const UNDERLINE_RED:            &str  = "\x1b[51m";
    pub const UNDERLINE_GREEN:          &str  = "\x1b[52m";
    pub const UNDERLINE_YELLOW:         &str  = "\x1b[53m";
    pub const UNDERLINE_BLUE:           &str  = "\x1b[54m";
    pub const UNDERLINE_MAGENTA:        &str  = "\x1b[55m";
    pub const UNDERLINE_CYAN:           &str  = "\x1b[56m";
    pub const UNDERLINE_WHITE:          &str  = "\x1b[57m";

    pub const FG_BRIGHT_BLACK:          &str  = "\x1b[90m";
    pub const FG_BRIGHT_RED:            &str  = "\x1b[91m";
    pub const FG_BRIGHT_GREEN:          &str  = "\x1b[92m";
    pub const FG_BRIGHT_YELLOW:         &str  = "\x1b[93m";
    pub const FG_BRIGHT_BLUE:           &str  = "\x1b[94m";
    pub const FG_BRIGHT_MAGENTA:        &str  = "\x1b[95m";
    pub const FG_BRIGHT_CYAN:           &str  = "\x1b[96m";
    pub const FG_BRIGHT_WHITE:          &str  = "\x1b[97m";

    pub const BG_BRIGHT_BLACK:          &str  = "\x1b[100m";
    pub const BG_BRIGHT_RED:            &str  = "\x1b[101m";
    pub const BG_BRIGHT_GREEN:          &str  = "\x1b[102m";
    pub const BG_BRIGHT_YELLOW:         &str  = "\x1b[103m";
    pub const BG_BRIGHT_BLUE:           &str  = "\x1b[104m";
    pub const BG_BRIGHT_MAGENTA:        &str  = "\x1b[105m";
    pub const BG_BRIGHT_CYAN:           &str  = "\x1b[106m";
    pub const BG_BRIGHT_WHITE:          &str  = "\x1b[107m";

    pub const UNDERLINE_BRIGHT_BLACK:   &str  = "\x1b[110m";
    pub const UNDERLINE_BRIGHT_RED:     &str  = "\x1b[111m";
    pub const UNDERLINE_BRIGHT_GREEN:   &str  = "\x1b[112m";
    pub const UNDERLINE_BRIGHT_YELLOW:  &str  = "\x1b[113m";
    pub const UNDERLINE_BRIGHT_BLUE:    &str  = "\x1b[114m";
    pub const UNDERLINE_BRIGHT_MAGENTA: &str  = "\x1b[115m";
    pub const UNDERLINE_BRIGHT_CYAN:    &str  = "\x1b[116m";
    pub const UNDERLINE_BRIGHT_WHITE:   &str  = "\x1b[117m";

    /// the amount of [web-safe colors](https://en.wikipedia.org/wiki/Web_colors#Web-safe_colors) in each array
    pub const WEB_SAFE_COLORS_LEN:      usize = u8::MAX as usize + 1;

    // TODO: generate these instead of hardcode, when rust macros git gud
    /// foregroud [web-safe colors](https://en.wikipedia.org/wiki/Web_colors#Web-safe_colors) array
    pub static FG_WEB_SAFE_COLORS: [&str; WEB_SAFE_COLORS_LEN] = [
        // standard colors
        "\x1b[38;5;0m",   // black
        "\x1b[38;5;1m",   // red
        "\x1b[38;5;2m",   // green
        "\x1b[38;5;3m",   // yellow
        "\x1b[38;5;4m",   // blue
        "\x1b[38;5;5m",   // magenta
        "\x1b[38;5;6m",   // cyan
        "\x1b[38;5;7m",   // white

        // high-intensity colors
        "\x1b[38;5;8m",   // bright black
        "\x1b[38;5;9m",   // bright red
        "\x1b[38;5;10m",  // bright green
        "\x1b[38;5;11m",  // bright yellow
        "\x1b[38;5;12m",  // bright blue
        "\x1b[38;5;13m",  // bright magenta
        "\x1b[38;5;14m",  // bright cyan
        "\x1b[38;5;15m",  // bright white

        // 216 colors
        "\x1b[38;5;16m",
        "\x1b[38;5;17m",
        "\x1b[38;5;18m",
        "\x1b[38;5;19m",
        "\x1b[38;5;20m",
        "\x1b[38;5;21m",
        "\x1b[38;5;22m",
        "\x1b[38;5;23m",
        "\x1b[38;5;24m",
        "\x1b[38;5;25m",
        "\x1b[38;5;26m",
        "\x1b[38;5;27m",
        "\x1b[38;5;28m",
        "\x1b[38;5;29m",
        "\x1b[38;5;30m",
        "\x1b[38;5;31m",
        "\x1b[38;5;32m",
        "\x1b[38;5;33m",
        "\x1b[38;5;34m",
        "\x1b[38;5;35m",
        "\x1b[38;5;36m",
        "\x1b[38;5;37m",
        "\x1b[38;5;38m",
        "\x1b[38;5;39m",
        "\x1b[38;5;40m",
        "\x1b[38;5;41m",
        "\x1b[38;5;42m",
        "\x1b[38;5;43m",
        "\x1b[38;5;44m",
        "\x1b[38;5;45m",
        "\x1b[38;5;46m",
        "\x1b[38;5;47m",
        "\x1b[38;5;48m",
        "\x1b[38;5;49m",
        "\x1b[38;5;50m",
        "\x1b[38;5;51m",
        "\x1b[38;5;52m",
        "\x1b[38;5;53m",
        "\x1b[38;5;54m",
        "\x1b[38;5;55m",
        "\x1b[38;5;56m",
        "\x1b[38;5;57m",
        "\x1b[38;5;58m",
        "\x1b[38;5;59m",
        "\x1b[38;5;60m",
        "\x1b[38;5;61m",
        "\x1b[38;5;62m",
        "\x1b[38;5;63m",
        "\x1b[38;5;64m",
        "\x1b[38;5;65m",
        "\x1b[38;5;66m",
        "\x1b[38;5;67m",
        "\x1b[38;5;68m",
        "\x1b[38;5;69m",
        "\x1b[38;5;70m",
        "\x1b[38;5;71m",
        "\x1b[38;5;72m",
        "\x1b[38;5;73m",
        "\x1b[38;5;74m",
        "\x1b[38;5;75m",
        "\x1b[38;5;76m",
        "\x1b[38;5;77m",
        "\x1b[38;5;78m",
        "\x1b[38;5;79m",
        "\x1b[38;5;80m",
        "\x1b[38;5;81m",
        "\x1b[38;5;82m",
        "\x1b[38;5;83m",
        "\x1b[38;5;84m",
        "\x1b[38;5;85m",
        "\x1b[38;5;86m",
        "\x1b[38;5;87m",
        "\x1b[38;5;88m",
        "\x1b[38;5;89m",
        "\x1b[38;5;90m",
        "\x1b[38;5;91m",
        "\x1b[38;5;92m",
        "\x1b[38;5;93m",
        "\x1b[38;5;94m",
        "\x1b[38;5;95m",
        "\x1b[38;5;96m",
        "\x1b[38;5;97m",
        "\x1b[38;5;98m",
        "\x1b[38;5;99m",
        "\x1b[38;5;100m",
        "\x1b[38;5;101m",
        "\x1b[38;5;102m",
        "\x1b[38;5;103m",
        "\x1b[38;5;104m",
        "\x1b[38;5;105m",
        "\x1b[38;5;106m",
        "\x1b[38;5;107m",
        "\x1b[38;5;108m",
        "\x1b[38;5;109m",
        "\x1b[38;5;110m",
        "\x1b[38;5;111m",
        "\x1b[38;5;112m",
        "\x1b[38;5;113m",
        "\x1b[38;5;114m",
        "\x1b[38;5;115m",
        "\x1b[38;5;116m",
        "\x1b[38;5;117m",
        "\x1b[38;5;118m",
        "\x1b[38;5;119m",
        "\x1b[38;5;120m",
        "\x1b[38;5;121m",
        "\x1b[38;5;122m",
        "\x1b[38;5;123m",
        "\x1b[38;5;124m",
        "\x1b[38;5;125m",
        "\x1b[38;5;126m",
        "\x1b[38;5;127m",
        "\x1b[38;5;128m",
        "\x1b[38;5;129m",
        "\x1b[38;5;130m",
        "\x1b[38;5;131m",
        "\x1b[38;5;132m",
        "\x1b[38;5;133m",
        "\x1b[38;5;134m",
        "\x1b[38;5;135m",
        "\x1b[38;5;136m",
        "\x1b[38;5;137m",
        "\x1b[38;5;138m",
        "\x1b[38;5;139m",
        "\x1b[38;5;140m",
        "\x1b[38;5;141m",
        "\x1b[38;5;142m",
        "\x1b[38;5;143m",
        "\x1b[38;5;144m",
        "\x1b[38;5;145m",
        "\x1b[38;5;146m",
        "\x1b[38;5;147m",
        "\x1b[38;5;148m",
        "\x1b[38;5;149m",
        "\x1b[38;5;150m",
        "\x1b[38;5;151m",
        "\x1b[38;5;152m",
        "\x1b[38;5;153m",
        "\x1b[38;5;154m",
        "\x1b[38;5;155m",
        "\x1b[38;5;156m",
        "\x1b[38;5;157m",
        "\x1b[38;5;158m",
        "\x1b[38;5;159m",
        "\x1b[38;5;160m",
        "\x1b[38;5;161m",
        "\x1b[38;5;162m",
        "\x1b[38;5;163m",
        "\x1b[38;5;164m",
        "\x1b[38;5;165m",
        "\x1b[38;5;166m",
        "\x1b[38;5;167m",
        "\x1b[38;5;168m",
        "\x1b[38;5;169m",
        "\x1b[38;5;170m",
        "\x1b[38;5;171m",
        "\x1b[38;5;172m",
        "\x1b[38;5;173m",
        "\x1b[38;5;174m",
        "\x1b[38;5;175m",
        "\x1b[38;5;176m",
        "\x1b[38;5;177m",
        "\x1b[38;5;178m",
        "\x1b[38;5;179m",
        "\x1b[38;5;180m",
        "\x1b[38;5;181m",
        "\x1b[38;5;182m",
        "\x1b[38;5;183m",
        "\x1b[38;5;184m",
        "\x1b[38;5;185m",
        "\x1b[38;5;186m",
        "\x1b[38;5;187m",
        "\x1b[38;5;188m",
        "\x1b[38;5;189m",
        "\x1b[38;5;190m",
        "\x1b[38;5;191m",
        "\x1b[38;5;192m",
        "\x1b[38;5;193m",
        "\x1b[38;5;194m",
        "\x1b[38;5;195m",
        "\x1b[38;5;196m",
        "\x1b[38;5;197m",
        "\x1b[38;5;198m",
        "\x1b[38;5;199m",
        "\x1b[38;5;200m",
        "\x1b[38;5;201m",
        "\x1b[38;5;202m",
        "\x1b[38;5;203m",
        "\x1b[38;5;204m",
        "\x1b[38;5;205m",
        "\x1b[38;5;206m",
        "\x1b[38;5;207m",
        "\x1b[38;5;208m",
        "\x1b[38;5;209m",
        "\x1b[38;5;210m",
        "\x1b[38;5;211m",
        "\x1b[38;5;212m",
        "\x1b[38;5;213m",
        "\x1b[38;5;214m",
        "\x1b[38;5;215m",
        "\x1b[38;5;216m",
        "\x1b[38;5;217m",
        "\x1b[38;5;218m",
        "\x1b[38;5;219m",
        "\x1b[38;5;220m",
        "\x1b[38;5;221m",
        "\x1b[38;5;222m",
        "\x1b[38;5;223m",
        "\x1b[38;5;224m",
        "\x1b[38;5;225m",
        "\x1b[38;5;226m",
        "\x1b[38;5;227m",
        "\x1b[38;5;228m",
        "\x1b[38;5;229m",
        "\x1b[38;5;230m",
        "\x1b[38;5;231m",

        // greyscale colors (gradient)
        "\x1b[38;5;232m", // black
        "\x1b[38;5;233m",
        "\x1b[38;5;234m",
        "\x1b[38;5;235m",
        "\x1b[38;5;236m",
        "\x1b[38;5;237m",
        "\x1b[38;5;238m",
        "\x1b[38;5;239m",
        "\x1b[38;5;240m",
        "\x1b[38;5;241m",
        "\x1b[38;5;242m",
        "\x1b[38;5;243m",
        "\x1b[38;5;244m",
        "\x1b[38;5;245m",
        "\x1b[38;5;246m",
        "\x1b[38;5;247m",
        "\x1b[38;5;248m",
        "\x1b[38;5;249m",
        "\x1b[38;5;250m",
        "\x1b[38;5;251m",
        "\x1b[38;5;252m",
        "\x1b[38;5;253m",
        "\x1b[38;5;254m",
        "\x1b[38;5;255m"  // white
    ];

    // TODO: generate these instead of hardcode, when rust macros git gud
    /// backgroud [web-safe colors](https://en.wikipedia.org/wiki/Web_colors#Web-safe_colors) array
    pub static BG_WEB_SAFE_COLORS: [&str; WEB_SAFE_COLORS_LEN] = [
        // standard colors
        "\x1b[48;5;0m",   // black
        "\x1b[48;5;1m",   // red
        "\x1b[48;5;2m",   // green
        "\x1b[48;5;3m",   // yellow
        "\x1b[48;5;4m",   // blue
        "\x1b[48;5;5m",   // magenta
        "\x1b[48;5;6m",   // cyan
        "\x1b[48;5;7m",   // white

        // high-intensity colors
        "\x1b[48;5;8m",   // bright black
        "\x1b[48;5;9m",   // bright red
        "\x1b[48;5;10m",  // bright green
        "\x1b[48;5;11m",  // bright yellow
        "\x1b[48;5;12m",  // bright blue
        "\x1b[48;5;13m",  // bright magenta
        "\x1b[48;5;14m",  // bright cyan
        "\x1b[48;5;15m",  // bright white

        // 216 colors
        "\x1b[48;5;16m",
        "\x1b[48;5;17m",
        "\x1b[48;5;18m",
        "\x1b[48;5;19m",
        "\x1b[48;5;20m",
        "\x1b[48;5;21m",
        "\x1b[48;5;22m",
        "\x1b[48;5;23m",
        "\x1b[48;5;24m",
        "\x1b[48;5;25m",
        "\x1b[48;5;26m",
        "\x1b[48;5;27m",
        "\x1b[48;5;28m",
        "\x1b[48;5;29m",
        "\x1b[48;5;30m",
        "\x1b[48;5;31m",
        "\x1b[48;5;32m",
        "\x1b[48;5;33m",
        "\x1b[48;5;34m",
        "\x1b[48;5;35m",
        "\x1b[48;5;36m",
        "\x1b[48;5;37m",
        "\x1b[48;5;38m",
        "\x1b[48;5;39m",
        "\x1b[48;5;40m",
        "\x1b[48;5;41m",
        "\x1b[48;5;42m",
        "\x1b[48;5;43m",
        "\x1b[48;5;44m",
        "\x1b[48;5;45m",
        "\x1b[48;5;46m",
        "\x1b[48;5;47m",
        "\x1b[48;5;48m",
        "\x1b[48;5;49m",
        "\x1b[48;5;50m",
        "\x1b[48;5;51m",
        "\x1b[48;5;52m",
        "\x1b[48;5;53m",
        "\x1b[48;5;54m",
        "\x1b[48;5;55m",
        "\x1b[48;5;56m",
        "\x1b[48;5;57m",
        "\x1b[48;5;58m",
        "\x1b[48;5;59m",
        "\x1b[48;5;60m",
        "\x1b[48;5;61m",
        "\x1b[48;5;62m",
        "\x1b[48;5;63m",
        "\x1b[48;5;64m",
        "\x1b[48;5;65m",
        "\x1b[48;5;66m",
        "\x1b[48;5;67m",
        "\x1b[48;5;68m",
        "\x1b[48;5;69m",
        "\x1b[48;5;70m",
        "\x1b[48;5;71m",
        "\x1b[48;5;72m",
        "\x1b[48;5;73m",
        "\x1b[48;5;74m",
        "\x1b[48;5;75m",
        "\x1b[48;5;76m",
        "\x1b[48;5;77m",
        "\x1b[48;5;78m",
        "\x1b[48;5;79m",
        "\x1b[48;5;80m",
        "\x1b[48;5;81m",
        "\x1b[48;5;82m",
        "\x1b[48;5;83m",
        "\x1b[48;5;84m",
        "\x1b[48;5;85m",
        "\x1b[48;5;86m",
        "\x1b[48;5;87m",
        "\x1b[48;5;88m",
        "\x1b[48;5;89m",
        "\x1b[48;5;90m",
        "\x1b[48;5;91m",
        "\x1b[48;5;92m",
        "\x1b[48;5;93m",
        "\x1b[48;5;94m",
        "\x1b[48;5;95m",
        "\x1b[48;5;96m",
        "\x1b[48;5;97m",
        "\x1b[48;5;98m",
        "\x1b[48;5;99m",
        "\x1b[48;5;100m",
        "\x1b[48;5;101m",
        "\x1b[48;5;102m",
        "\x1b[48;5;103m",
        "\x1b[48;5;104m",
        "\x1b[48;5;105m",
        "\x1b[48;5;106m",
        "\x1b[48;5;107m",
        "\x1b[48;5;108m",
        "\x1b[48;5;109m",
        "\x1b[48;5;110m",
        "\x1b[48;5;111m",
        "\x1b[48;5;112m",
        "\x1b[48;5;113m",
        "\x1b[48;5;114m",
        "\x1b[48;5;115m",
        "\x1b[48;5;116m",
        "\x1b[48;5;117m",
        "\x1b[48;5;118m",
        "\x1b[48;5;119m",
        "\x1b[48;5;120m",
        "\x1b[48;5;121m",
        "\x1b[48;5;122m",
        "\x1b[48;5;123m",
        "\x1b[48;5;124m",
        "\x1b[48;5;125m",
        "\x1b[48;5;126m",
        "\x1b[48;5;127m",
        "\x1b[48;5;128m",
        "\x1b[48;5;129m",
        "\x1b[48;5;130m",
        "\x1b[48;5;131m",
        "\x1b[48;5;132m",
        "\x1b[48;5;133m",
        "\x1b[48;5;134m",
        "\x1b[48;5;135m",
        "\x1b[48;5;136m",
        "\x1b[48;5;137m",
        "\x1b[48;5;138m",
        "\x1b[48;5;139m",
        "\x1b[48;5;140m",
        "\x1b[48;5;141m",
        "\x1b[48;5;142m",
        "\x1b[48;5;143m",
        "\x1b[48;5;144m",
        "\x1b[48;5;145m",
        "\x1b[48;5;146m",
        "\x1b[48;5;147m",
        "\x1b[48;5;148m",
        "\x1b[48;5;149m",
        "\x1b[48;5;150m",
        "\x1b[48;5;151m",
        "\x1b[48;5;152m",
        "\x1b[48;5;153m",
        "\x1b[48;5;154m",
        "\x1b[48;5;155m",
        "\x1b[48;5;156m",
        "\x1b[48;5;157m",
        "\x1b[48;5;158m",
        "\x1b[48;5;159m",
        "\x1b[48;5;160m",
        "\x1b[48;5;161m",
        "\x1b[48;5;162m",
        "\x1b[48;5;163m",
        "\x1b[48;5;164m",
        "\x1b[48;5;165m",
        "\x1b[48;5;166m",
        "\x1b[48;5;167m",
        "\x1b[48;5;168m",
        "\x1b[48;5;169m",
        "\x1b[48;5;170m",
        "\x1b[48;5;171m",
        "\x1b[48;5;172m",
        "\x1b[48;5;173m",
        "\x1b[48;5;174m",
        "\x1b[48;5;175m",
        "\x1b[48;5;176m",
        "\x1b[48;5;177m",
        "\x1b[48;5;178m",
        "\x1b[48;5;179m",
        "\x1b[48;5;180m",
        "\x1b[48;5;181m",
        "\x1b[48;5;182m",
        "\x1b[48;5;183m",
        "\x1b[48;5;184m",
        "\x1b[48;5;185m",
        "\x1b[48;5;186m",
        "\x1b[48;5;187m",
        "\x1b[48;5;188m",
        "\x1b[48;5;189m",
        "\x1b[48;5;190m",
        "\x1b[48;5;191m",
        "\x1b[48;5;192m",
        "\x1b[48;5;193m",
        "\x1b[48;5;194m",
        "\x1b[48;5;195m",
        "\x1b[48;5;196m",
        "\x1b[48;5;197m",
        "\x1b[48;5;198m",
        "\x1b[48;5;199m",
        "\x1b[48;5;200m",
        "\x1b[48;5;201m",
        "\x1b[48;5;202m",
        "\x1b[48;5;203m",
        "\x1b[48;5;204m",
        "\x1b[48;5;205m",
        "\x1b[48;5;206m",
        "\x1b[48;5;207m",
        "\x1b[48;5;208m",
        "\x1b[48;5;209m",
        "\x1b[48;5;210m",
        "\x1b[48;5;211m",
        "\x1b[48;5;212m",
        "\x1b[48;5;213m",
        "\x1b[48;5;214m",
        "\x1b[48;5;215m",
        "\x1b[48;5;216m",
        "\x1b[48;5;217m",
        "\x1b[48;5;218m",
        "\x1b[48;5;219m",
        "\x1b[48;5;220m",
        "\x1b[48;5;221m",
        "\x1b[48;5;222m",
        "\x1b[48;5;223m",
        "\x1b[48;5;224m",
        "\x1b[48;5;225m",
        "\x1b[48;5;226m",
        "\x1b[48;5;227m",
        "\x1b[48;5;228m",
        "\x1b[48;5;229m",
        "\x1b[48;5;230m",
        "\x1b[48;5;231m",

        // greyscale colors (gradient)
        "\x1b[48;5;232m", // black
        "\x1b[48;5;233m",
        "\x1b[48;5;234m",
        "\x1b[48;5;235m",
        "\x1b[48;5;236m",
        "\x1b[48;5;237m",
        "\x1b[48;5;238m",
        "\x1b[48;5;239m",
        "\x1b[48;5;240m",
        "\x1b[48;5;241m",
        "\x1b[48;5;242m",
        "\x1b[48;5;243m",
        "\x1b[48;5;244m",
        "\x1b[48;5;245m",
        "\x1b[48;5;246m",
        "\x1b[48;5;247m",
        "\x1b[48;5;248m",
        "\x1b[48;5;249m",
        "\x1b[48;5;250m",
        "\x1b[48;5;251m",
        "\x1b[48;5;252m",
        "\x1b[48;5;253m",
        "\x1b[48;5;254m",
        "\x1b[48;5;255m"  // white
    ];

    // TODO: generate these instead of hardcode, when rust macros git gud
    /// underline [web-safe colors](https://en.wikipedia.org/wiki/Web_colors#Web-safe_colors) array
    pub static UNDERLINE_WEB_SAFE_COLORS: [&str; WEB_SAFE_COLORS_LEN] = [
        // standard colors
        "\x1b[58;5;0m",   // black
        "\x1b[58;5;1m",   // red
        "\x1b[58;5;2m",   // green
        "\x1b[58;5;3m",   // yellow
        "\x1b[58;5;4m",   // blue
        "\x1b[58;5;5m",   // magenta
        "\x1b[58;5;6m",   // cyan
        "\x1b[58;5;7m",   // white

        // high-intensity colors
        "\x1b[58;5;8m",   // bright black
        "\x1b[58;5;9m",   // bright red
        "\x1b[58;5;10m",  // bright green
        "\x1b[58;5;11m",  // bright yellow
        "\x1b[58;5;12m",  // bright blue
        "\x1b[58;5;13m",  // bright magenta
        "\x1b[58;5;14m",  // bright cyan
        "\x1b[58;5;15m",  // bright white

        // 216 colors
        "\x1b[58;5;16m",
        "\x1b[58;5;17m",
        "\x1b[58;5;18m",
        "\x1b[58;5;19m",
        "\x1b[58;5;20m",
        "\x1b[58;5;21m",
        "\x1b[58;5;22m",
        "\x1b[58;5;23m",
        "\x1b[58;5;24m",
        "\x1b[58;5;25m",
        "\x1b[58;5;26m",
        "\x1b[58;5;27m",
        "\x1b[58;5;28m",
        "\x1b[58;5;29m",
        "\x1b[58;5;30m",
        "\x1b[58;5;31m",
        "\x1b[58;5;32m",
        "\x1b[58;5;33m",
        "\x1b[58;5;34m",
        "\x1b[58;5;35m",
        "\x1b[58;5;36m",
        "\x1b[58;5;37m",
        "\x1b[58;5;38m",
        "\x1b[58;5;39m",
        "\x1b[58;5;40m",
        "\x1b[58;5;41m",
        "\x1b[58;5;42m",
        "\x1b[58;5;43m",
        "\x1b[58;5;44m",
        "\x1b[58;5;45m",
        "\x1b[58;5;46m",
        "\x1b[58;5;47m",
        "\x1b[58;5;48m",
        "\x1b[58;5;49m",
        "\x1b[58;5;50m",
        "\x1b[58;5;51m",
        "\x1b[58;5;52m",
        "\x1b[58;5;53m",
        "\x1b[58;5;54m",
        "\x1b[58;5;55m",
        "\x1b[58;5;56m",
        "\x1b[58;5;57m",
        "\x1b[58;5;58m",
        "\x1b[58;5;59m",
        "\x1b[58;5;60m",
        "\x1b[58;5;61m",
        "\x1b[58;5;62m",
        "\x1b[58;5;63m",
        "\x1b[58;5;64m",
        "\x1b[58;5;65m",
        "\x1b[58;5;66m",
        "\x1b[58;5;67m",
        "\x1b[58;5;68m",
        "\x1b[58;5;69m",
        "\x1b[58;5;70m",
        "\x1b[58;5;71m",
        "\x1b[58;5;72m",
        "\x1b[58;5;73m",
        "\x1b[58;5;74m",
        "\x1b[58;5;75m",
        "\x1b[58;5;76m",
        "\x1b[58;5;77m",
        "\x1b[58;5;78m",
        "\x1b[58;5;79m",
        "\x1b[58;5;80m",
        "\x1b[58;5;81m",
        "\x1b[58;5;82m",
        "\x1b[58;5;83m",
        "\x1b[58;5;84m",
        "\x1b[58;5;85m",
        "\x1b[58;5;86m",
        "\x1b[58;5;87m",
        "\x1b[58;5;88m",
        "\x1b[58;5;89m",
        "\x1b[58;5;90m",
        "\x1b[58;5;91m",
        "\x1b[58;5;92m",
        "\x1b[58;5;93m",
        "\x1b[58;5;94m",
        "\x1b[58;5;95m",
        "\x1b[58;5;96m",
        "\x1b[58;5;97m",
        "\x1b[58;5;98m",
        "\x1b[58;5;99m",
        "\x1b[58;5;100m",
        "\x1b[58;5;101m",
        "\x1b[58;5;102m",
        "\x1b[58;5;103m",
        "\x1b[58;5;104m",
        "\x1b[58;5;105m",
        "\x1b[58;5;106m",
        "\x1b[58;5;107m",
        "\x1b[58;5;108m",
        "\x1b[58;5;109m",
        "\x1b[58;5;110m",
        "\x1b[58;5;111m",
        "\x1b[58;5;112m",
        "\x1b[58;5;113m",
        "\x1b[58;5;114m",
        "\x1b[58;5;115m",
        "\x1b[58;5;116m",
        "\x1b[58;5;117m",
        "\x1b[58;5;118m",
        "\x1b[58;5;119m",
        "\x1b[58;5;120m",
        "\x1b[58;5;121m",
        "\x1b[58;5;122m",
        "\x1b[58;5;123m",
        "\x1b[58;5;124m",
        "\x1b[58;5;125m",
        "\x1b[58;5;126m",
        "\x1b[58;5;127m",
        "\x1b[58;5;128m",
        "\x1b[58;5;129m",
        "\x1b[58;5;130m",
        "\x1b[58;5;131m",
        "\x1b[58;5;132m",
        "\x1b[58;5;133m",
        "\x1b[58;5;134m",
        "\x1b[58;5;135m",
        "\x1b[58;5;136m",
        "\x1b[58;5;137m",
        "\x1b[58;5;138m",
        "\x1b[58;5;139m",
        "\x1b[58;5;140m",
        "\x1b[58;5;141m",
        "\x1b[58;5;142m",
        "\x1b[58;5;143m",
        "\x1b[58;5;144m",
        "\x1b[58;5;145m",
        "\x1b[58;5;146m",
        "\x1b[58;5;147m",
        "\x1b[58;5;148m",
        "\x1b[58;5;149m",
        "\x1b[58;5;150m",
        "\x1b[58;5;151m",
        "\x1b[58;5;152m",
        "\x1b[58;5;153m",
        "\x1b[58;5;154m",
        "\x1b[58;5;155m",
        "\x1b[58;5;156m",
        "\x1b[58;5;157m",
        "\x1b[58;5;158m",
        "\x1b[58;5;159m",
        "\x1b[58;5;160m",
        "\x1b[58;5;161m",
        "\x1b[58;5;162m",
        "\x1b[58;5;163m",
        "\x1b[58;5;164m",
        "\x1b[58;5;165m",
        "\x1b[58;5;166m",
        "\x1b[58;5;167m",
        "\x1b[58;5;168m",
        "\x1b[58;5;169m",
        "\x1b[58;5;170m",
        "\x1b[58;5;171m",
        "\x1b[58;5;172m",
        "\x1b[58;5;173m",
        "\x1b[58;5;174m",
        "\x1b[58;5;175m",
        "\x1b[58;5;176m",
        "\x1b[58;5;177m",
        "\x1b[58;5;178m",
        "\x1b[58;5;179m",
        "\x1b[58;5;180m",
        "\x1b[58;5;181m",
        "\x1b[58;5;182m",
        "\x1b[58;5;183m",
        "\x1b[58;5;184m",
        "\x1b[58;5;185m",
        "\x1b[58;5;186m",
        "\x1b[58;5;187m",
        "\x1b[58;5;188m",
        "\x1b[58;5;189m",
        "\x1b[58;5;190m",
        "\x1b[58;5;191m",
        "\x1b[58;5;192m",
        "\x1b[58;5;193m",
        "\x1b[58;5;194m",
        "\x1b[58;5;195m",
        "\x1b[58;5;196m",
        "\x1b[58;5;197m",
        "\x1b[58;5;198m",
        "\x1b[58;5;199m",
        "\x1b[58;5;200m",
        "\x1b[58;5;201m",
        "\x1b[58;5;202m",
        "\x1b[58;5;203m",
        "\x1b[58;5;204m",
        "\x1b[58;5;205m",
        "\x1b[58;5;206m",
        "\x1b[58;5;207m",
        "\x1b[58;5;208m",
        "\x1b[58;5;209m",
        "\x1b[58;5;210m",
        "\x1b[58;5;211m",
        "\x1b[58;5;212m",
        "\x1b[58;5;213m",
        "\x1b[58;5;214m",
        "\x1b[58;5;215m",
        "\x1b[58;5;216m",
        "\x1b[58;5;217m",
        "\x1b[58;5;218m",
        "\x1b[58;5;219m",
        "\x1b[58;5;220m",
        "\x1b[58;5;221m",
        "\x1b[58;5;222m",
        "\x1b[58;5;223m",
        "\x1b[58;5;224m",
        "\x1b[58;5;225m",
        "\x1b[58;5;226m",
        "\x1b[58;5;227m",
        "\x1b[58;5;228m",
        "\x1b[58;5;229m",
        "\x1b[58;5;230m",
        "\x1b[58;5;231m",

        // greyscale colors (gradient)
        "\x1b[58;5;232m", // black
        "\x1b[58;5;233m",
        "\x1b[58;5;234m",
        "\x1b[58;5;235m",
        "\x1b[58;5;236m",
        "\x1b[58;5;237m",
        "\x1b[58;5;238m",
        "\x1b[58;5;239m",
        "\x1b[58;5;240m",
        "\x1b[58;5;241m",
        "\x1b[58;5;242m",
        "\x1b[58;5;243m",
        "\x1b[58;5;244m",
        "\x1b[58;5;245m",
        "\x1b[58;5;246m",
        "\x1b[58;5;247m",
        "\x1b[58;5;248m",
        "\x1b[58;5;249m",
        "\x1b[58;5;250m",
        "\x1b[58;5;251m",
        "\x1b[58;5;252m",
        "\x1b[58;5;253m",
        "\x1b[58;5;254m",
        "\x1b[58;5;255m"  // white
    ];

    /// foreground [RGB colors](https://en.wikipedia.org/wiki/RGB_color_model) (basically all the colors)
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    ///
    /// (red, green, blue)
    #[derive(Clone, Copy, Debug)] pub struct        FgRgb(pub u8, pub u8, pub u8);

    /// background [RGB colors](https://en.wikipedia.org/wiki/RGB_color_model) (basically all the colors)
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    ///
    /// (red, green, blue)
    #[derive(Clone, Copy, Debug)] pub struct        BgRgb(pub u8, pub u8, pub u8);

    /// underline [RGB colors](https://en.wikipedia.org/wiki/RGB_color_model) (basically all the colors)
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    ///
    /// (red, green, blue)
    #[derive(Clone, Copy, Debug)] pub struct UnderlineRgb(pub u8, pub u8, pub u8);

    impl fmt::Display for FgRgb {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[38;2;{};{};{}m", self.0, self.1, self.2)
        }
    }

    impl fmt::Display for BgRgb {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[48;2;{};{};{}m", self.0, self.1, self.2)
        }
    }

    impl fmt::Display for UnderlineRgb {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[58;2;{};{};{}m", self.0, self.1, self.2)
        }
    }
}

/// hide/show, movement, save/restore position
///
/// the coordinates are 1-based. that means the top left corner of the terminal is (1, 1)
pub mod cursor {
    use std::fmt;

    /// save the cursor position to restore it later
    pub const    SAVE_POSITION:      &str = "\x1b[s";

    /// restore the saved cursor position
    pub const RESTORE_POSITION:      &str = "\x1b[u";

    /// hide the cursor (it still exists and movement/typing works)
    pub const HIDE:                  &str = "\x1b[?25l";

    /// show the cursor
    pub const SHOW:                  &str = "\x1b[?25h";

    pub const MOVE_UP_BY_ONE:        &str = "\x1b[1A";
    pub const MOVE_DOWN_BY_ONE:      &str = "\x1b[1B";
    pub const MOVE_RIGHT_BY_ONE:     &str = "\x1b[1C";
    pub const MOVE_LEFT_BY_ONE:      &str = "\x1b[1D";

    pub const MOVE_TO_TOP_LEFT:      &str = "\x1b[1;1H";

    pub const MOVE_TO_START_OF_LINE: &str = "\x1b[1G";

    /// move cursor up by the given number, it can scroll the terminal
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct MoveUpBy                   (pub u16);

    /// move cursor down by the given number, it can scroll the terminal
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct MoveDownBy                 (pub u16);

    /// move cursor right by the given number, it stops at the end of line
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct MoveRightBy                (pub u16);

    /// move cursor right by the given number, it stops at the start of line
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct MoveLeftBy                 (pub u16);

    /// move cursor down by one, and to the start of the line
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct MoveToStartOfNextLine      (pub u16);

    /// move cursor up by one, and to the start of the line
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct MoveToStartOfPreviousLine  (pub u16);

    /// move cursor to a column of the given number
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct MoveToColumn               (pub u16);

    /// move cursor to a row of the given number
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct MoveToRow                  (pub u16);

    /// move cursor to the specified cell (column first, row second)
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct MoveToColumnAndRow         (pub u16, pub u16);

    /// like [MoveToColumnAndRow], but behaves differently in certain terminal modes
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct FormattedMoveToColumnAndRow(pub u16, pub u16);

    /// request the current cursor position
    #[must_use] pub fn get_position() -> (u16, u16) { todo!(); }

    impl fmt::Display for MoveUpBy {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[{}A", self.0)
        }
    }

    impl fmt::Display for MoveDownBy {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[{}B", self.0)
        }
    }

    impl fmt::Display for MoveRightBy {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[{}C", self.0)
        }
    }

    impl fmt::Display for MoveLeftBy {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[{}D", self.0)
        }
    }

    impl fmt::Display for MoveToStartOfNextLine {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[{}E", self.0)
        }
    }

    impl fmt::Display for MoveToStartOfPreviousLine {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[{}F", self.0)
        }
    }

    impl fmt::Display for MoveToColumn {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[{}G", self.0)
        }
    }

    impl fmt::Display for MoveToRow {
        #[inline]
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            todo!();
        }
    }

    impl fmt::Display for MoveToColumnAndRow {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[{};{}H", self.1, self.0)
        }
    }

    impl fmt::Display for FormattedMoveToColumnAndRow {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[{};{}f", self.1, self.0)
        }
    }
}

/// raw mode, mouse input, focus reporting, bracketed paste
pub mod mode {
    pub const TURN_FOCUS_REPORTS_ON:    &str = "\x1b[?1004h";

    pub const TURN_FOCUS_REPORTS_OFF:   &str = "\x1b[?1004l";

    pub const TURN_BRACKETED_PASTE_ON:  &str = "\x1b[?2004h";

    pub const TURN_BRACKETED_PASTE_OFF: &str = "\x1b[?2004l";

    // raw mode, mouse input, ...

    /// check if raw mode is currently enabled
    #[must_use] pub fn is_raw() -> bool { todo!(); }
}

/// page break, AUX toggle
pub mod printer {
    // pub const PAGE_BREAK: char = '\f';

    pub const TURN_AUX_ON:     &str = "\x1b[5i";

    pub const TURN_AUX_OFF:    &str = "\x1b[4i";
}

/// alternate screen, terminal size in cells or pixels
pub mod screen {
    /// enter a temporary screen where stuff does not affect the real one
    pub const ENTER_ALTERNATE: &str = "\x1b[?1049h";

    /// get back to the real original terminal state and text
    pub const LEAVE_ALTERNATE: &str = "\x1b[?1049l";

    /// check if the current screen is alternative
    #[must_use] pub fn     is_alternate () -> bool       { todo!(); }

    /// get the terminal size in columns and rows
    #[must_use] pub fn   size_in_cells  () -> (u16, u16) { todo!(); }

    /// get the terminal size in pixels (width first, height second)
    #[must_use] pub fn   size_in_pixels () -> (u16, u16) { todo!(); }

    /// get the terminal width in columns
    #[must_use] pub fn  width_in_columns() -> u16        { todo!(); }

    /// get the terminal width in pixels
    #[must_use] pub fn  width_in_pixels () -> u16        { todo!(); }

    /// get the terminal height in rows
    #[must_use] pub fn height_in_rows   () -> u16        { todo!(); }

    /// get the terminal height in pixels
    #[must_use] pub fn height_in_pixels () -> u16        { todo!(); }
}

/// up, down, set region
pub mod scroll {
    use std::fmt;

    pub const   UP_BY_ONE:  &str = "\x1b[1S";

    pub const DOWN_BY_ONE:  &str = "\x1b[1T";

    /// reset the effects of [SetRegionStartAndEndRow]
    pub const UNSET_REGION: &str = "\x1b[r";

    /// scroll the terminal up by the given number (region can be customised with [SetRegionStartAndEndRow])
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct UpBy                   (pub u16);

    /// scroll the terminal down by the given number (region can be customised with [SetRegionStartAndEndRow])
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct DownBy                 (pub u16);

    /// set the scrolling region to the given range, lines outside are not affected
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct SetRegionStartAndEndRow(pub u16, pub u16);

    impl fmt::Display for UpBy {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[{}S", self.0)
        }
    }

    impl fmt::Display for DownBy {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[{}T", self.0)
        }
    }

    impl fmt::Display for SetRegionStartAndEndRow {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[{};{}r", self.0, self.1)
        }
    }
}

/// bold, italic, underline, strikethrough, ...
pub mod style {
    use std::fmt;

    /// higher intensity of color, or thicker font weight
    pub const        INTENSITY_BOLD:            &str = "\x1b[1m";

    /// lower intensity of color, or thinner font weight
    pub const        INTENSITY_FAINT:           &str = "\x1b[2m";

    /// reset the effects of INTENSITY_*
    pub const  UNSET_INTENSITY:                 &str = "\x1b[22m";

    /// slanted text (e.g. to indicate a quote, or emphasis)
    pub const        ITALIC:                    &str = "\x1b[3m";

    /// reset the effects of [ITALIC]
    pub const  UNSET_ITALIC:                    &str = "\x1b[23m";

    /// a line under text (e.g. to indicate something important)
    pub const        UNDERLINE:                 &str = "\x1b[4m";

    /// a double line under text (e.g. to indicate something really important)
    pub const        UNDERLINE_DOUBLE:          &str = "\x1b[21m";

    /// reset the effects of UNDERLINE_*
    pub const  UNSET_UNDERLINE:                 &str = "\x1b[24m";

    /// blink the text slowly (either with intensity or completely hide/show)
    pub const   BLINK_SLOW:                     &str = "\x1b[5m";

    /// blink the text quickly (either with intensity or completely hide/show)
    pub const   BLINK_FAST:                     &str = "\x1b[6m";

    /// reset the effects of BLINK_*
    pub const  UNSET_BLINK:                     &str = "\x1b[25m";

    /// swap the background and foreground colors
    pub const        INVERT_COLORS:             &str = "\x1b[7m";

    /// reset the effects of [INVERT_COLORS]
    pub const  UNSET_INVERT:                    &str = "\x1b[27m";

    /// hide the text (it still exists and can be e.g. copied)
    pub const        CONCEAL:                   &str = "\x1b[8m";

    /// reset the effects of [CONCEAL]
    pub const  UNSET_CONCEAL:                   &str = "\x1b[28m";

    /// a line going through the text to indicate redacted text
    pub const        STRIKETHROUGH:             &str = "\x1b[9m";

    /// reset the effects of [STRIKETHROUGH]
    pub const  UNSET_STRIKETHROUGH:             &str = "\x1b[29m";

    pub const        FONT_ALTERNATIVE_1:        &str = "\x1b[11m";

    pub const        FONT_ALTERNATIVE_2:        &str = "\x1b[12m";

    pub const        FONT_ALTERNATIVE_3:        &str = "\x1b[13m";

    pub const        FONT_ALTERNATIVE_4:        &str = "\x1b[14m";

    pub const        FONT_ALTERNATIVE_5:        &str = "\x1b[15m";

    pub const        FONT_ALTERNATIVE_6:        &str = "\x1b[16m";

    pub const        FONT_ALTERNATIVE_7:        &str = "\x1b[17m";

    pub const        FONT_ALTERNATIVE_8:        &str = "\x1b[18m";

    pub const        FONT_ALTERNATIVE_9:        &str = "\x1b[19m";

    pub const        FONT_GOTHIC:               &str = "\x1b[20m";

    /// reset the effects of FONT_* (set the font to the primary one)
    pub const  UNSET_FONT:                      &str = "\x1b[10m";

    pub const        SPACING_PROPORTIONAL:      &str = "\x1b[26m";

    /// reset the effects of [SPACING_PROPORTIONAL]
    pub const  UNSET_SPACING:                   &str = "\x1b[50m";

    /// emoji [variation selector](https://en.wikipedia.org/wiki/Variation_Selectors_(Unicode_block))
    pub const        EMOJI_FRAMED:              &str = "\x1b[51m";

    /// emoji [variation selector](https://en.wikipedia.org/wiki/Variation_Selectors_(Unicode_block))
    pub const        EMOJI_ENCIRCLED:           &str = "\x1b[52m";

    /// reset the effects of EMOJI_*
    pub const  UNSET_EMOJI:                     &str = "\x1b[54m";

    /// a line on top of text (e.g. to indicate repeating decimals)
    pub const        OVERLINE:                  &str = "\x1b[53m";

    /// reset the effects of [OVERLINE]
    pub const  UNSET_OVERLINE:                  &str = "\x1b[55m";

    /// sometimes a line on the right side
    pub const        IDEOGRAM_UNDERLINE:        &str = "\x1b[60m";

    /// sometimes a double line on the right side
    pub const        IDEOGRAM_DOUBLE_UNDERLINE: &str = "\x1b[61m";

    /// sometimes a line on the left side
    pub const        IDEOGRAM_OVERLINE:         &str = "\x1b[62m";

    /// sometimes a double line on the left side
    pub const        IDEOGRAM_DOUBLE_OVERLINE:  &str = "\x1b[63m";

    pub const        IDEOGRAM_STRESS_MARKING:   &str = "\x1b[64m";

    /// reset the effects of IDEOGRAM_*
    pub const  UNSET_IDEOGRAM:                  &str = "\x1b[65m";

    /// [superscript](https://en.wikipedia.org/wiki/Subscript_and_superscript), small text above the normal line
    pub const        SCRIPT_SUPER:              &str = "\x1b[73m";

    /// [subscript](https://en.wikipedia.org/wiki/Subscript_and_superscript), small text under the normal line
    pub const        SCRIPT_SUB:                &str = "\x1b[74m";

    /// reset the effects of SCRIPT_*
    pub const  UNSET_SCRIPT:                    &str = "\x1b[75m";

    /// the same as FONT_ALTERNATIVE_* (panics when input is outside 1..=9)
    ///
    /// it implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), so
    /// you can just print it, or convert it into a [String](https://doc.rust-lang.org/std/string/struct.String.html)
    /// with [.to_string()](https://doc.rust-lang.org/std/string/trait.ToString.html)
    #[derive(Clone, Copy, Debug)] pub struct FontAlternative(pub u8);

    impl fmt::Display for FontAlternative {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let n = self.0 + 10;
            assert!(n < 20 && n > 10, "alternative font is NOT in the range of 1..=9");

            write!(f, "\x1b[{n}m")
        }
    }
}

