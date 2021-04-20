pub struct ColorCode {
    pub start: u8,
    pub end: u8
}

pub enum Colors {
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Inverse,
    Hidden,
    Strike,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    Grey,
    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite
}

impl Colors {
    pub fn from(string: &str, colors: Colors) -> String {
        let code = Colors::from_color_code(colors);

        format!("\x1b[{}m{}\x1b[{}m", code.start, string, code.end)
    }

    pub fn from_color_code(colors: Colors) -> ColorCode {
        match colors {
            Colors::Reset         => ColorCode { start: 0, end: 0 },
            Colors::Bold          => ColorCode { start: 1, end: 22 },
            Colors::Dim           => ColorCode { start: 2, end: 22 },
            Colors::Italic        => ColorCode { start: 3, end: 23 },
            Colors::Underline     => ColorCode { start: 4, end: 24 },
            Colors::Inverse       => ColorCode { start: 7, end: 27 },
            Colors::Hidden        => ColorCode { start: 8, end: 28 },
            Colors::Strike        => ColorCode { start: 9, end: 29 },
            Colors::Black         => ColorCode { start: 30, end: 39 },
            Colors::Red           => ColorCode { start: 31, end: 39 },
            Colors::Green         => ColorCode { start: 32, end: 39 },
            Colors::Yellow        => ColorCode { start: 33, end: 39 },
            Colors::Blue          => ColorCode { start: 34, end: 39 },
            Colors::Magenta       => ColorCode { start: 35, end: 39 },
            Colors::Cyan          => ColorCode { start: 36, end: 39 },
            Colors::White         => ColorCode { start: 37, end: 39 },
            Colors::Gray          => ColorCode { start: 90, end: 39 },
            Colors::Grey          => ColorCode { start: 90, end: 39 },
            Colors::BgBlack       => ColorCode { start: 40, end: 49 },
            Colors::BgRed         => ColorCode { start: 41, end: 49 },
            Colors::BgGreen       => ColorCode { start: 42, end: 49 },
            Colors::BgYellow      => ColorCode { start: 43, end: 49 },
            Colors::BgBlue        => ColorCode { start: 44, end: 49 },
            Colors::BgMagenta     => ColorCode { start: 45, end: 49 },
            Colors::BgCyan        => ColorCode { start: 46, end: 49 },
            Colors::BgWhite       => ColorCode { start: 47, end: 49 }
        }
    }
}