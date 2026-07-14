// mochou-p/betterm/examples/colors.rs

use std::io::Write as _;
use betterm::*;


fn main() {
    // of course you can mix `with` and `push/pop`,
    // based on your use case of each color

    // --------------------------------------------

    // level 3: with
    styled_printer::StyledPrinter::default()
        .with_fg(color::ansi::cyan(), |cyan| {
            cyan.text("hope you ")
                .fg(color::ansi::green(),  "enjoy")
                .text(" this ")
                .fg(color::ansi::yellow(), "crate")
                .text("!!")
        })
        .text("\n")
        .write_all(&mut std::io::stdout())
        .unwrap();

    // NOTE: dont forget this!
    std::io::stdout().flush().unwrap();

    // --------------------------------------------

    // level 2: push/pop
    styled_printer::StyledPrinter::default()
        .push_fg(color::ansi::cyan())
            .text("hope you ")
            .push_fg(color::ansi::green())
                .text("enjoy")
            .pop_fg()
            .text(" this ")
            .push_fg(color::ansi::yellow())
                .text("crate")
            .pop_fg()
            .text("!!")
        .pop_fg()
        .text("\n")
        .write_all(&mut std::io::stdout())
        .unwrap();

    // NOTE: dont forget this!
    std::io::stdout().flush().unwrap();

    // --------------------------------------------

    // level 1: manual
    println!(
        "{}hope you {}enjoy{} this {}crate{}!!{}",
        color::fg(color::ansi::  cyan()),
        color::fg(color::ansi:: green()),
        color::fg(color::ansi::  cyan()),
        color::fg(color::ansi::yellow()),
        color::fg(color::ansi::  cyan()),
        reset::fg()
    );
}

