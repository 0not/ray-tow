use crate::Color;

#[derive(Debug)]
pub struct RawImageBuffer {
    pub width: u32,
    pub height: u32,
    pub buf: Vec<u8>,
}

impl RawImageBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buf: Vec::with_capacity((width * height * 3) as usize),
        }
    }

    pub fn push_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.buf.push(r);
        self.buf.push(g);
        self.buf.push(b);
    }

    pub fn push_color(&mut self, pixel: Color) {
        // Linear to gamma corrected
        let pixel = pixel.clamp(Color::ZERO, Color::ONE).map(|p| p.sqrt());

        let pixel = ((256. - f64::EPSILON) * pixel).as_u8vec3();

        self.push_rgb(pixel.x, pixel.y, pixel.z);
    }

    pub fn save<T>(&self, path: T) -> image::ImageResult<()>
    where
        T: AsRef<std::path::Path>,
    {
        image::save_buffer(
            path,
            &self.buf,
            self.width,
            self.height,
            image::ExtendedColorType::Rgb8,
        )
    }
}
