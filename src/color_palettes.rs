pub struct Palette {
    pub background: &'static str,
    pub primary: &'static str,
    pub secondary: &'static str,
    pub tertiary: &'static str,
}

pub const FLAT: Palette = Palette {
    background: "#2c3e50",
    primary: "#c0392b",
    secondary: "#2980b9",
    tertiary: "#27ae60",
};

pub const NORD: Palette = Palette {
    background: "#2e3440",
    primary: "#88c0d0",
    secondary: "#81a1c1",
    tertiary: "#5e81ac",
};

impl Palette {
    pub fn to_rgba(&self, color: &'static str) -> [f32; 4] {
        let hex = color.trim_start_matches("#");
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0]
    }
}