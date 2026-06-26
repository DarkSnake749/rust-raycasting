use macroquad::prelude::Color;

/* 

Color palette: gang gang cockatoo
By: bacondgoat
src: https://www.color-hex.com/color-palette/1080779

#f44336
#999999
#5b5b5b
#2b2929  - the original was: #000000
#cc0000

*/

pub const RED: Color        = Color::from_hex(0xf44336);
pub const LIGHT_GREY: Color = Color::from_hex(0x999999);
pub const DARK_GREY: Color  = Color::from_hex(0x5b5b5b);
pub const BLACK: Color      = Color::from_hex(0x2b2929);
pub const DARK_RED: Color   = Color::from_hex(0xcc0000);

pub fn pseudo_light_interpolation(origin_color: Color, brightness: f32) -> Color {
    let brightness = (brightness, brightness, brightness);
    let new_color = Color::from_rgba(
        ( (origin_color.r as f32) * brightness.0 ).max(255.) as u8, 
        ( (origin_color.g as f32) * brightness.1 ).max(255.) as u8, 
        ( (origin_color.b as f32) * brightness.2 ).max(255.) as u8, 
        255
    );

    new_color
}