use crate::rt::{color::Color, pdfs::Pdf, ray::Ray};

pub enum ScatterRecord {
    Specular {
        attenuation: Color,
        ray: Ray,
    },
    Diffuse {
        attenuation: Color,
        pdf: Box<dyn Pdf>,
    },
}
