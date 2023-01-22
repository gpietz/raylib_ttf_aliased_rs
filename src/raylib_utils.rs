use raylib::prelude::*;
use regex::Regex;
use std::convert::Into;

/// Converts a string in hex or rgb format to color.
pub(crate) fn string_to_color(s: &str) -> Option<Color> {
    let hex_re = Regex::new(r"^#([0-9a-fA-F]{6})$").unwrap();
    let rgb_re = Regex::new(r"^([0-9]{1,3})-([0-9]{1,3})-([0-9]{1,3})$").unwrap();
    if hex_re.is_match(s) {
        let r = u8::from_str_radix(&s[1..3], 16).unwrap();
        let g = u8::from_str_radix(&s[3..5], 16).unwrap();
        let b = u8::from_str_radix(&s[5..7], 16).unwrap();
        Some(Color::new(r, g, b, 255))
    } else if rgb_re.is_match(s) {
        let caps = rgb_re.captures(s).unwrap();
        let r = caps[1].parse::<u8>().unwrap();
        let g = caps[2].parse::<u8>().unwrap();
        let b = caps[3].parse::<u8>().unwrap();
        Some(Color::new(r, g, b, 255))
    } else {
        None
    }
}
