// mochou-p/betterm/examples/events.rs

use betterm::*;


fn main() {
    let mut terminal = terminal::RawTerminal::new(
        std::io::stdin (),
        std::io::stdout()
    );

    loop {
        let event = terminal.blocking_event();

        if matches!(
            event,
            terminal::Event::Keyboard(
                terminal::KeyboardEvent::NoModifiers(
                    terminal::Key::Escape
                )
            )
        ) {
            break;
        }

        println!("{event:?}\r");
    }
}

