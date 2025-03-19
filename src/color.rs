use crate::vec3::Color;
use std::io::Write;

pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) -> std::io::Result<()> {
    let scale = 1.0 / samples_per_pixel as f64;
    
    // Apply gamma correction (gamma 2)
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();
    
    // Clamp values to [0, 1] range and convert to [0, 255]
    let ir = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as i32;
    
    writeln!(out, "{} {} {}", ir, ig, ib)
}

pub fn color_to_rgb(pixel_color: Color, samples_per_pixel: i32) -> [u8; 3] {
    let scale = 1.0 / samples_per_pixel as f64;
    
    // Apply gamma correction (gamma 2)
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();
    
    // Clamp values to [0, 1] range and convert to [0, 255]
    let ir = (256.0 * r.clamp(0.0, 0.999)) as u8;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as u8;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as u8;
    
    [ir, ig, ib]
} 