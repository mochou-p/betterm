// mochou-p/betterm/examples/events.rs

use betterm::*;


fn main() {
    let mut terminal = terminal::RawTerminal::new(
        std::io::stdin (),
        std::io::stdout()
    );

    loop {
        let event = terminal.blocking_event();

        if matches!(event, terminal::Event::NoModifiers(terminal::Eventee::Escape)) {
            break;
        }

        println!("{event:?}\r");
    }
}

