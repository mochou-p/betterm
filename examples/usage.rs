fn main() {
    use betterm::*;

    println!(
        "{}{}{}{}{}betterm{} does {}NOT{} guarantee that all of this will work in your terminal,",
        clear::WHOLE_SCREEN,      clear::SCROLLBACK_BUFFER,
        cursor::MOVE_TO_TOP_LEFT, style::INTENSITY_FAINT,
        color::FG_GREEN,          color::UNSET_FG,
        color::FG_RED,            color::UNSET_FG
    );
    println!(
        "it only provides convenient string constants, and structs implementing {}Display{}",
        color::FG_YELLOW, RESET_ALL
    );

    println!("\nANSI colors");
    {
        print!  ("  foreground:                     "  );
        print!  ("{}t", color::FG_BLACK                );
        print!  ("{}e", color::FG_RED                  );
        print!  ("{}r", color::FG_GREEN                );
        print!  ("{}m", color::FG_YELLOW               );
        print!  ("{}i", color::FG_BLUE                 );
        print!  ("{}n", color::FG_MAGENTA              );
        print!  ("{}a", color::FG_CYAN                 );
        print!  ("{}l", color::FG_WHITE                );
        println!("{}",  color::UNSET_FG                );

        print!  ("  bright foreground:              "  );
        print!  ("{}t", color::FG_BRIGHT_BLACK         );
        print!  ("{}e", color::FG_BRIGHT_RED           );
        print!  ("{}r", color::FG_BRIGHT_GREEN         );
        print!  ("{}m", color::FG_BRIGHT_YELLOW        );
        print!  ("{}i", color::FG_BRIGHT_BLUE          );
        print!  ("{}n", color::FG_BRIGHT_MAGENTA       );
        print!  ("{}a", color::FG_BRIGHT_CYAN          );
        print!  ("{}l", color::FG_BRIGHT_WHITE         );
        println!("{}",  color::UNSET_FG                );

        print!  ("  background:                     "  );
        print!  ("{} ", color::BG_BLACK                );
        print!  ("{} ", color::BG_RED                  );
        print!  ("{} ", color::BG_GREEN                );
        print!  ("{} ", color::BG_YELLOW               );
        print!  ("{} ", color::BG_BLUE                 );
        print!  ("{} ", color::BG_MAGENTA              );
        print!  ("{} ", color::BG_CYAN                 );
        print!  ("{} ", color::BG_WHITE                );
        println!("{}",  color::UNSET_BG                );

        print!  ("  bright background:              "  );
        print!  ("{} ", color::BG_BRIGHT_BLACK         );
        print!  ("{} ", color::BG_BRIGHT_RED           );
        print!  ("{} ", color::BG_BRIGHT_GREEN         );
        print!  ("{} ", color::BG_BRIGHT_YELLOW        );
        print!  ("{} ", color::BG_BRIGHT_BLUE          );
        print!  ("{} ", color::BG_BRIGHT_MAGENTA       );
        print!  ("{} ", color::BG_BRIGHT_CYAN          );
        print!  ("{} ", color::BG_BRIGHT_WHITE         );
        println!("{}",  color::UNSET_BG                );

        print!  ("  underline:                      "  );
        print!  ("{}",  style::UNDERLINE               );
        print!  ("{}t", color::UNDERLINE_BLACK         );
        print!  ("{}e", color::UNDERLINE_RED           );
        print!  ("{}r", color::UNDERLINE_GREEN         );
        print!  ("{}m", color::UNDERLINE_YELLOW        );
        print!  ("{}i", color::UNDERLINE_BLUE          );
        print!  ("{}n", color::UNDERLINE_MAGENTA       );
        print!  ("{}a", color::UNDERLINE_CYAN          );
        print!  ("{}l", color::UNDERLINE_WHITE         );
        print!  ("{}",  color::UNSET_UNDERLINE         );
        println!("{}",  style::UNSET_UNDERLINE         );

        print!  ("  bright underline:               "  );
        print!  ("{}",  style::UNDERLINE               );
        print!  ("{}t", color::UNDERLINE_BRIGHT_BLACK  );
        print!  ("{}e", color::UNDERLINE_BRIGHT_RED    );
        print!  ("{}r", color::UNDERLINE_BRIGHT_GREEN  );
        print!  ("{}m", color::UNDERLINE_BRIGHT_YELLOW );
        print!  ("{}i", color::UNDERLINE_BRIGHT_BLUE   );
        print!  ("{}n", color::UNDERLINE_BRIGHT_MAGENTA);
        print!  ("{}a", color::UNDERLINE_BRIGHT_CYAN   );
        print!  ("{}l", color::UNDERLINE_BRIGHT_WHITE  );
        print!  ("{}",  color::UNSET_UNDERLINE         );
        println!("{}",  style::UNSET_UNDERLINE         );
    }

    println!(
        "\nweb-safe colors {}(only bg showed, but there are also fg and underline){}",
        style::INTENSITY_FAINT,
        style::UNSET_INTENSITY
    );
    {
        print!("  standard colors (0..=7):        ");
        for i in 0..8 {
            print!("{} ", color::BG_WEB_SAFE_COLORS[i]);
        }
        print!("{}\n", color::UNSET_BG);

        print!("  high-intensity colors (8..=15): ");
        for i in 8..16 {
            print!("{} ", color::BG_WEB_SAFE_COLORS[i]);
        }
        print!("{}\n", color::UNSET_BG);

        print!("  \"216 colors\" (16..=231):        ");
        let column_len = 36;
        let note_lines = [
            "while web-safe colors are a LUT",
            "note that some of these may be",
            "overwritten by the color theme",
            "configuration of your terminal",
            "(most commonly the first few)"
        ];
        for row in 0..6 {
            if row != 0 {
                let note_line = note_lines[row - 1];
                print!(
                    "{}\n  {}{note_line}{}{} ",
                    color::UNSET_BG,
                    style::INTENSITY_FAINT,
                    style::UNSET_INTENSITY,
                    " ".repeat(31 - note_line.len())
                );
            }
            for column in 0..column_len {
                let i = 16 + (row * column_len) + column;
                print!("{} ", color::BG_WEB_SAFE_COLORS[i]);
            }
        }
        print!("{}\n", color::UNSET_BG);

        print!("  greyscale colors (232..=255):   ");
        for i in 232..256 {
            print!("{} ", color::BG_WEB_SAFE_COLORS[i]);
        }
        print!("{}\n", color::UNSET_BG);
    }

    println!("\nRGB colors");
    {
        print!  ("  custom foreground:              "     );
        print!  ("{}t", color::FgRgb(  0,   0,   0)       );
        print!  ("{}e", color::FgRgb(255,   0,   0)       );
        print!  ("{}r", color::FgRgb(255, 255,   0)       );
        print!  ("{}m", color::FgRgb(  0, 255,   0)       );
        print!  ("{}i", color::FgRgb(  0, 255, 255)       );
        print!  ("{}n", color::FgRgb(  0,   0, 255)       );
        print!  ("{}a", color::FgRgb(255,   0, 255)       );
        print!  ("{}l", color::FgRgb(255, 255, 255)       );
        println!("{}",  color::UNSET_FG                   );

        print!  ("  custom background:              "     );
        print!  ("{} ", color::BgRgb(  0,   0,   0)       );
        print!  ("{} ", color::BgRgb(255,   0,   0)       );
        print!  ("{} ", color::BgRgb(255, 255,   0)       );
        print!  ("{} ", color::BgRgb(  0, 255,   0)       );
        print!  ("{} ", color::BgRgb(  0, 255, 255)       );
        print!  ("{} ", color::BgRgb(  0,   0, 255)       );
        print!  ("{} ", color::BgRgb(255,   0, 255)       );
        print!  ("{} ", color::BgRgb(255, 255, 255)       );
        println!("{}",  color::UNSET_BG                   );

        print!  ("  custom underline:               "     );
        print!  ("{}",  style::UNDERLINE                  );
        print!  ("{}t", color::UnderlineRgb(  0,   0,   0));
        print!  ("{}e", color::UnderlineRgb(255,   0,   0));
        print!  ("{}r", color::UnderlineRgb(255, 255,   0));
        print!  ("{}m", color::UnderlineRgb(  0, 255,   0));
        print!  ("{}i", color::UnderlineRgb(  0, 255, 255));
        print!  ("{}n", color::UnderlineRgb(  0,   0, 255));
        print!  ("{}a", color::UnderlineRgb(255,   0, 255));
        print!  ("{}l", color::UnderlineRgb(255, 255, 255));
        print!  ("{}",  color::UNSET_UNDERLINE            );
        println!("{}",  style::UNSET_UNDERLINE            );
    }

    println!("\ntext styling");
    {
        let text = "terminal";

        print!  ("  normal:                         ");
        println!("{text}"                            );

        print!  ("  bold:                           "                                      );
        println!("{}{text}{}", style::INTENSITY_BOLD,            style::UNSET_INTENSITY    );

        print!  ("  faint:                          "                                      );
        println!("{}{text}{}", style::INTENSITY_FAINT,           style::UNSET_INTENSITY    );

        print!  ("  italic:                         "                                      );
        println!("{}{text}{}", style::ITALIC,                    style::UNSET_ITALIC       );

        print!  ("  overlined:                      "                                      );
        println!("{}{text}{}", style::OVERLINE,                  style::UNSET_OVERLINE     );

        print!  ("  striked:                        "                                      );
        println!("{}{text}{}", style::STRIKETHROUGH,             style::UNSET_STRIKETHROUGH);

        print!  ("  underline:                      "                                      );
        println!("{}{text}{}", style::UNDERLINE,                 style::UNSET_UNDERLINE    );

        print!  ("  double underlinee:              "                                      );
        println!("{}{text}{}", style::UNDERLINE_DOUBLE,          style::UNSET_UNDERLINE    );

        print!  ("  slow blink:                     "                                      );
        println!("{}{text}{}", style::BLINK_SLOW,                style::UNSET_BLINK        );

        print!  ("  fast blink:                     "                                      );
        println!("{}{text}{}", style::BLINK_FAST,                style::UNSET_BLINK        );

        print!  ("  invert:                         "                                      );
        println!("{}{text}{}", style::INVERT_COLORS,             style::UNSET_INVERT       );

        print!  ("  conceal:                        "                                      );
        println!("{}{text}{}", style::CONCEAL,                   style::UNSET_CONCEAL      );

        print!  ("  alt font 1:                     "                                      );
        println!("{}{text}{}", style::FONT_ALTERNATIVE_1,        style::UNSET_FONT         );
        print!  ("  alt font 2:                     "                                      );
        println!("{}{text}{}", style::FONT_ALTERNATIVE_2,        style::UNSET_FONT         );
        print!  ("  alt font 3:                     "                                      );
        println!("{}{text}{}", style::FONT_ALTERNATIVE_3,        style::UNSET_FONT         );
        print!  ("  alt font 4:                     "                                      );
        println!("{}{text}{}", style::FONT_ALTERNATIVE_4,        style::UNSET_FONT         );
        print!  ("  alt font 5:                     "                                      );
        println!("{}{text}{}", style::FONT_ALTERNATIVE_5,        style::UNSET_FONT         );
        print!  ("  alt font 6:                     "                                      );
        println!("{}{text}{}", style::FONT_ALTERNATIVE_6,        style::UNSET_FONT         );
        print!  ("  alt font 7:                     "                                      );
        println!("{}{text}{}", style::FONT_ALTERNATIVE_7,        style::UNSET_FONT         );
        print!  ("  alt font 8:                     "                                      );
        println!("{}{text}{}", style::FONT_ALTERNATIVE_8,        style::UNSET_FONT         );
        print!  ("  alt font 9:                     "                                      );
        println!("{}{text}{}", style::FONT_ALTERNATIVE_9,        style::UNSET_FONT         );

        print!  ("  gothic font:                    "                                      );
        println!("{}{text}{}", style::FONT_GOTHIC,               style::UNSET_FONT         );

        print!  ("  proportional spacing:           "                                      );
        println!("{}{text}{}", style::SPACING_PROPORTIONAL,      style::UNSET_SPACING      );

        print!  ("  superscript:                    "                                      );
        println!("{}{text}{}", style::SCRIPT_SUPER,              style::UNSET_SCRIPT       );
        print!  ("  subscript:                      "                                      );
        println!("{}{text}{}", style::SCRIPT_SUB,                style::UNSET_SCRIPT       );

        print!  ("  ideogram underline:             "                                      );
        println!("{}{text}{}", style::IDEOGRAM_UNDERLINE,        style::UNSET_IDEOGRAM     );
        print!  ("  ideogram double underline:      "                                      );
        println!("{}{text}{}", style::IDEOGRAM_DOUBLE_UNDERLINE, style::UNSET_IDEOGRAM     );
        print!  ("  ideogram overline:              "                                      );
        println!("{}{text}{}", style::IDEOGRAM_OVERLINE,         style::UNSET_IDEOGRAM     );
        print!  ("  ideogram double overline:       "                                      );
        println!("{}{text}{}", style::IDEOGRAM_DOUBLE_OVERLINE,  style::UNSET_IDEOGRAM     );
        print!  ("  ideogram stress marking:        "                                      );
        println!("{}{text}{}", style::IDEOGRAM_STRESS_MARKING,   style::UNSET_IDEOGRAM     );

        print!  ("  some combination:               ");
        println!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{text}{}",
            color::BG_BLUE,
            color::FG_YELLOW,
            style::INTENSITY_BOLD,
            style::ITALIC,
            style::UNDERLINE,
            color::UnderlineRgb(255, 0, 0),
            style::BLINK_SLOW,
            style::STRIKETHROUGH,
            style::FONT_GOTHIC,
            style::SCRIPT_SUB,
            style::IDEOGRAM_DOUBLE_UNDERLINE,
            style::IDEOGRAM_DOUBLE_OVERLINE,
            style::IDEOGRAM_STRESS_MARKING,
            RESET_ALL
        );
    }
}

