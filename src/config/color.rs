use midi_controller::config::Color;

pub fn parse_color(s: &str) -> Color {
    match s {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        "cyan" => Color::Cyan,
        "magenta" => Color::Magenta,
        "white" => Color::White,
        "orange" => Color::Orange,
        "purple" => Color::Purple,
        "off" => Color::Off,
        hex if hex.starts_with('#') && hex.len() == 7 => {
            let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
            Color::Custom(r, g, b)
        }
        _ => Color::Off,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_named_colors() {
        assert_eq!(parse_color("red"), Color::Red);
        assert_eq!(parse_color("green"), Color::Green);
        assert_eq!(parse_color("blue"), Color::Blue);
        assert_eq!(parse_color("yellow"), Color::Yellow);
        assert_eq!(parse_color("cyan"), Color::Cyan);
        assert_eq!(parse_color("magenta"), Color::Magenta);
        assert_eq!(parse_color("white"), Color::White);
        assert_eq!(parse_color("orange"), Color::Orange);
        assert_eq!(parse_color("purple"), Color::Purple);
        assert_eq!(parse_color("off"), Color::Off);
    }

    #[test]
    fn parse_hex_colors() {
        assert_eq!(parse_color("#FF0000"), Color::Custom(255, 0, 0));
        assert_eq!(parse_color("#00FF00"), Color::Custom(0, 255, 0));
        assert_eq!(parse_color("#0000FF"), Color::Custom(0, 0, 255));
        assert_eq!(parse_color("#ABCDEF"), Color::Custom(0xAB, 0xCD, 0xEF));
        assert_eq!(parse_color("#000000"), Color::Custom(0, 0, 0));
        assert_eq!(parse_color("#FFFFFF"), Color::Custom(255, 255, 255));
    }

    #[test]
    fn parse_hex_lowercase() {
        assert_eq!(parse_color("#ff8800"), Color::Custom(255, 136, 0));
    }

    #[test]
    fn parse_unknown_returns_off() {
        assert_eq!(parse_color("unknown"), Color::Off);
        assert_eq!(parse_color(""), Color::Off);
        assert_eq!(parse_color("RED"), Color::Off); // case-sensitive
    }

    #[test]
    fn parse_invalid_hex_returns_zeros() {
        // Too short — doesn't match the hex pattern
        assert_eq!(parse_color("#FFF"), Color::Off);
        // Invalid hex digits — from_str_radix returns 0
        assert_eq!(parse_color("#ZZZZZZ"), Color::Custom(0, 0, 0));
    }
}
