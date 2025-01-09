pub fn color_from_rgb (r: u8, g: u8, b: u8, a: u8) -> wgpu::Color {
    wgpu::Color {
        r: r as f64 / 255.0,
        g: g as f64 / 255.0,
        b: b as f64 / 255.0,
        a: a as f64 / 255.0,
    }
}

pub fn color_from_rgba_hex (color: u32) -> wgpu::Color {
    let r = ((color >> 24) & 0xff) as u8;
    let g = ((color >> 16) & 0xff) as u8;
    let b = ((color >> 8) & 0xff) as u8;
    let a = (color & 0xff) as u8;
    color_from_rgb(r, g, b, a)
}