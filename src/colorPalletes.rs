pub trait Palette {
    fn background() -> &'static str;
    fn primary() -> &'static str;
    fn secondary() -> &'static str;
    fn tertiary() -> &'static str;
    // Add as many as you need
}

pub struct Flat;
pub struct Nord;

impl Palette for Flat {
    fn background() -> &'static str { "#2c3e50" } 
    fn primary() -> &'static str { "#c0392b" } 
    fn secondary() -> &'static str { "#2980b9" } 
    fn tertiary() -> &'static str { "#27ae60" }
}

impl Pallete for Nord{
    fn background() -> &'static str { "#2e3440" } 
    fn primary() -> &'static str { "#88c0d0" } 
    fn secondary() -> &'static str { "#81a1c1" } 
    fn tertiary() -> &'static str { "#5e81ac" }
}