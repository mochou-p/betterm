// mochou-p/betterm/src/terminal.rs

use std::io::{self, Write, Read};
use std::os::fd::AsRawFd;


/// get the size of output in cells (not in pixels)
pub fn size(output_fd: std::os::fd::RawFd) -> (u16, u16) {
    let mut winsize: libc::winsize = unsafe { std::mem::zeroed() };
    assert_eq!(unsafe { libc::ioctl(output_fd, libc::TIOCGWINSZ, &mut winsize) }, 0);

    (winsize.ws_col, winsize.ws_row)
}

/// makes all input state raw through libc::cfmakeraw,  
/// makes input block waiting for atleast 1 byte when read,  
/// enters alternate screen,  
/// clears the whole screen,  
/// enables all mouse events,  
/// enables focus reporting,  
/// enables bracketed paste mode,  
/// moves the cursor to the top left corner  
/// (these will be customizable later)
pub struct RawTerminal {
        old_termios: libc::termios,
    /// a stream that produces bytes (for example: events)
    pub input:       Box<dyn Input>,
    /// a stream that consumes bytes (for example: printing)
    pub output:      Box<dyn Output>
}

/// combines all required traits of input into one trait
pub trait Input: Read + AsRawFd {}
impl<T: Read + AsRawFd> Input for T {}

/// combines all required traits of output into one trait
pub trait Output: Write + AsRawFd {}
impl<T: Write + AsRawFd> Output for T {}

impl Write for RawTerminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

impl Read for RawTerminal {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.input.read(buf)
    }
}

impl RawTerminal {
    /// turns input and output into a raw state
    pub fn new(input: impl Input + 'static, mut output: impl Output + 'static) -> Self {
        let mut maybe_termios = std::mem::MaybeUninit::uninit();
        assert_eq!(unsafe { libc::tcgetattr(input.as_raw_fd(), maybe_termios.as_mut_ptr()) }, 0);

        let mut termios = unsafe { maybe_termios.assume_init() };
        let old_termios = termios;

        unsafe { libc::cfmakeraw(&mut termios); }
        termios.c_cc[libc::VMIN ] = 1;
        termios.c_cc[libc::VTIME] = 0;

        assert_eq!(unsafe { libc::tcsetattr(input.as_raw_fd(), libc::TCSAFLUSH, &termios) }, 0);

        output.write_all(
            format!(
                "{}{}{}{}{}{}{}{}",
                "\x1b[?25l",   // hide cursor
                "\x1b[?1049h", // enter alternate screen
                "\x1b[3J",     // clear whole screen
                "\x1b[?1003h", // enable mouse events
                "\x1b[?1006h", // extend mouse support
                "\x1b[?1004h", // enable focus reporting
                "\x1b[?2004h", // enable bracketed paste
                "\x1b[1;1H"    // move cursor to 1,1
            ).as_bytes()
        ).unwrap();

        output.flush().unwrap();

        Self { old_termios, input: Box::new(input), output: Box::new(output) }
    }

    fn read_event_to_string(&mut self) -> String {
        let mut buffer = [0; 64];
        let     count  = self.read(&mut buffer).unwrap();
        let     bytes  = &buffer[..count];

        String::from_utf8(bytes.to_vec()).unwrap()
    }

    fn keyboard_event(&mut self, string: &str) -> Option<KeyboardEvent> {
        match string {
            "\u{1b}[200~" => {
                let mut accumulator = String::new();

                loop {
                    let chunk = self.read_event_to_string();

                    if chunk == "\u{1b}[201~" {
                        break;
                    } else {
                        accumulator.push_str(&chunk);
                    }
                }

                Some(KeyboardEvent::Pasted(accumulator))
            },

            "\u{7f}"          => Some(KeyboardEvent::         Backspace),
            "\u{8}"           => Some(KeyboardEvent::     CtrlBackspace),
            "\u{1b}\u{7f}"    => Some(KeyboardEvent::      AltBackspace),
            "\u{1b}\u{8}"     => Some(KeyboardEvent::  CtrlAltBackspace),

            "\u{1b}[2~"       => Some(KeyboardEvent::            Insert),
            "\u{1b}[2;5~"     => Some(KeyboardEvent::        CtrlInsert),
            "\u{1b}[2;3~"     => Some(KeyboardEvent::         AltInsert),
            "\u{1b}[2;7~"     => Some(KeyboardEvent::     CtrlAltInsert),
            "\u{1b}[2;6~"     => Some(KeyboardEvent::   CtrlShiftInsert),
            "\u{1b}[2;4~"     => Some(KeyboardEvent::    AltShiftInsert),
            "\u{1b}[2;8~"     => Some(KeyboardEvent::CtrlAltShiftInsert),

            "\u{1}"           => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::A)),
            "\u{2}"           => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::B)),
            "\u{3}"           => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::C)),
            "\u{4}"           => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::D)),
            "\u{5}"           => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::E)),
            "\u{6}"           => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::F)),
            "\u{7}"           => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::G)),
            "\u{b}"           => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::K)),
            "\u{c}"           => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::L)),
            "\u{e}"           => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::N)),
            "\u{f}"           => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::O)),
            "\u{10}"          => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::P)),
            "\u{11}"          => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::Q)),
            "\u{12}"          => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::R)),
            "\u{13}"          => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::S)),
            "\u{14}"          => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::T)),
            "\u{15}"          => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::U)),
            "\u{16}"          => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::V)),
            "\u{17}"          => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::W)),
            "\u{18}"          => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::X)),
            "\u{19}"          => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::Y)),
            "\u{1a}"          => Some(KeyboardEvent::   CtrlChar (   CtrlableChar::Z)),

            "\u{1b}\u{1}"     => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::A)),
            "\u{1b}\u{2}"     => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::B)),
            "\u{1b}\u{3}"     => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::C)),
            "\u{1b}\u{4}"     => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::D)),
            "\u{1b}\u{5}"     => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::E)),
            "\u{1b}\u{6}"     => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::F)),
            "\u{1b}\u{7}"     => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::G)),
            "\u{1b}\n"        => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::J)), // TODO: could be something else
            "\u{1b}\u{b}"     => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::K)),
            "\u{1b}\u{c}"     => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::L)),
            "\u{1b}\u{e}"     => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::N)),
            "\u{1b}\u{f}"     => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::O)),
            "\u{1b}\u{10}"    => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::P)),
            "\u{1b}\u{11}"    => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::Q)),
            "\u{1b}\u{12}"    => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::R)),
            "\u{1b}\u{13}"    => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::S)),
            "\u{1b}\u{14}"    => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::T)),
            "\u{1b}\u{15}"    => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::U)),
            "\u{1b}\u{16}"    => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::V)),
            "\u{1b}\u{17}"    => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::W)),
            "\u{1b}\u{18}"    => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::X)),
            "\u{1b}\u{19}"    => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::Y)),
            "\u{1b}\u{1a}"    => Some(KeyboardEvent::CtrlAltChar (CtrlAltableChar::Z)),

            "\r" | "\n"       => Some(KeyboardEvent::NoModifiers (Key::Enter        )),
            "\u{1b}"          => Some(KeyboardEvent::NoModifiers (Key::Escape       )),
            "\u{1b}OP"        => Some(KeyboardEvent::NoModifiers (Key::F1           )),
            "\u{1b}OQ"        => Some(KeyboardEvent::NoModifiers (Key::F2           )),
            "\u{1b}OR"        => Some(KeyboardEvent::NoModifiers (Key::F3           )),
            "\u{1b}OS"        => Some(KeyboardEvent::NoModifiers (Key::F4           )),
            "\u{1b}[15~"      => Some(KeyboardEvent::NoModifiers (Key::F5           )),
            "\u{1b}[17~"      => Some(KeyboardEvent::NoModifiers (Key::F6           )),
            "\u{1b}[18~"      => Some(KeyboardEvent::NoModifiers (Key::F7           )),
            "\u{1b}[19~"      => Some(KeyboardEvent::NoModifiers (Key::F8           )),
            "\u{1b}[20~"      => Some(KeyboardEvent::NoModifiers (Key::F9           )),
            "\u{1b}[21~"      => Some(KeyboardEvent::NoModifiers (Key::F10          )),
            "\u{1b}[23~"      => Some(KeyboardEvent::NoModifiers (Key::F11          )),
            "\u{1b}[24~"      => Some(KeyboardEvent::NoModifiers (Key::F12          )),
            "\t"              => Some(KeyboardEvent::NoModifiers (Key::Tab          )),
            "\u{1b}[3~"       => Some(KeyboardEvent::NoModifiers (Key::Delete       )),
            "\u{1b}[H"        => Some(KeyboardEvent::NoModifiers (Key::Home         )),
            "\u{1b}[F"        => Some(KeyboardEvent::NoModifiers (Key::End          )),
            "\u{1b}[5~"       => Some(KeyboardEvent::NoModifiers (Key::PageUp       )),
            "\u{1b}[6~"       => Some(KeyboardEvent::NoModifiers (Key::PageDown     )),
            "\u{1b}[A"        => Some(KeyboardEvent::NoModifiers (Key::ArrowUp      )),
            "\u{1b}[D"        => Some(KeyboardEvent::NoModifiers (Key::ArrowLeft    )),
            "\u{1b}[B"        => Some(KeyboardEvent::NoModifiers (Key::ArrowDown    )),
            "\u{1b}[C"        => Some(KeyboardEvent::NoModifiers (Key::ArrowRight   )),

            "\u{1b}[27;5;13~" => Some(KeyboardEvent::Ctrl        (Key::Enter        )),
            "\u{1b}[27;5;27~" => Some(KeyboardEvent::Ctrl        (Key::Escape       )),
            "\u{1b}[1;5P"     => Some(KeyboardEvent::Ctrl        (Key::F1           )),
            "\u{1b}[1;5Q"     => Some(KeyboardEvent::Ctrl        (Key::F2           )),
            "\u{1b}[1;5R"     => Some(KeyboardEvent::Ctrl        (Key::F3           )),
            "\u{1b}[1;5S"     => Some(KeyboardEvent::Ctrl        (Key::F4           )),
            "\u{1b}[15;5~"    => Some(KeyboardEvent::Ctrl        (Key::F5           )),
            "\u{1b}[17;5~"    => Some(KeyboardEvent::Ctrl        (Key::F6           )),
            "\u{1b}[18;5~"    => Some(KeyboardEvent::Ctrl        (Key::F7           )),
            "\u{1b}[19;5~"    => Some(KeyboardEvent::Ctrl        (Key::F8           )),
            "\u{1b}[20;5~"    => Some(KeyboardEvent::Ctrl        (Key::F9           )),
            "\u{1b}[21;5~"    => Some(KeyboardEvent::Ctrl        (Key::F10          )),
            "\u{1b}[23;5~"    => Some(KeyboardEvent::Ctrl        (Key::F11          )),
            "\u{1b}[24;5~"    => Some(KeyboardEvent::Ctrl        (Key::F12          )),
            "\u{1b}[27;5;9~"  => Some(KeyboardEvent::Ctrl        (Key::Tab          )),
            "\u{1b}[3;5~"     => Some(KeyboardEvent::Ctrl        (Key::Delete       )),
            "\u{1b}[1;5H"     => Some(KeyboardEvent::Ctrl        (Key::Home         )),
            "\u{1b}[1;5F"     => Some(KeyboardEvent::Ctrl        (Key::End          )),
            "\u{1b}[5;5~"     => Some(KeyboardEvent::Ctrl        (Key::PageUp       )),
            "\u{1b}[6;5~"     => Some(KeyboardEvent::Ctrl        (Key::PageDown     )),
            "\u{1b}[1;5A"     => Some(KeyboardEvent::Ctrl        (Key::ArrowUp      )),
            "\u{1b}[1;5D"     => Some(KeyboardEvent::Ctrl        (Key::ArrowLeft    )),
            "\u{1b}[1;5B"     => Some(KeyboardEvent::Ctrl        (Key::ArrowDown    )),
            "\u{1b}[1;5C"     => Some(KeyboardEvent::Ctrl        (Key::ArrowRight   )),

            "\u{1b}\r"        => Some(KeyboardEvent::Alt         (Key::Enter        )),
            "\u{1b}\u{1b}"    => Some(KeyboardEvent::Alt         (Key::Escape       )),
            "\u{1b}[1;3P"     => Some(KeyboardEvent::Alt         (Key::F1           )),
            "\u{1b}[1;3Q"     => Some(KeyboardEvent::Alt         (Key::F2           )),
            "\u{1b}[1;3R"     => Some(KeyboardEvent::Alt         (Key::F3           )),
            "\u{1b}[1;3S"     => Some(KeyboardEvent::Alt         (Key::F4           )),
            "\u{1b}[15;3~"    => Some(KeyboardEvent::Alt         (Key::F5           )),
            "\u{1b}[17;3~"    => Some(KeyboardEvent::Alt         (Key::F6           )),
            "\u{1b}[18;3~"    => Some(KeyboardEvent::Alt         (Key::F7           )),
            "\u{1b}[19;3~"    => Some(KeyboardEvent::Alt         (Key::F8           )),
            "\u{1b}[20;3~"    => Some(KeyboardEvent::Alt         (Key::F9           )),
            "\u{1b}[21;3~"    => Some(KeyboardEvent::Alt         (Key::F10          )),
            "\u{1b}[23;3~"    => Some(KeyboardEvent::Alt         (Key::F11          )),
            "\u{1b}[24;3~"    => Some(KeyboardEvent::Alt         (Key::F12          )),
            "\u{1b}\t"        => Some(KeyboardEvent::Alt         (Key::Tab          )),
            "\u{1b}[3;3~"     => Some(KeyboardEvent::Alt         (Key::Delete       )),
            "\u{1b}[1;3H"     => Some(KeyboardEvent::Alt         (Key::Home         )),
            "\u{1b}[1;3F"     => Some(KeyboardEvent::Alt         (Key::End          )),
            "\u{1b}[5;3~"     => Some(KeyboardEvent::Alt         (Key::PageUp       )),
            "\u{1b}[6;3~"     => Some(KeyboardEvent::Alt         (Key::PageDown     )),
            "\u{1b}[1;3A"     => Some(KeyboardEvent::Alt         (Key::ArrowUp      )),
            "\u{1b}[1;3D"     => Some(KeyboardEvent::Alt         (Key::ArrowLeft    )),
            "\u{1b}[1;3B"     => Some(KeyboardEvent::Alt         (Key::ArrowDown    )),
            "\u{1b}[1;3C"     => Some(KeyboardEvent::Alt         (Key::ArrowRight   )),

            "\u{1b}[27;2;13~" => Some(KeyboardEvent::Shift       (Key::Enter        )),
            "\u{1b}[27;2;27~" => Some(KeyboardEvent::Shift       (Key::Escape       )),
            "\u{1b}[1;2P"     => Some(KeyboardEvent::Shift       (Key::F1           )),
            "\u{1b}[1;2Q"     => Some(KeyboardEvent::Shift       (Key::F2           )),
            "\u{1b}[1;2R"     => Some(KeyboardEvent::Shift       (Key::F3           )),
            "\u{1b}[1;2S"     => Some(KeyboardEvent::Shift       (Key::F4           )),
            "\u{1b}[15;2~"    => Some(KeyboardEvent::Shift       (Key::F5           )),
            "\u{1b}[17;2~"    => Some(KeyboardEvent::Shift       (Key::F6           )),
            "\u{1b}[18;2~"    => Some(KeyboardEvent::Shift       (Key::F7           )),
            "\u{1b}[19;2~"    => Some(KeyboardEvent::Shift       (Key::F8           )),
            "\u{1b}[20;2~"    => Some(KeyboardEvent::Shift       (Key::F9           )),
            "\u{1b}[21;2~"    => Some(KeyboardEvent::Shift       (Key::F10          )),
            "\u{1b}[23;2~"    => Some(KeyboardEvent::Shift       (Key::F11          )),
            "\u{1b}[24;2~"    => Some(KeyboardEvent::Shift       (Key::F12          )),
            "\u{1b}[Z"        => Some(KeyboardEvent::Shift       (Key::Tab          )),
            "\u{1b}[3;2~"     => Some(KeyboardEvent::Shift       (Key::Delete       )),
            "\u{1b}[1;2H"     => Some(KeyboardEvent::Shift       (Key::Home         )),
            "\u{1b}[1;2F"     => Some(KeyboardEvent::Shift       (Key::End          )),
            "\u{1b}[5;2~"     => Some(KeyboardEvent::Shift       (Key::PageUp       )),
            "\u{1b}[6;2~"     => Some(KeyboardEvent::Shift       (Key::PageDown     )),
            "\u{1b}[1;2A"     => Some(KeyboardEvent::Shift       (Key::ArrowUp      )),
            "\u{1b}[1;2D"     => Some(KeyboardEvent::Shift       (Key::ArrowLeft    )),
            "\u{1b}[1;2B"     => Some(KeyboardEvent::Shift       (Key::ArrowDown    )),
            "\u{1b}[1;2C"     => Some(KeyboardEvent::Shift       (Key::ArrowRight   )),

            "\u{1b}[27;7;13~" => Some(KeyboardEvent::CtrlAlt     (Key::Enter        )),
            "\u{1b}[27;7;27~" => Some(KeyboardEvent::CtrlAlt     (Key::Escape       )),
            "\u{1b}[1;7P"     => Some(KeyboardEvent::CtrlAlt     (Key::F1           )), // TTY1  lol
            "\u{1b}[1;7Q"     => Some(KeyboardEvent::CtrlAlt     (Key::F2           )), // TTY2  lol
            "\u{1b}[1;7R"     => Some(KeyboardEvent::CtrlAlt     (Key::F3           )), // TTY3  lol
            "\u{1b}[1;7S"     => Some(KeyboardEvent::CtrlAlt     (Key::F4           )), // TTY4  lol
            "\u{1b}[15;7~"    => Some(KeyboardEvent::CtrlAlt     (Key::F5           )), // TTY5  lol
            "\u{1b}[17;7~"    => Some(KeyboardEvent::CtrlAlt     (Key::F6           )), // TTY6  lol
            "\u{1b}[18;7~"    => Some(KeyboardEvent::CtrlAlt     (Key::F7           )), // TTY7  lol
            "\u{1b}[19;7~"    => Some(KeyboardEvent::CtrlAlt     (Key::F8           )), // TTY8  lol
            "\u{1b}[20;7~"    => Some(KeyboardEvent::CtrlAlt     (Key::F9           )), // TTY9  lol
            "\u{1b}[21;7~"    => Some(KeyboardEvent::CtrlAlt     (Key::F10          )), // TTY10 lol
            "\u{1b}[23;7~"    => Some(KeyboardEvent::CtrlAlt     (Key::F11          )), // TTY11 lol
            "\u{1b}[24;7~"    => Some(KeyboardEvent::CtrlAlt     (Key::F12          )), // TTY12 lol
            "\u{1b}[27;7;9~"  => Some(KeyboardEvent::CtrlAlt     (Key::Tab          )),
            "\u{1b}[3;7~"     => Some(KeyboardEvent::CtrlAlt     (Key::Delete       )),
            "\u{1b}[1;7H"     => Some(KeyboardEvent::CtrlAlt     (Key::Home         )),
            "\u{1b}[1;7F"     => Some(KeyboardEvent::CtrlAlt     (Key::End          )),
            "\u{1b}[5;7~"     => Some(KeyboardEvent::CtrlAlt     (Key::PageUp       )),
            "\u{1b}[6;7~"     => Some(KeyboardEvent::CtrlAlt     (Key::PageDown     )),
            "\u{1b}[1;7A"     => Some(KeyboardEvent::CtrlAlt     (Key::ArrowUp      )),
            "\u{1b}[1;7D"     => Some(KeyboardEvent::CtrlAlt     (Key::ArrowLeft    )),
            "\u{1b}[1;7B"     => Some(KeyboardEvent::CtrlAlt     (Key::ArrowDown    )),
            "\u{1b}[1;7C"     => Some(KeyboardEvent::CtrlAlt     (Key::ArrowRight   )),

            "\u{1b}[27;6;13~" => Some(KeyboardEvent::CtrlShift   (Key::Enter        )),
            "\u{1b}[27;6;27~" => Some(KeyboardEvent::CtrlShift   (Key::Escape       )),
            "\u{1b}[1;6P"     => Some(KeyboardEvent::CtrlShift   (Key::F1           )),
            "\u{1b}[1;6Q"     => Some(KeyboardEvent::CtrlShift   (Key::F2           )),
            "\u{1b}[1;6R"     => Some(KeyboardEvent::CtrlShift   (Key::F3           )),
            "\u{1b}[1;6S"     => Some(KeyboardEvent::CtrlShift   (Key::F4           )),
            "\u{1b}[15;6~"    => Some(KeyboardEvent::CtrlShift   (Key::F5           )),
            "\u{1b}[17;6~"    => Some(KeyboardEvent::CtrlShift   (Key::F6           )),
            "\u{1b}[18;6~"    => Some(KeyboardEvent::CtrlShift   (Key::F7           )),
            "\u{1b}[19;6~"    => Some(KeyboardEvent::CtrlShift   (Key::F8           )),
            "\u{1b}[20;6~"    => Some(KeyboardEvent::CtrlShift   (Key::F9           )),
            "\u{1b}[21;6~"    => Some(KeyboardEvent::CtrlShift   (Key::F10          )),
            "\u{1b}[23;6~"    => Some(KeyboardEvent::CtrlShift   (Key::F11          )),
            "\u{1b}[24;6~"    => Some(KeyboardEvent::CtrlShift   (Key::F12          )),
            "\u{1b}[27;6;9~"  => Some(KeyboardEvent::CtrlShift   (Key::Tab          )),
            "\u{1b}[3;6~"     => Some(KeyboardEvent::CtrlShift   (Key::Delete       )),
            "\u{1b}[1;6H"     => Some(KeyboardEvent::CtrlShift   (Key::Home         )),
            "\u{1b}[1;6F"     => Some(KeyboardEvent::CtrlShift   (Key::End          )),
            "\u{1b}[5;6~"     => Some(KeyboardEvent::CtrlShift   (Key::PageUp       )),
            "\u{1b}[6;6~"     => Some(KeyboardEvent::CtrlShift   (Key::PageDown     )),
            "\u{1b}[1;6A"     => Some(KeyboardEvent::CtrlShift   (Key::ArrowUp      )),
            "\u{1b}[1;6D"     => Some(KeyboardEvent::CtrlShift   (Key::ArrowLeft    )),
            "\u{1b}[1;6B"     => Some(KeyboardEvent::CtrlShift   (Key::ArrowDown    )),
            "\u{1b}[1;6C"     => Some(KeyboardEvent::CtrlShift   (Key::ArrowRight   )),

            "\u{1b}[27;4;13~" => Some(KeyboardEvent::AltShift    (Key::Enter        )),
            "\u{1b}[27;4;27~" => Some(KeyboardEvent::AltShift    (Key::Escape       )),
            "\u{1b}[1;4P"     => Some(KeyboardEvent::AltShift    (Key::F1           )),
            "\u{1b}[1;4Q"     => Some(KeyboardEvent::AltShift    (Key::F2           )),
            "\u{1b}[1;4R"     => Some(KeyboardEvent::AltShift    (Key::F3           )),
            "\u{1b}[1;4S"     => Some(KeyboardEvent::AltShift    (Key::F4           )),
            "\u{1b}[15;4~"    => Some(KeyboardEvent::AltShift    (Key::F5           )),
            "\u{1b}[17;4~"    => Some(KeyboardEvent::AltShift    (Key::F6           )),
            "\u{1b}[18;4~"    => Some(KeyboardEvent::AltShift    (Key::F7           )),
            "\u{1b}[19;4~"    => Some(KeyboardEvent::AltShift    (Key::F8           )),
            "\u{1b}[20;4~"    => Some(KeyboardEvent::AltShift    (Key::F9           )),
            "\u{1b}[21;4~"    => Some(KeyboardEvent::AltShift    (Key::F10          )),
            "\u{1b}[23;4~"    => Some(KeyboardEvent::AltShift    (Key::F11          )),
            "\u{1b}[24;4~"    => Some(KeyboardEvent::AltShift    (Key::F12          )),
            "\u{1b}[27;4;9~"  => Some(KeyboardEvent::AltShift    (Key::Tab          )),
            "\u{1b}[3;4~"     => Some(KeyboardEvent::AltShift    (Key::Delete       )),
            "\u{1b}[1;4H"     => Some(KeyboardEvent::AltShift    (Key::Home         )),
            "\u{1b}[1;4F"     => Some(KeyboardEvent::AltShift    (Key::End          )),
            "\u{1b}[5;4~"     => Some(KeyboardEvent::AltShift    (Key::PageUp       )),
            "\u{1b}[6;4~"     => Some(KeyboardEvent::AltShift    (Key::PageDown     )),
            "\u{1b}[1;4A"     => Some(KeyboardEvent::AltShift    (Key::ArrowUp      )),
            "\u{1b}[1;4D"     => Some(KeyboardEvent::AltShift    (Key::ArrowLeft    )),
            "\u{1b}[1;4B"     => Some(KeyboardEvent::AltShift    (Key::ArrowDown    )),
            "\u{1b}[1;4C"     => Some(KeyboardEvent::AltShift    (Key::ArrowRight   )),

            "\u{1b}[27;8;13~" => Some(KeyboardEvent::CtrlAltShift(Key::Enter        )),
            "\u{1b}[27;8;27~" => Some(KeyboardEvent::CtrlAltShift(Key::Escape       )),
            "\u{1b}[1;8P"     => Some(KeyboardEvent::CtrlAltShift(Key::F1           )),
            "\u{1b}[1;8Q"     => Some(KeyboardEvent::CtrlAltShift(Key::F2           )),
            "\u{1b}[1;8R"     => Some(KeyboardEvent::CtrlAltShift(Key::F3           )),
            "\u{1b}[1;8S"     => Some(KeyboardEvent::CtrlAltShift(Key::F4           )),
            "\u{1b}[15;8~"    => Some(KeyboardEvent::CtrlAltShift(Key::F5           )),
            "\u{1b}[17;8~"    => Some(KeyboardEvent::CtrlAltShift(Key::F6           )),
            "\u{1b}[18;8~"    => Some(KeyboardEvent::CtrlAltShift(Key::F7           )),
            "\u{1b}[19;8~"    => Some(KeyboardEvent::CtrlAltShift(Key::F8           )),
            "\u{1b}[20;8~"    => Some(KeyboardEvent::CtrlAltShift(Key::F9           )),
            "\u{1b}[21;8~"    => Some(KeyboardEvent::CtrlAltShift(Key::F10          )),
            "\u{1b}[23;8~"    => Some(KeyboardEvent::CtrlAltShift(Key::F11          )),
            "\u{1b}[24;8~"    => Some(KeyboardEvent::CtrlAltShift(Key::F12          )),
            "\u{1b}[27;8;9~"  => Some(KeyboardEvent::CtrlAltShift(Key::Tab          )),
            "\u{1b}[3;8~"     => Some(KeyboardEvent::CtrlAltShift(Key::Delete       )),
            "\u{1b}[1;8H"     => Some(KeyboardEvent::CtrlAltShift(Key::Home         )),
            "\u{1b}[1;8F"     => Some(KeyboardEvent::CtrlAltShift(Key::End          )),
            "\u{1b}[5;8~"     => Some(KeyboardEvent::CtrlAltShift(Key::PageUp       )),
            "\u{1b}[6;8~"     => Some(KeyboardEvent::CtrlAltShift(Key::PageDown     )),
            "\u{1b}[1;8A"     => Some(KeyboardEvent::CtrlAltShift(Key::ArrowUp      )),
            "\u{1b}[1;8D"     => Some(KeyboardEvent::CtrlAltShift(Key::ArrowLeft    )),
            "\u{1b}[1;8B"     => Some(KeyboardEvent::CtrlAltShift(Key::ArrowDown    )),
            "\u{1b}[1;8C"     => Some(KeyboardEvent::CtrlAltShift(Key::ArrowRight   )),

            _                 => None
        }
    }

    fn mouse_event(string: &str) -> Option<MouseEvent> {
        const PREFIX: &str = "\u{1b}[<";

        if          !string.starts_with(PREFIX) { return None; }
        let i      = string.find(';')?;
        let action = &string[PREFIX.len()..i];
        let j      = string[i+1..].find(';')?;
        let x      = &string[i+1..i+1+j];
        let end    = &string[i+1+j+1..];
        let m      = &end.chars().last().unwrap();
        let y      = &end[..end.len()-1];
        let Ok(x)  = x.parse::<u16>() else { return None; };
        let Ok(y)  = y.parse::<u16>() else { return None; };
        let x      = x - 1;
        let y      = y - 1;

        match m {
            'M' => {
                match action {
                    "64"  => Some(MouseEvent::Scroll(     ScrollEvent::NoModifiers(ScrollDirection::     Up(x, y)))),
                    "65"  => Some(MouseEvent::Scroll(     ScrollEvent::NoModifiers(ScrollDirection::   Down(x, y)))),
                    "80"  => Some(MouseEvent::Scroll(     ScrollEvent::    Ctrl   (ScrollDirection::     Up(x, y)))),
                    "81"  => Some(MouseEvent::Scroll(     ScrollEvent::    Ctrl   (ScrollDirection::   Down(x, y)))),
                    "72"  => Some(MouseEvent::Scroll(     ScrollEvent::        Alt(ScrollDirection::     Up(x, y)))),
                    "73"  => Some(MouseEvent::Scroll(     ScrollEvent::        Alt(ScrollDirection::   Down(x, y)))),
                    "88"  => Some(MouseEvent::Scroll(     ScrollEvent::    CtrlAlt(ScrollDirection::     Up(x, y)))),
                    "89"  => Some(MouseEvent::Scroll(     ScrollEvent::    CtrlAlt(ScrollDirection::   Down(x, y)))),

                    "35"  => Some(MouseEvent:: Hover(      HoverEvent::NoModifiers                         (x, y))),
                    "51"  => Some(MouseEvent:: Hover(      HoverEvent::    Ctrl                            (x, y))),
                    "43"  => Some(MouseEvent:: Hover(      HoverEvent::        Alt                         (x, y))),
                    "59"  => Some(MouseEvent:: Hover(      HoverEvent::    CtrlAlt                         (x, y))),

                    "32"  => Some(MouseEvent::  Drag(MouseButtonEvent::NoModifiers(    MouseButton::   Left(x, y)))),
                    "48"  => Some(MouseEvent::  Drag(MouseButtonEvent::    Ctrl   (    MouseButton::   Left(x, y)))),
                    "40"  => Some(MouseEvent::  Drag(MouseButtonEvent::        Alt(    MouseButton::   Left(x, y)))),
                    "56"  => Some(MouseEvent::  Drag(MouseButtonEvent::    CtrlAlt(    MouseButton::   Left(x, y)))),

                    "33"  => Some(MouseEvent::  Drag(MouseButtonEvent::NoModifiers(    MouseButton:: Middle(x, y)))),
                    "49"  => Some(MouseEvent::  Drag(MouseButtonEvent::    Ctrl   (    MouseButton:: Middle(x, y)))),
                    "41"  => Some(MouseEvent::  Drag(MouseButtonEvent::        Alt(    MouseButton:: Middle(x, y)))),
                    "57"  => Some(MouseEvent::  Drag(MouseButtonEvent::    CtrlAlt(    MouseButton:: Middle(x, y)))),

                    "34"  => Some(MouseEvent::  Drag(MouseButtonEvent::NoModifiers(    MouseButton::  Right(x, y)))),
                    "50"  => Some(MouseEvent::  Drag(MouseButtonEvent::    Ctrl   (    MouseButton::  Right(x, y)))),
                    "42"  => Some(MouseEvent::  Drag(MouseButtonEvent::        Alt(    MouseButton::  Right(x, y)))),
                    "58"  => Some(MouseEvent::  Drag(MouseButtonEvent::    CtrlAlt(    MouseButton::  Right(x, y)))),

                    "160" => Some(MouseEvent::  Drag(MouseButtonEvent::NoModifiers(    MouseButton::   Back(x, y)))),
                    "176" => Some(MouseEvent::  Drag(MouseButtonEvent::    Ctrl   (    MouseButton::   Back(x, y)))),
                    "168" => Some(MouseEvent::  Drag(MouseButtonEvent::        Alt(    MouseButton::   Back(x, y)))),
                    "184" => Some(MouseEvent::  Drag(MouseButtonEvent::    CtrlAlt(    MouseButton::   Back(x, y)))),

                    "161" => Some(MouseEvent::  Drag(MouseButtonEvent::NoModifiers(    MouseButton::Forward(x, y)))),
                    "177" => Some(MouseEvent::  Drag(MouseButtonEvent::    Ctrl   (    MouseButton::Forward(x, y)))),
                    "169" => Some(MouseEvent::  Drag(MouseButtonEvent::        Alt(    MouseButton::Forward(x, y)))),
                    "185" => Some(MouseEvent::  Drag(MouseButtonEvent::    CtrlAlt(    MouseButton::Forward(x, y)))),

                    "0"   => Some(MouseEvent:: Press(MouseButtonEvent::NoModifiers(    MouseButton::   Left(x, y)))),
                    "16"  => Some(MouseEvent:: Press(MouseButtonEvent::    Ctrl   (    MouseButton::   Left(x, y)))),
                    "8"   => Some(MouseEvent:: Press(MouseButtonEvent::        Alt(    MouseButton::   Left(x, y)))),
                    "24"  => Some(MouseEvent:: Press(MouseButtonEvent::    CtrlAlt(    MouseButton::   Left(x, y)))),

                    "1"   => Some(MouseEvent:: Press(MouseButtonEvent::NoModifiers(    MouseButton:: Middle(x, y)))),
                    "17"  => Some(MouseEvent:: Press(MouseButtonEvent::    Ctrl   (    MouseButton:: Middle(x, y)))),
                    "9"   => Some(MouseEvent:: Press(MouseButtonEvent::        Alt(    MouseButton:: Middle(x, y)))),
                    "25"  => Some(MouseEvent:: Press(MouseButtonEvent::    CtrlAlt(    MouseButton:: Middle(x, y)))),

                    "2"   => Some(MouseEvent:: Press(MouseButtonEvent::NoModifiers(    MouseButton::  Right(x, y)))),
                    "18"  => Some(MouseEvent:: Press(MouseButtonEvent::    Ctrl   (    MouseButton::  Right(x, y)))),
                    "10"  => Some(MouseEvent:: Press(MouseButtonEvent::        Alt(    MouseButton::  Right(x, y)))),
                    "26"  => Some(MouseEvent:: Press(MouseButtonEvent::    CtrlAlt(    MouseButton::  Right(x, y)))),

                    "128" => Some(MouseEvent:: Press(MouseButtonEvent::NoModifiers(    MouseButton::   Back(x, y)))),
                    "144" => Some(MouseEvent:: Press(MouseButtonEvent::    Ctrl   (    MouseButton::   Back(x, y)))),
                    "136" => Some(MouseEvent:: Press(MouseButtonEvent::        Alt(    MouseButton::   Back(x, y)))),
                    "152" => Some(MouseEvent:: Press(MouseButtonEvent::    CtrlAlt(    MouseButton::   Back(x, y)))),

                    "129" => Some(MouseEvent:: Press(MouseButtonEvent::NoModifiers(    MouseButton::Forward(x, y)))),
                    "145" => Some(MouseEvent:: Press(MouseButtonEvent::    Ctrl   (    MouseButton::Forward(x, y)))),
                    "137" => Some(MouseEvent:: Press(MouseButtonEvent::        Alt(    MouseButton::Forward(x, y)))),
                    "153" => Some(MouseEvent:: Press(MouseButtonEvent::    CtrlAlt(    MouseButton::Forward(x, y)))),

                    _     => None
                }
            },
            'm' => {
                match action {
                    "0"   => Some(MouseEvent::Release(MouseButtonEvent::NoModifiers(MouseButton::   Left(x, y)))),
                    "16"  => Some(MouseEvent::Release(MouseButtonEvent::    Ctrl   (MouseButton::   Left(x, y)))),
                    "8"   => Some(MouseEvent::Release(MouseButtonEvent::        Alt(MouseButton::   Left(x, y)))),
                    "24"  => Some(MouseEvent::Release(MouseButtonEvent::    CtrlAlt(MouseButton::   Left(x, y)))),

                    "1"   => Some(MouseEvent::Release(MouseButtonEvent::NoModifiers(MouseButton:: Middle(x, y)))),
                    "17"  => Some(MouseEvent::Release(MouseButtonEvent::    Ctrl   (MouseButton:: Middle(x, y)))),
                    "9"   => Some(MouseEvent::Release(MouseButtonEvent::        Alt(MouseButton:: Middle(x, y)))),
                    "25"  => Some(MouseEvent::Release(MouseButtonEvent::    CtrlAlt(MouseButton:: Middle(x, y)))),

                    "2"   => Some(MouseEvent::Release(MouseButtonEvent::NoModifiers(MouseButton::  Right(x, y)))),
                    "18"  => Some(MouseEvent::Release(MouseButtonEvent::    Ctrl   (MouseButton::  Right(x, y)))),
                    "10"  => Some(MouseEvent::Release(MouseButtonEvent::        Alt(MouseButton::  Right(x, y)))),
                    "26"  => Some(MouseEvent::Release(MouseButtonEvent::    CtrlAlt(MouseButton::  Right(x, y)))),

                    "128" => Some(MouseEvent::Release(MouseButtonEvent::NoModifiers(MouseButton::   Back(x, y)))),
                    "144" => Some(MouseEvent::Release(MouseButtonEvent::    Ctrl   (MouseButton::   Back(x, y)))),
                    "136" => Some(MouseEvent::Release(MouseButtonEvent::        Alt(MouseButton::   Back(x, y)))),
                    "152" => Some(MouseEvent::Release(MouseButtonEvent::    CtrlAlt(MouseButton::   Back(x, y)))),

                    "129" => Some(MouseEvent::Release(MouseButtonEvent::NoModifiers(MouseButton::Forward(x, y)))),
                    "145" => Some(MouseEvent::Release(MouseButtonEvent::    Ctrl   (MouseButton::Forward(x, y)))),
                    "137" => Some(MouseEvent::Release(MouseButtonEvent::        Alt(MouseButton::Forward(x, y)))),
                    "153" => Some(MouseEvent::Release(MouseButtonEvent::    CtrlAlt(MouseButton::Forward(x, y)))),

                    _     => None
                }
            },
            _ => None
        }
    }

    fn other_event<T>(string: String) -> Event<T> {
        match string.chars().count() {
            1 => {
                return Event::Keyboard(
                    KeyboardEvent::Char(
                        string.chars().nth(0).unwrap()
                    )
                );
            },
            2 => {
                if string.chars().nth(0).unwrap() == '\u{1b}' {
                    return Event::Keyboard(
                        KeyboardEvent::AltChar(
                            string.chars().nth(1).unwrap()
                        )
                    );
                }
            },
            3 => {
                match string.as_str() {
                    "\u{1b}[I" => { return Event::FocusGained; },
                    "\u{1b}[O" => { return Event::FocusLost;   },
                    _          => ()
                }
            },
            _ => ()
        }

        Event::Unimplemented(string)
    }

    // TODO: breaks when multiple events are read at one time
    /// waits forever until something happens
    pub fn blocking_event<T>(&mut self) -> Event<T> {
        let string = self.read_event_to_string();

        if let Some(keyboard_event) = self.keyboard_event(&string) {
            return Event::Keyboard(keyboard_event);
        }

        if let Some(mouse_event) = Self::mouse_event(&string) {
            return Event::Mouse(mouse_event);
        }

        Self::other_event(string)
    }
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        let _ = self.write_all(
            format!(
                "{}{}{}{}{}{}",
                "\x1b[?2004l", // disable bracketed paste
                "\x1b[?1004l", // disable focus reporting
                "\x1b[?1006l", // reduce mouse support
                "\x1b[?1003l", // disable mouse events
                "\x1b[?1049l", // exit alternate screen
                "\x1b[?25h"    // show cursor
            ).as_bytes()
        );

        let _ = self.flush();

        assert_eq!(unsafe { libc::tcsetattr(self.input.as_raw_fd(), libc::TCSAFLUSH, &self.old_termios) }, 0);
    }
}

#[expect(missing_docs)]
#[derive(Debug, Clone)]
pub enum Event<T = ()> {
    Keyboard(KeyboardEvent),
    Mouse   (   MouseEvent),

    FocusGained,
    FocusLost,

    /// returns what was read from input when betterm cannot make sense of it,  
    /// if its relevant, you can submit an issue/pr
    Unimplemented(String),

    /// for your custom needs!
    Custom(T)
}

#[expect(missing_docs)]
#[derive(Debug, Clone)]
pub enum KeyboardEvent {
    /**/ NoModifiers(Key),
    /**/Ctrl        (Key),
    /**/    Alt     (Key),
    /**/       Shift(Key),
    /**/CtrlAlt     (Key),
    /**/CtrlShift   (Key),
    /**/    AltShift(Key),
    /**/CtrlAltShift(Key),

    /// characters are not in [`Key`], because some modifiers for typeable characters  
    /// produce locale specific characters (`Shift` and `AltGr`)
    /*       */Char(           char),
    /*    */AltChar(           char),
    /*   */CtrlChar(   CtrlableChar),
    /**/CtrlAltChar(CtrlAltableChar),

    /// `Backspace` is not in [`Key`], since these do not work in terminals:  
    /// - `Shift + Backspace`  
    /// - `Ctrl + Shift + Backspace`  
    /// - `Alt + Shift + Backspace`  
    /// - `Ctrl + Alt + Shift + Backspace`
    /*       */Backspace,
    /*   */CtrlBackspace,
    /*    */AltBackspace,
    /**/CtrlAltBackspace,

    /// `Insert` is not in [`Key`], because `Shift` is used for bracketed paste
    /*            */Insert,
    /*        */CtrlInsert,
    /*         */AltInsert,
    /*     */CtrlAltInsert,
    /*   */CtrlShiftInsert,
    /*    */AltShiftInsert,
    /**/CtrlAltShiftInsert,

    /// this is the paste from your clipboard
    Pasted(String)
}

#[expect(missing_docs)]
#[derive(Debug, Clone)]
pub enum MouseEvent {
    Scroll (     ScrollEvent),
    Hover  (      HoverEvent),
    Drag   (MouseButtonEvent),
    Press  (MouseButtonEvent),
    Release(MouseButtonEvent)
}

impl MouseEvent {
    /// useful when trying to determining focus without verbose pattern matching
    pub fn cell(&self) -> (u16, u16) {
        match self {
            Self::Scroll( se)                                         =>  se.cell(),
            Self::Hover ( he)                                         =>  he.cell(),
            Self::Drag  (mbe) | Self::Press(mbe) | Self::Release(mbe) => mbe.cell()
        }
    }

    /// useful when trying to get a relative position
    pub fn correct_by(&mut self, xoff: u16, yoff: u16) {
        match self {
            Self::Scroll( se)                                         =>  se.correct_by(xoff, yoff),
            Self::Hover ( he)                                         =>  he.correct_by(xoff, yoff),
            Self::Drag  (mbe) | Self::Press(mbe) | Self::Release(mbe) => mbe.correct_by(xoff, yoff)
        }
    }
}

#[expect(missing_docs)]
#[derive(Debug, Clone)]
pub enum ScrollEvent {
    /**/NoModifiers(ScrollDirection),
    /**/    Ctrl   (ScrollDirection),
    /**/        Alt(ScrollDirection),
    /**/    CtrlAlt(ScrollDirection)
}

impl ScrollEvent {
    /// useful when trying to determining focus without verbose pattern matching
    pub fn cell(&self) -> (u16, u16) {
        match self {
            Self::NoModifiers(sd)
                | Self::Ctrl   (sd)
                | Self::    Alt(sd)
                | Self::CtrlAlt(sd)
            => sd.cell()
        }
    }

    /// useful when trying to get a relative position
    pub fn correct_by(&mut self, xoff: u16, yoff: u16) {
        match self {
            Self::NoModifiers(sd)
                | Self::Ctrl   (sd)
                | Self::    Alt(sd)
                | Self::CtrlAlt(sd)
            => sd.correct_by(xoff, yoff)
        }
    }
}

#[expect(missing_docs)]
#[derive(Debug, Clone)]
pub enum ScrollDirection {
    Up  (u16, u16),
    Down(u16, u16)
}

impl ScrollDirection {
    /// useful when trying to determining focus without verbose pattern matching
    pub fn cell(&self) -> (u16, u16) {
        match self {
            Self::Up(x, y) | Self::Down(x, y) => (*x, *y)
        }
    }

    /// useful when trying to get a relative position
    pub fn correct_by(&mut self, xoff: u16, yoff: u16) {
        match self {
            Self::Up(x, y) | Self::Down(x, y) => {
                *x -= xoff;
                *y -= yoff;
            }
        }
    }
}

#[expect(missing_docs)]
#[derive(Debug, Clone)]
pub enum HoverEvent {
    /**/NoModifiers(u16, u16),
    /**/    Ctrl   (u16, u16),
    /**/        Alt(u16, u16),
    /**/    CtrlAlt(u16, u16)
}

impl HoverEvent {
    /// useful when trying to determining focus without verbose pattern matching
    pub fn cell(&self) -> (u16, u16) {
        match self {
            Self::NoModifiers(x, y)
                | Self::Ctrl   (x, y)
                | Self::    Alt(x, y)
                | Self::CtrlAlt(x, y)
            => (*x, *y)
        }
    }

    /// useful when trying to get a relative position
    pub fn correct_by(&mut self, xoff: u16, yoff: u16) {
        match self {
            Self::NoModifiers(x, y)
                | Self::Ctrl   (x, y)
                | Self::    Alt(x, y)
                | Self::CtrlAlt(x, y)
            => {
                *x -= xoff;
                *y -= yoff;
            }
        }
    }
}

#[expect(missing_docs)]
#[derive(Debug, Clone)]
pub enum MouseButtonEvent {
    /**/NoModifiers(MouseButton),
    /**/    Ctrl   (MouseButton),
    /**/        Alt(MouseButton),
    /**/    CtrlAlt(MouseButton)
}

impl MouseButtonEvent {
    /// useful when trying to determining focus without verbose pattern matching
    pub fn cell(&self) -> (u16, u16) {
        match self {
            Self::NoModifiers(mb)
                | Self::Ctrl   (mb)
                | Self::    Alt(mb)
                | Self::CtrlAlt(mb)
            => mb.cell()
        }
    }

    /// useful when trying to get a relative position
    pub fn correct_by(&mut self, xoff: u16, yoff: u16) {
        match self {
            Self::NoModifiers(mb)
                | Self::Ctrl   (mb)
                | Self::    Alt(mb)
                | Self::CtrlAlt(mb)
            => mb.correct_by(xoff, yoff)
        }
    }
}

#[expect(missing_docs)]
#[derive(Debug, Clone)]
pub enum MouseButton {
    Left   (u16, u16),
    Middle (u16, u16),
    Right  (u16, u16),
    Back   (u16, u16),
    Forward(u16, u16)
}

impl MouseButton {
    /// useful when trying to determining focus without verbose pattern matching
    pub fn cell(&self) -> (u16, u16) {
        match self {
            Self::Left(x, y)
                | Self::Middle (x, y)
                | Self::Right  (x, y)
                | Self::Back   (x, y)
                | Self::Forward(x, y)
            => (*x, *y)
        }
    }

    /// useful when trying to get a relative position
    pub fn correct_by(&mut self, xoff: u16, yoff: u16) {
        match self {
            Self::Left(x, y)
                | Self::Middle (x, y)
                | Self::Right  (x, y)
                | Self::Back   (x, y)
                | Self::Forward(x, y)
            => {
                *x -= xoff;
                *y -= yoff;
            }
        }
    }
}

/// these events are compatible with all combinations of modifiers:  
/// - none  
/// - `Ctrl`  
/// - `Alt`  
/// - `Shift`  
/// - `Ctrl + Alt`  
/// - `Ctrl + Shift`  
/// - `Alt + Shift`  
/// - `Ctrl + Alt + Shift`
#[expect(missing_docs)]
#[derive(Debug, Clone)]
pub enum Key {
    Enter, // NOTE: deliberately not `Char('\n')`
    Tab,   // NOTE: deliberately not `Char('\t')`
    Escape,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Delete,
    Home, End,
    PageUp, PageDown,
    ArrowUp, ArrowDown, ArrowRight, ArrowLeft
}

// ------------------------------------------------------------------------------------------------------------------ //

/// includes the whole alphabet, except: `H`, `I`, `J`, `M`  
///
/// those 4 just do not work properly in terminals,  
/// since they produce duplicate bytes,  
/// where i have prioritized the other meaning:  
/// - `Ctrl + H` = `Ctrl + Backspace`  
/// - `Ctrl + I` = `Tab`  
/// - `Ctrl + J` = `Enter`  
/// - `Ctrl + M` = `Enter`
#[expect(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum CtrlableChar {
    A, B, C, D, E, F, G, K, L, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
}

/// includes the whole alphabet, except: `H`, `I`, `M`  
///
/// those 3 just do not work properly in terminals,  
/// since they produce duplicate bytes,  
/// where i have prioritized the other meaning:  
/// - `Ctrl + Alt + H` = `Ctrl + Alt + Backspace`  
/// - `Ctrl + Alt + I` = `Alt + Tab`  
/// - `Ctrl + Alt + M` = `Alt + Enter`
#[expect(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum CtrlAltableChar {
    A, B, C, D, E, F, G, J, K, L, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
}

