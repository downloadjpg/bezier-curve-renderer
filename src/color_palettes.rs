pub struct Palette {
    pub background: Color,
    pub background_accent: Color,
    pub white: Color,
    pub curve_colors: Vec<Color>,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,

}
pub struct Color {
    pub value: &'static str,
}
impl Palette {
    pub fn nord() -> Palette { Palette {
        background: Color { value: "#3b4252" },
        background_accent: Color { value: "#4c566a" },
        white: Color { value: "#e5e9f0" },
        primary: Color { value: "#5e81ac" },
        secondary: Color { value: "#81a1c1" },
        accent: Color { value: "#d08770" },
        curve_colors: vec![
            Color { value: "#bf616a" },
            Color { value: "#a3be8c" },
            Color { value: "#ebcb8b" },
            Color { value: "#88c0d0" },
            ],
        }
    }
    
    pub fn default() -> Palette {
        Self::nord()
    }
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