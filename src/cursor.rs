// mochou-p/betterm/src/cursor.rs

/// absolute cursor position, order: column, row (top left corner is 1,1)
pub fn goto(x: u16, y: u16) -> String {
    format!("\x1b[{y};{x}H")
}

