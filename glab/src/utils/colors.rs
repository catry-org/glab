pub struct ColorCode {
    pub start: u8,
    pub end: u8
}

// https://github.com/Marak/colors.js/blob/7ddd6a3d657efb081abb9beddfec26a01a8790a8/lib/styles.js
pub enum Colors {
    Reset,

    Bold,
    Dim,
    Italic,
    Underline,
    Inverse,
    Hidden,
    Strikethrough,

    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,

    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,

    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite,
    BgGray,


    BgBrightRed,
    BgBrightGreen,
    BgBrightYellow,
    BgBrightBlue,
    BgBrightMagenta,
    BgBrightCyan,
    BgBrightWhite,
  
    // legacy styles for colors pre v1.0.0
    BlackBG,
    RedBG,
    GreenBG,
    YellowBG,
    BlueBG,
    MagentaBG,
    CyanBG,
    WhiteBG,
}

impl Colors {
    pub fn from(string: &str, colors: Colors) -> String {
        let code = Colors::from_color_code(colors);

        format!("\x1b[{}m{}\x1b[{}m", code.start, string, code.end)
    }

    pub fn from_color_code(colors: Colors) -> ColorCode {
        match colors {
            Colors::Reset =>            ColorCode { start: 0,   end: 0  },
            Colors::Bold =>             ColorCode { start: 1,   end: 22 },
            Colors::Dim =>              ColorCode { start: 2,   end: 22 },
            Colors::Italic =>           ColorCode { start: 3,   end: 23 },
            Colors::Underline =>        ColorCode { start: 4,   end: 24 },
            Colors::Inverse =>          ColorCode { start: 7,   end: 27 },
            Colors::Hidden =>           ColorCode { start: 8,   end: 28 },
            Colors::Strikethrough =>    ColorCode { start: 9,   end: 29 },
            Colors::Black =>            ColorCode { start: 30,  end: 39 },
            Colors::Red =>              ColorCode { start: 31,  end: 39 },
            Colors::Green =>            ColorCode { start: 32,  end: 39 },
            Colors::Yellow =>           ColorCode { start: 33,  end: 39 },
            Colors::Blue =>             ColorCode { start: 34,  end: 39 },
            Colors::Magenta =>          ColorCode { start: 35,  end: 39 },
            Colors::Cyan =>             ColorCode { start: 36,  end: 39 },
            Colors::White =>            ColorCode { start: 37,  end: 39 },
            Colors::Gray =>             ColorCode { start: 90,  end: 39 },
            Colors::BrightRed =>        ColorCode { start: 91,  end: 39 },
            Colors::BrightGreen =>      ColorCode { start: 92,  end: 39 },
            Colors::BrightYellow =>     ColorCode { start: 93,  end: 39 },
            Colors::BrightBlue =>       ColorCode { start: 94,  end: 39 },
            Colors::BrightMagenta =>    ColorCode { start: 95,  end: 39 },
            Colors::BrightCyan =>       ColorCode { start: 96,  end: 39 },
            Colors::BrightWhite =>      ColorCode { start: 97,  end: 39 },
            Colors::BgBlack =>          ColorCode { start: 40,  end: 49 },
            Colors::BgRed =>            ColorCode { start: 41,  end: 49 },
            Colors::BgGreen =>          ColorCode { start: 42,  end: 49 },
            Colors::BgYellow =>         ColorCode { start: 43,  end: 49 },
            Colors::BgBlue =>           ColorCode { start: 44,  end: 49 },
            Colors::BgMagenta =>        ColorCode { start: 45,  end: 49 },
            Colors::BgCyan =>           ColorCode { start: 46,  end: 49 },
            Colors::BgWhite =>          ColorCode { start: 47,  end: 49 },
            Colors::BgGray =>           ColorCode { start: 100, end: 49 },
            Colors::BgBrightRed =>      ColorCode { start: 101, end: 49 },
            Colors::BgBrightGreen =>    ColorCode { start: 102, end: 49 },
            Colors::BgBrightYellow =>   ColorCode { start: 103, end: 49 },
            Colors::BgBrightBlue =>     ColorCode { start: 104, end: 49 },
            Colors::BgBrightMagenta =>  ColorCode { start: 105, end: 49 },
            Colors::BgBrightCyan =>     ColorCode { start: 106, end: 49 },
            Colors::BgBrightWhite =>    ColorCode { start: 107, end: 49 },
            Colors::BlackBG =>          ColorCode { start: 40,  end: 49 },
            Colors::RedBG =>            ColorCode { start: 41,  end: 49 },
            Colors::GreenBG =>          ColorCode { start: 42,  end: 49 },
            Colors::YellowBG =>         ColorCode { start: 43,  end: 49 },
            Colors::BlueBG =>           ColorCode { start: 44,  end: 49 },
            Colors::MagentaBG =>        ColorCode { start: 45,  end: 49 },
            Colors::CyanBG =>           ColorCode { start: 46,  end: 49 },
            Colors::WhiteBG =>          ColorCode { start: 47,  end: 49 },
        }
    }
}