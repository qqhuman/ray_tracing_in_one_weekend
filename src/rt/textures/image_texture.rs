use super::Texture;
use crate::rt::{color::Color, Point3};
use image::{io::Reader as ImageReader, DynamicImage};

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_pixel: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    pub fn new(data: Vec<u8>, width: usize, height: usize, bytes_per_pixel: usize) -> ImageTexture {
        ImageTexture {
            data,
            width,
            height,
            bytes_per_pixel,
            bytes_per_scanline: width * bytes_per_pixel,
        }
    }

    pub fn from_file(file_name: &str) -> ImageTexture {
        let img = ImageReader::open(file_name)
            .expect("couldn't open image file")
            .decode()
            .expect("couldn't decode image file");

        let bytes_per_pixel = match &img {
            DynamicImage::ImageRgb8(_) => 3,
            DynamicImage::ImageRgba8(_) => 4,
            _ => panic!("unsupported texture file"),
        };

        ImageTexture::new(
            Vec::from(img.as_bytes()),
            img.width() as usize,
            img.height() as usize,
            bytes_per_pixel as usize,
        )
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

        let i = u * self.width as f64;
        let j = v * self.height as f64;

        let i = (i as usize).min(self.width - 1);
        let j = (j as usize).min(self.height - 1);

        let color_scale = 1.0 / 255.0;
        let index = j * self.bytes_per_scanline + i * self.bytes_per_pixel;

        return Color::new(
            color_scale * self.data[index] as f64,
            color_scale * self.data[index + 1] as f64,
            color_scale * self.data[index + 2] as f64,
        );
    }
}
