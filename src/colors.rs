pub enum AnsiEscape {
    Reset,
    Bold,
    Strike,
    Underline,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Black,
}

pub fn get_ansi_escape(escape: AnsiEscape) -> &'static str {
    match escape {
        AnsiEscape::Reset => "\x1b[0m",
        AnsiEscape::Bold => "\x1b[1m",
        AnsiEscape::Strike => "\x1b[9m",
        AnsiEscape::Underline => "\x1b[4m",
        AnsiEscape::Red => "\x1b[31m",
        AnsiEscape::Green => "\x1b[32m",
        AnsiEscape::Yellow => "\x1b[33m",
        AnsiEscape::Blue => "\x1b[34m",
        AnsiEscape::Magenta => "\x1b[35m",
        AnsiEscape::Cyan => "\x1b[36m",
        AnsiEscape::White => "\x1b[37m",
        AnsiEscape::Black => "\x1b[30m",
    }
}

pub fn colorize(text: &str, color: AnsiEscape) -> String {
    format!(
        "{}{}{}",
        get_ansi_escape(color),
        text,
        get_ansi_escape(AnsiEscape::Reset)
    )
}
