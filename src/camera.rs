use crate::canvas::Canvas;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::tuple::{Color, Point, Vector};
use crate::world::World;
use rand::Rng;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f32,
    pub transform: Matrix4,
    pub pixel_size: f32,
    pub half_width: f32,
    pub half_height: f32,
}

const SAMPLES_PER_PIXEL: usize = 2;

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f32) -> Self {
        let mut c = Self {
            hsize,
            vsize,
            field_of_view: fov,
            transform: Matrix4::identity(),
            pixel_size: 0.,
            half_width: 0.,
            half_height: 0.,
        };

        let half_view = (fov / 2.).tan();
        let aspect = hsize as f32 / vsize as f32;

        if aspect >= 1. {
            c.half_width = half_view;
            c.half_height = half_view / aspect;
        } else {
            c.half_width = half_view * aspect;
            c.half_height = half_view;
        }
        c.pixel_size = (c.half_width * 2.) / hsize as f32;

        c
    }

    pub fn set_transform(&mut self, from: Point, to: Point, up: Vector) {
        self.transform = Matrix4::view_transform(from, to, up);
    }

    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f32 + 0.5 + rand::thread_rng().gen_range(-0.5..0.5)) * self.pixel_size;
        let yoffset = (py as f32 + 0.5 + rand::thread_rng().gen_range(-0.5..0.5)) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let inv = self.transform.inverse();
        let pixel = inv * Point::new(world_x, world_y, -1.);
        let origin = inv * Point::new(0., 0., 0.);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);
        for y in 0..self.vsize - 1 {
            for x in 0..self.hsize - 1 {
                let mut color = Color::new(0., 0., 0.);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let ray = self.ray_for_pixel(x, y);
                    color += world.color_at(&ray);
                }
                canvas
                    .write_pixel(x, y, Self::rescale_color_range(color))
                    .unwrap();
            }
        }

        canvas
    }

    fn rescale_color_range(color: Color) -> Color {
        let scale = 1.0 / SAMPLES_PER_PIXEL as f32;
        let scaled = color * scale;
        Color::new(
            scaled.r.clamp(0., 1.),
            scaled.g.clamp(0., 1.),
            scaled.b.clamp(0., 1.),
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(120, 160, PI / 2.)
    }
}

#[cfg(test)]
mod tests {
    use crate::camera::Camera;
    use crate::matrix::Matrix4;
    use crate::tuple::{Color, Point, Vector};
    use crate::world::World;
    use std::f32::consts::PI;

    #[test]
    pub fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    pub fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    pub fn ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, crate::tuple::Point::new(0., 0., 0.));
        assert_eq!(r.direction, crate::tuple::Vector::new(0., 0., -1.));
    }

    #[test]
    pub fn ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, crate::tuple::Point::new(0., 0., 0.));
        assert_eq!(
            r.direction,
            crate::tuple::Vector::new(0.66519, 0.33259, -0.66851)
        );
    }

    #[test]
    pub fn ray_when_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.);
        c.transform = Matrix4::identity()
            .translate(&Vector::new(0., -2., 5.))
            .rotate_y(PI / 4.);

        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, crate::tuple::Point::new(0., 2., -5.));
        assert_eq!(
            r.direction,
            crate::tuple::Vector::new(2.0_f32.sqrt() / 2., 0., -(2.0_f32.sqrt()) / 2.)
        );
    }

    #[test]
    pub fn render_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.);
        c.set_transform(
            Point::new(0., 0., -5.),
            Point::zero(),
            Vector::new(0., 1., 0.),
        );
        let image = c.render(&w);
        assert_eq!(
            image.pixel_at(5, 5).unwrap(),
            Color::new(0.38066, 0.47582, 0.28549)
        );
    }
}
