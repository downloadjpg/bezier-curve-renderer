pub struct Palette {
    pub background: Color,
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,
    pub accent: Color,
    pub background_accent: Color,

}

pub const FLAT: Palette = Palette {
    background: Color { value: "#2c3e50" },
    primary: Color { value: "#c0392b" },
    secondary: Color { value: "#2980b9" },
    tertiary: Color { value: "#27ae60" },
    accent: Color { value: "#8e44ad" },
    background_accent: Color { value: "#3d4f60" },
};

pub const NORD: Palette = Palette {
    background: Color { value: "#2e3440" },
    primary: Color { value: "#88c0d0" },
    secondary: Color { value: "#81a1c1" },
    tertiary: Color { value: "#5e81ac" },
    accent: Color { value: "#bf616a" },
    background_accent: Color { value: "#4c566a" },
};
pub struct Color {
    pub value: &'static str,
}

impl Color {
    pub fn to_rgba(&self) -> [f32; 4] {
        let hex = self.value.trim_start_matches("#");
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0]
    }
}