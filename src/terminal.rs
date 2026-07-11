// mochou-p/betterm/src/terminal.rs

use std::io::{self, Write, Read};
use std::os::fd::AsRawFd;


/// makes all terminal state raw through libc::cfmakeraw,  
/// makes input block waiting for atleast 1 byte when read,  
/// enters alternate screen,  
/// clears the whole screen,  
/// enables all mouse events,  
/// enables focus reporting,  
/// enables bracketed paste mode,  
/// moves the cursor to the top left corner  
/// (these will be customizable soon)
pub struct RawTerminal {
    old_termios: libc::termios,
    input:       Box<dyn ReadAndAsRawFd>,
    output:      Box<dyn Write>
}

/// combines all required traits of input into one trait
pub trait ReadAndAsRawFd: Read + AsRawFd {}

impl<T: Read + AsRawFd> ReadAndAsRawFd for T {}

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
    /// turns input and output into raw terminal state,  
    /// assuming stdin and stdout compatibility
    pub fn new(input: impl ReadAndAsRawFd + 'static, mut output: impl Write + 'static) -> Self {
        let mut maybe_termios = std::mem::MaybeUninit::uninit();
        assert_eq!(unsafe { libc::tcgetattr(input.as_raw_fd(), maybe_termios.as_mut_ptr()) }, 0);

        let mut termios = unsafe { maybe_termios.assume_init() };
        let old_termios = termios;

        unsafe { libc::cfmakeraw(&mut termios); }
        termios.c_cc[libc::VMIN ] = 1;
        termios.c_cc[libc::VTIME] = 0;

        unsafe { libc::tcsetattr(input.as_raw_fd(), libc::TCSAFLUSH, &termios); }

        output.write_all(
            format!(
                "{}{}{}{}{}{}{}",
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

    // NOTE: all my homies hate advanced ANSI escapes
    /// waits forever until something happens
    pub fn blocking_event(&mut self) -> Event {
        let string = self.read_event_to_string();

        match string.as_str() {
            "\u{7f}"          => { return Event::                             Backspace ; },
            "\u{8}"           => { return Event::                         CtrlBackspace ; },
            "\u{1b}\u{7f}"    => { return Event::                          AltBackspace ; },
            "\u{1b}\u{8}"     => { return Event::                      CtrlAltBackspace ; },

            "\u{1b}[I"        => { return Event::                      FocusGained      ; },
            "\u{1b}[O"        => { return Event::                      FocusLost        ; },

            "\r" | "\n"       => { return Event::NoModifiers (Eventee::Enter           ); },
            "\u{1b}"          => { return Event::NoModifiers (Eventee::Escape          ); },
            "\u{1b}OP"        => { return Event::NoModifiers (Eventee::F1              ); },
            "\u{1b}OQ"        => { return Event::NoModifiers (Eventee::F2              ); },
            "\u{1b}OR"        => { return Event::NoModifiers (Eventee::F3              ); },
            "\u{1b}OS"        => { return Event::NoModifiers (Eventee::F4              ); },
            "\u{1b}[15~"      => { return Event::NoModifiers (Eventee::F5              ); },
            "\u{1b}[17~"      => { return Event::NoModifiers (Eventee::F6              ); },
            "\u{1b}[18~"      => { return Event::NoModifiers (Eventee::F7              ); },
            "\u{1b}[19~"      => { return Event::NoModifiers (Eventee::F8              ); },
            "\u{1b}[20~"      => { return Event::NoModifiers (Eventee::F9              ); },
            "\u{1b}[21~"      => { return Event::NoModifiers (Eventee::F10             ); },
            "\u{1b}[23~"      => { return Event::NoModifiers (Eventee::F11             ); },
            "\u{1b}[24~"      => { return Event::NoModifiers (Eventee::F12             ); },
            "\t"              => { return Event::NoModifiers (Eventee::Tab             ); },
            "\u{1b}[2~"       => { return Event::NoModifiers (Eventee::Insert          ); },
            "\u{1b}[3~"       => { return Event::NoModifiers (Eventee::Delete          ); },
            "\u{1b}[H"        => { return Event::NoModifiers (Eventee::Home            ); },
            "\u{1b}[F"        => { return Event::NoModifiers (Eventee::End             ); },
            "\u{1b}[5~"       => { return Event::NoModifiers (Eventee::PageUp          ); },
            "\u{1b}[6~"       => { return Event::NoModifiers (Eventee::PageDown        ); },
            "\u{1b}[A"        => { return Event::NoModifiers (Eventee::ArrowUp         ); },
            "\u{1b}[D"        => { return Event::NoModifiers (Eventee::ArrowLeft       ); },
            "\u{1b}[B"        => { return Event::NoModifiers (Eventee::ArrowDown       ); },
            "\u{1b}[C"        => { return Event::NoModifiers (Eventee::ArrowRight      ); },

            "\u{1b}[27;5;13~" => { return Event::Ctrl        (Eventee::Enter           ); },
            "\u{1b}[27;5;27~" => { return Event::Ctrl        (Eventee::Escape          ); },
            "\u{1b}[1;5P"     => { return Event::Ctrl        (Eventee::F1              ); },
            "\u{1b}[1;5Q"     => { return Event::Ctrl        (Eventee::F2              ); },
            "\u{1b}[1;5R"     => { return Event::Ctrl        (Eventee::F3              ); },
            "\u{1b}[1;5S"     => { return Event::Ctrl        (Eventee::F4              ); },
            "\u{1b}[15;5~"    => { return Event::Ctrl        (Eventee::F5              ); },
            "\u{1b}[17;5~"    => { return Event::Ctrl        (Eventee::F6              ); },
            "\u{1b}[18;5~"    => { return Event::Ctrl        (Eventee::F7              ); },
            "\u{1b}[19;5~"    => { return Event::Ctrl        (Eventee::F8              ); },
            "\u{1b}[20;5~"    => { return Event::Ctrl        (Eventee::F9              ); },
            "\u{1b}[21;5~"    => { return Event::Ctrl        (Eventee::F10             ); },
            "\u{1b}[23;5~"    => { return Event::Ctrl        (Eventee::F11             ); },
            "\u{1b}[24;5~"    => { return Event::Ctrl        (Eventee::F12             ); },
            "\u{1b}[27;5;9~"  => { return Event::Ctrl        (Eventee::Tab             ); },
            "\u{1b}[2;5~"     => { return Event::Ctrl        (Eventee::Insert          ); },
            "\u{1b}[3;5~"     => { return Event::Ctrl        (Eventee::Delete          ); },
            "\u{1b}[1;5H"     => { return Event::Ctrl        (Eventee::Home            ); },
            "\u{1b}[1;5F"     => { return Event::Ctrl        (Eventee::End             ); },
            "\u{1b}[5;5~"     => { return Event::Ctrl        (Eventee::PageUp          ); },
            "\u{1b}[6;5~"     => { return Event::Ctrl        (Eventee::PageDown        ); },
            "\u{1b}[1;5A"     => { return Event::Ctrl        (Eventee::ArrowUp         ); },
            "\u{1b}[1;5D"     => { return Event::Ctrl        (Eventee::ArrowLeft       ); },
            "\u{1b}[1;5B"     => { return Event::Ctrl        (Eventee::ArrowDown       ); },
            "\u{1b}[1;5C"     => { return Event::Ctrl        (Eventee::ArrowRight      ); },

            "\u{1b}\r"        => { return Event::Alt         (Eventee::Enter           ); },
            "\u{1b}\u{1b}"    => { return Event::Alt         (Eventee::Escape          ); },
            "\u{1b}[1;3P"     => { return Event::Alt         (Eventee::F1              ); },
            "\u{1b}[1;3Q"     => { return Event::Alt         (Eventee::F2              ); },
            "\u{1b}[1;3R"     => { return Event::Alt         (Eventee::F3              ); },
            "\u{1b}[1;3S"     => { return Event::Alt         (Eventee::F4              ); },
            "\u{1b}[15;3~"    => { return Event::Alt         (Eventee::F5              ); },
            "\u{1b}[17;3~"    => { return Event::Alt         (Eventee::F6              ); },
            "\u{1b}[18;3~"    => { return Event::Alt         (Eventee::F7              ); },
            "\u{1b}[19;3~"    => { return Event::Alt         (Eventee::F8              ); },
            "\u{1b}[20;3~"    => { return Event::Alt         (Eventee::F9              ); },
            "\u{1b}[21;3~"    => { return Event::Alt         (Eventee::F10             ); },
            "\u{1b}[23;3~"    => { return Event::Alt         (Eventee::F11             ); },
            "\u{1b}[24;3~"    => { return Event::Alt         (Eventee::F12             ); },
            "\u{1b}\t"        => { return Event::Alt         (Eventee::Tab             ); },
            "\u{1b}[2;3~"     => { return Event::Alt         (Eventee::Insert          ); },
            "\u{1b}[3;3~"     => { return Event::Alt         (Eventee::Delete          ); },
            "\u{1b}[1;3H"     => { return Event::Alt         (Eventee::Home            ); },
            "\u{1b}[1;3F"     => { return Event::Alt         (Eventee::End             ); },
            "\u{1b}[5;3~"     => { return Event::Alt         (Eventee::PageUp          ); },
            "\u{1b}[6;3~"     => { return Event::Alt         (Eventee::PageDown        ); },
            "\u{1b}[1;3A"     => { return Event::Alt         (Eventee::ArrowUp         ); },
            "\u{1b}[1;3D"     => { return Event::Alt         (Eventee::ArrowLeft       ); },
            "\u{1b}[1;3B"     => { return Event::Alt         (Eventee::ArrowDown       ); },
            "\u{1b}[1;3C"     => { return Event::Alt         (Eventee::ArrowRight      ); },

            "\u{1b}[27;2;13~" => { return Event::Shift       (Eventee::Enter           ); },
            "\u{1b}[27;2;27~" => { return Event::Shift       (Eventee::Escape          ); },
            "\u{1b}[1;2P"     => { return Event::Shift       (Eventee::F1              ); },
            "\u{1b}[1;2Q"     => { return Event::Shift       (Eventee::F2              ); },
            "\u{1b}[1;2R"     => { return Event::Shift       (Eventee::F3              ); },
            "\u{1b}[1;2S"     => { return Event::Shift       (Eventee::F4              ); },
            "\u{1b}[15;2~"    => { return Event::Shift       (Eventee::F5              ); },
            "\u{1b}[17;2~"    => { return Event::Shift       (Eventee::F6              ); },
            "\u{1b}[18;2~"    => { return Event::Shift       (Eventee::F7              ); },
            "\u{1b}[19;2~"    => { return Event::Shift       (Eventee::F8              ); },
            "\u{1b}[20;2~"    => { return Event::Shift       (Eventee::F9              ); },
            "\u{1b}[21;2~"    => { return Event::Shift       (Eventee::F10             ); },
            "\u{1b}[23;2~"    => { return Event::Shift       (Eventee::F11             ); },
            "\u{1b}[24;2~"    => { return Event::Shift       (Eventee::F12             ); },
            "\u{1b}[Z"        => { return Event::Shift       (Eventee::Tab             ); },
            "\u{1b}[200~"     => { let string2  = self.read_event_to_string();
                                   if  string2 != "\u{1b}[201~" {
                                       return Event::Unimplemented(
                                           format!("{string}{string2}")
                                       );
                                   }
                                   return Event::Shift       (Eventee::Insert          ); },
            "\u{1b}[3;2~"     => { return Event::Shift       (Eventee::Delete          ); },
            "\u{1b}[1;2H"     => { return Event::Shift       (Eventee::Home            ); },
            "\u{1b}[1;2F"     => { return Event::Shift       (Eventee::End             ); },
            "\u{1b}[5;2~"     => { return Event::Shift       (Eventee::PageUp          ); },
            "\u{1b}[6;2~"     => { return Event::Shift       (Eventee::PageDown        ); },
            "\u{1b}[1;2A"     => { return Event::Shift       (Eventee::ArrowUp         ); },
            "\u{1b}[1;2D"     => { return Event::Shift       (Eventee::ArrowLeft       ); },
            "\u{1b}[1;2B"     => { return Event::Shift       (Eventee::ArrowDown       ); },
            "\u{1b}[1;2C"     => { return Event::Shift       (Eventee::ArrowRight      ); },

            "\u{1b}[27;7;13~" => { return Event::CtrlAlt     (Eventee::Enter           ); },
            "\u{1b}[27;7;27~" => { return Event::CtrlAlt     (Eventee::Escape          ); },
            "\u{1b}[1;7P"     => { return Event::CtrlAlt     (Eventee::F1              ); }, // TTY1  lol
            "\u{1b}[1;7Q"     => { return Event::CtrlAlt     (Eventee::F2              ); }, // TTY2  lol
            "\u{1b}[1;7R"     => { return Event::CtrlAlt     (Eventee::F3              ); }, // TTY3  lol
            "\u{1b}[1;7S"     => { return Event::CtrlAlt     (Eventee::F4              ); }, // TTY4  lol
            "\u{1b}[15;7~"    => { return Event::CtrlAlt     (Eventee::F5              ); }, // TTY5  lol
            "\u{1b}[17;7~"    => { return Event::CtrlAlt     (Eventee::F6              ); }, // TTY6  lol
            "\u{1b}[18;7~"    => { return Event::CtrlAlt     (Eventee::F7              ); }, // TTY7  lol
            "\u{1b}[19;7~"    => { return Event::CtrlAlt     (Eventee::F8              ); }, // TTY8  lol
            "\u{1b}[20;7~"    => { return Event::CtrlAlt     (Eventee::F9              ); }, // TTY9  lol
            "\u{1b}[21;7~"    => { return Event::CtrlAlt     (Eventee::F10             ); }, // TTY10 lol
            "\u{1b}[23;7~"    => { return Event::CtrlAlt     (Eventee::F11             ); }, // TTY11 lol
            "\u{1b}[24;7~"    => { return Event::CtrlAlt     (Eventee::F12             ); }, // TTY12 lol
            "\u{1b}[27;7;9~"  => { return Event::CtrlAlt     (Eventee::Tab             ); },
            "\u{1b}[2;7~"     => { return Event::CtrlAlt     (Eventee::Insert          ); },
            "\u{1b}[3;7~"     => { return Event::CtrlAlt     (Eventee::Delete          ); },
            "\u{1b}[1;7H"     => { return Event::CtrlAlt     (Eventee::Home            ); },
            "\u{1b}[1;7F"     => { return Event::CtrlAlt     (Eventee::End             ); },
            "\u{1b}[5;7~"     => { return Event::CtrlAlt     (Eventee::PageUp          ); },
            "\u{1b}[6;7~"     => { return Event::CtrlAlt     (Eventee::PageDown        ); },
            "\u{1b}[1;7A"     => { return Event::CtrlAlt     (Eventee::ArrowUp         ); },
            "\u{1b}[1;7D"     => { return Event::CtrlAlt     (Eventee::ArrowLeft       ); },
            "\u{1b}[1;7B"     => { return Event::CtrlAlt     (Eventee::ArrowDown       ); },
            "\u{1b}[1;7C"     => { return Event::CtrlAlt     (Eventee::ArrowRight      ); },

            "\u{1b}[27;6;13~" => { return Event::CtrlShift   (Eventee::Enter           ); },
            "\u{1b}[27;6;27~" => { return Event::CtrlShift   (Eventee::Escape          ); },
            "\u{1b}[1;6P"     => { return Event::CtrlShift   (Eventee::F1              ); },
            "\u{1b}[1;6Q"     => { return Event::CtrlShift   (Eventee::F2              ); },
            "\u{1b}[1;6R"     => { return Event::CtrlShift   (Eventee::F3              ); },
            "\u{1b}[1;6S"     => { return Event::CtrlShift   (Eventee::F4              ); },
            "\u{1b}[15;6~"    => { return Event::CtrlShift   (Eventee::F5              ); },
            "\u{1b}[17;6~"    => { return Event::CtrlShift   (Eventee::F6              ); },
            "\u{1b}[18;6~"    => { return Event::CtrlShift   (Eventee::F7              ); },
            "\u{1b}[19;6~"    => { return Event::CtrlShift   (Eventee::F8              ); },
            "\u{1b}[20;6~"    => { return Event::CtrlShift   (Eventee::F9              ); },
            "\u{1b}[21;6~"    => { return Event::CtrlShift   (Eventee::F10             ); },
            "\u{1b}[23;6~"    => { return Event::CtrlShift   (Eventee::F11             ); },
            "\u{1b}[24;6~"    => { return Event::CtrlShift   (Eventee::F12             ); },
            "\u{1b}[27;6;9~"  => { return Event::CtrlShift   (Eventee::Tab             ); },
            "\u{1b}[2;6~"     => { return Event::CtrlShift   (Eventee::Insert          ); },
            "\u{1b}[3;6~"     => { return Event::CtrlShift   (Eventee::Delete          ); },
            "\u{1b}[1;6H"     => { return Event::CtrlShift   (Eventee::Home            ); },
            "\u{1b}[1;6F"     => { return Event::CtrlShift   (Eventee::End             ); },
            "\u{1b}[5;6~"     => { return Event::CtrlShift   (Eventee::PageUp          ); },
            "\u{1b}[6;6~"     => { return Event::CtrlShift   (Eventee::PageDown        ); },
            "\u{1b}[1;6A"     => { return Event::CtrlShift   (Eventee::ArrowUp         ); },
            "\u{1b}[1;6D"     => { return Event::CtrlShift   (Eventee::ArrowLeft       ); },
            "\u{1b}[1;6B"     => { return Event::CtrlShift   (Eventee::ArrowDown       ); },
            "\u{1b}[1;6C"     => { return Event::CtrlShift   (Eventee::ArrowRight      ); },

            "\u{1b}[27;4;13~" => { return Event::AltShift    (Eventee::Enter           ); },
            "\u{1b}[27;4;27~" => { return Event::AltShift    (Eventee::Escape          ); },
            "\u{1b}[1;4P"     => { return Event::AltShift    (Eventee::F1              ); },
            "\u{1b}[1;4Q"     => { return Event::AltShift    (Eventee::F2              ); },
            "\u{1b}[1;4R"     => { return Event::AltShift    (Eventee::F3              ); },
            "\u{1b}[1;4S"     => { return Event::AltShift    (Eventee::F4              ); },
            "\u{1b}[15;4~"    => { return Event::AltShift    (Eventee::F5              ); },
            "\u{1b}[17;4~"    => { return Event::AltShift    (Eventee::F6              ); },
            "\u{1b}[18;4~"    => { return Event::AltShift    (Eventee::F7              ); },
            "\u{1b}[19;4~"    => { return Event::AltShift    (Eventee::F8              ); },
            "\u{1b}[20;4~"    => { return Event::AltShift    (Eventee::F9              ); },
            "\u{1b}[21;4~"    => { return Event::AltShift    (Eventee::F10             ); },
            "\u{1b}[23;4~"    => { return Event::AltShift    (Eventee::F11             ); },
            "\u{1b}[24;4~"    => { return Event::AltShift    (Eventee::F12             ); },
            "\u{1b}[27;4;9~"  => { return Event::AltShift    (Eventee::Tab             ); },
            "\u{1b}[2;4~"     => { return Event::AltShift    (Eventee::Insert          ); },
            "\u{1b}[3;4~"     => { return Event::AltShift    (Eventee::Delete          ); },
            "\u{1b}[1;4H"     => { return Event::AltShift    (Eventee::Home            ); },
            "\u{1b}[1;4F"     => { return Event::AltShift    (Eventee::End             ); },
            "\u{1b}[5;4~"     => { return Event::AltShift    (Eventee::PageUp          ); },
            "\u{1b}[6;4~"     => { return Event::AltShift    (Eventee::PageDown        ); },
            "\u{1b}[1;4A"     => { return Event::AltShift    (Eventee::ArrowUp         ); },
            "\u{1b}[1;4D"     => { return Event::AltShift    (Eventee::ArrowLeft       ); },
            "\u{1b}[1;4B"     => { return Event::AltShift    (Eventee::ArrowDown       ); },
            "\u{1b}[1;4C"     => { return Event::AltShift    (Eventee::ArrowRight      ); },

            "\u{1b}[27;8;13~" => { return Event::CtrlAltShift(Eventee::Enter           ); },
            "\u{1b}[27;8;27~" => { return Event::CtrlAltShift(Eventee::Escape          ); },
            "\u{1b}[1;8P"     => { return Event::CtrlAltShift(Eventee::F1              ); },
            "\u{1b}[1;8Q"     => { return Event::CtrlAltShift(Eventee::F2              ); },
            "\u{1b}[1;8R"     => { return Event::CtrlAltShift(Eventee::F3              ); },
            "\u{1b}[1;8S"     => { return Event::CtrlAltShift(Eventee::F4              ); },
            "\u{1b}[15;8~"    => { return Event::CtrlAltShift(Eventee::F5              ); },
            "\u{1b}[17;8~"    => { return Event::CtrlAltShift(Eventee::F6              ); },
            "\u{1b}[18;8~"    => { return Event::CtrlAltShift(Eventee::F7              ); },
            "\u{1b}[19;8~"    => { return Event::CtrlAltShift(Eventee::F8              ); },
            "\u{1b}[20;8~"    => { return Event::CtrlAltShift(Eventee::F9              ); },
            "\u{1b}[21;8~"    => { return Event::CtrlAltShift(Eventee::F10             ); },
            "\u{1b}[23;8~"    => { return Event::CtrlAltShift(Eventee::F11             ); },
            "\u{1b}[24;8~"    => { return Event::CtrlAltShift(Eventee::F12             ); },
            "\u{1b}[27;8;9~"  => { return Event::CtrlAltShift(Eventee::Tab             ); },
            "\u{1b}[2;8~"     => { return Event::CtrlAltShift(Eventee::Insert          ); },
            "\u{1b}[3;8~"     => { return Event::CtrlAltShift(Eventee::Delete          ); },
            "\u{1b}[1;8H"     => { return Event::CtrlAltShift(Eventee::Home            ); },
            "\u{1b}[1;8F"     => { return Event::CtrlAltShift(Eventee::End             ); },
            "\u{1b}[5;8~"     => { return Event::CtrlAltShift(Eventee::PageUp          ); },
            "\u{1b}[6;8~"     => { return Event::CtrlAltShift(Eventee::PageDown        ); },
            "\u{1b}[1;8A"     => { return Event::CtrlAltShift(Eventee::ArrowUp         ); },
            "\u{1b}[1;8D"     => { return Event::CtrlAltShift(Eventee::ArrowLeft       ); },
            "\u{1b}[1;8B"     => { return Event::CtrlAltShift(Eventee::ArrowDown       ); },
            "\u{1b}[1;8C"     => { return Event::CtrlAltShift(Eventee::ArrowRight      ); },

            _                 => ()
        }

        if string.chars().count() == 1 {
            Event::Char(
                string.chars().next().unwrap()
            )
        } else {
            Event::Unimplemented(string)
        }
    }
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        let _ = self.write_all(
            format!(
                "{}{}{}{}{}",
                "\x1b[?2004l", // disable bracketed paste
                "\x1b[?1004l", // disable focus reporting
                "\x1b[?1006l", // reduce mouse support
                "\x1b[?1003l", // disable mouse events
                "\x1b[?1049l"  // exit alternate screen
            ).as_bytes()
        );

        let _ = self.flush();

        unsafe { libc::tcsetattr(self.input.as_raw_fd(), libc::TCSAFLUSH, &self.old_termios); }
    }
}

#[expect(missing_docs)]
#[derive(Debug, Clone)]
pub enum Event {
    /**/ NoModifiers(Eventee),
    /**/Ctrl        (Eventee),
    /**/    Alt     (Eventee),
    /**/       Shift(Eventee),
    /**/CtrlAlt     (Eventee),
    /**/CtrlShift   (Eventee),
    /**/    AltShift(Eventee),
    /**/CtrlAltShift(Eventee),

    FocusGained,
    FocusLost,

    /// characters are not in [`Eventee`], because:
    /// - a lot of reasons and im lazy at docs (will surely write a whole book about this one day)
    /*    */Char(        char),
    /**/CtrlChar(CtrlableChar),
    /* */AltChar( AltableChar),

    /// `Backspace` is not in [`Eventee`], since these do not work in terminals:  
    /// - `Shift + Backspace`  
    /// - `Ctrl + Shift + Backspace`  
    /// - `Alt + Shift + Backspace`  
    /// - `Ctrl + Alt + Shift + Backspace`
    /*       */Backspace,
    /*   */CtrlBackspace,
    /*    */AltBackspace,
    /**/CtrlAltBackspace,

    /// returns what was read from input when betterm cannot make sense of it,  
    /// if its relevant, you can submit an issue/pr
    Unimplemented(String)
}

/// includes the whole alphabet, except: H, I, J, M  
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

/// includes the whole alphabet
#[expect(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum AltableChar {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
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
#[derive(Debug, Clone, Copy)]
pub enum Eventee {
    Enter, // NOTE: deliberately not `Char('\n')`
    Tab,   // NOTE: deliberately not `Char('\t')`
    Escape,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Insert, Delete,
    Home, End,
    PageUp, PageDown,
    ArrowUp, ArrowDown, ArrowRight, ArrowLeft
}

