use crate::tuple::Color;
use color_eyre::eyre::eyre;
use color_eyre::Result;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = vec![Color::new(0., 0., 0.); (width * height) as usize];
        Self {
            width,
            height,
            pixels,
        }
    }

    fn index_at(&self, x: u32, y: u32) -> Result<usize> {
        if x >= self.width || y >= self.height {
            return Err(eyre!("Index out of bounds: ({}, {})", x, y));
        }

        Ok((y * self.width + x) as usize)
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) -> Result<()> {
        let index = self.index_at(x, y)?;
        self.pixels[index] = color;
        Ok(())
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Result<Color> {
        let index = self.index_at(x, y)?;
        Ok(self.pixels[index])
    }

    pub fn convert_to_ppm(&self) -> String {
        let mut ppm = String::new();
        ppm.push_str("P3\n");
        ppm.push_str(format!("{} {}\n", self.width, self.height).as_str());
        ppm.push_str("255\n");

        let mut char_count = 0;
        for pixel in &self.pixels {
            let r = (pixel.r * 255.) as u8;
            let g = (pixel.g * 255.) as u8;
            let b = (pixel.b * 255.) as u8;

            ppm.push_str(format!("{r} {g} {b} ").as_str());
            if char_count > 70 - 12 {
                ppm.push('\n');
                char_count = 0;
            } else {
                char_count += 12;
            }
        }
        ppm.push('\n');

        ppm
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;

    #[test]
    pub fn creating_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for pixel in c.pixels {
            assert_eq!(pixel, crate::tuple::Color::new(0., 0., 0.));
        }
    }

    #[test]
    pub fn writing_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20);
        c.write_pixel(2, 3, crate::tuple::Color::new(1., 0., 0.))
            .unwrap();
        assert_eq!(
            c.pixel_at(2, 3).unwrap(),
            crate::tuple::Color::new(1., 0., 0.)
        );
    }
}
