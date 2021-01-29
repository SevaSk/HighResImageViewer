extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;


pub struct Color 
{
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color
{
    fn _from_rgb_u32 (c :u32) -> Color
    {
        let c = c.to_be_bytes();

        Color::from((c[1], c[2], c[3]))
    }

    fn to_rgb_u32(self) -> u32
    {
        let (r,g,b) : (u8, u8, u8) = self.into();

        u32::from_be_bytes([0,r,g,b])
    }

    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b}
    }
}

impl From<(u8, u8, u8)> for Color 
{
    /// Convert a `(R, G, B)` tuple of `u8`'s in the range `[0-255]` into a `Color`
    fn from(val: (u8, u8, u8)) -> Self {
        let (r, g, b) = val;
        let rf = (f32::from(r)) / 255.0;
        let gf = (f32::from(g)) / 255.0;
        let bf = (f32::from(b)) / 255.0;
        Color::new(rf, gf, bf)
    }
}

impl From<Color> for (u8, u8, u8) {
    /// Convert a `Color` into a `(R, G, B)` tuple of `u8`'s in the range of `[0-255]`,
    fn from(color: Color) -> Self {
        let r = (color.r *255.0) as u8;
        let g = (color.g *255.0) as u8;
        let b = (color.b *255.0) as u8;
        (r, g, b)
    }
}

fn main() {

    let color = Color {r :0.5f32, g : 0.6f32, b :0.3f32};

    let buffer: Vec<u32> = vec![color.to_rgb_u32(); WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}