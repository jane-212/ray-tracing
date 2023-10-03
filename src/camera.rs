#![allow(unused)]
use std::fs::{File, OpenOptions};
use std::io::Write;

use rand::Rng;

use crate::{Color, HitRecord, Hittable, Interval, Point, Ray, Vec3};

pub struct Camera {
    aspect_ratio: f64,
    width: i32,
    height: i32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    file: File,
    max_depth: i32,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16. / 9.;

        let width = 400;
        let height = ((width as f64 / aspect_ratio) as i32).clamp(1, i32::MAX);

        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (width as f64) / (height as f64);
        let center = Point::new(0., 0., 0.);

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        let pixel_delta_u = viewport_u.clone() / width as f64;
        let pixel_delta_v = viewport_v.clone() / height as f64;

        let viewport_upper_left =
            center.clone() - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc =
            viewport_upper_left + 0.5 * (pixel_delta_u.clone() + pixel_delta_v.clone());

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("image.ppm")
            .expect("open file [image.ppm] failed");

        Self {
            aspect_ratio,
            width,
            height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: 100,
            file,
            max_depth: 50,
        }
    }

    pub fn render(mut self, world: &dyn Hittable) {
        self.file.write_all("P3\n".as_bytes());
        self.file
            .write_all(format!("{} {}\n", self.width, self.height).as_bytes());
        self.file.write_all("255\n".as_bytes());

        for h_pixel in 0..self.height {
            println!("Scanlines remaining: {}", self.height - h_pixel);
            for w_pixel in 0..self.width {
                let mut color = Color::default();
                for sample in 0..self.samples_per_pixel {
                    let mut ray = self.get_ray(w_pixel, h_pixel);
                    color += Self::ray_color(&mut ray, world, self.max_depth);
                }
                self.write_color(color);
            }
        }
        println!("Done");
    }

    fn pixel_sample_square(&mut self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen_range(0.0..1.);
        let py = -0.5 + rng.gen_range(0.0..1.);
        (px * self.pixel_delta_u.clone()) + (py * self.pixel_delta_v.clone())
    }

    fn get_ray(&mut self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc.clone()
            + (i as f64 * self.pixel_delta_u.clone())
            + (j as f64 * self.pixel_delta_v.clone());
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_direction = pixel_sample - self.center.clone();

        Ray::new(self.center.clone(), ray_direction)
    }

    fn write_color(&mut self, color: Color) {
        let scale = 1. / (self.samples_per_pixel as f64);
        let r = color.x() * scale;
        let g = color.y() * scale;
        let b = color.z() * scale;

        let interval = Interval::new(0., 0.999);
        self.file.write_all(
            format!(
                "{} {} {}\n",
                (256. * interval.clamp(r.sqrt())) as i32,
                (256. * interval.clamp(g.sqrt())) as i32,
                (256. * interval.clamp(b.sqrt())) as i32,
            )
            .as_bytes(),
        );
    }

    fn ray_color(ray: &mut Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        let mut record = HitRecord::default();
        if world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut record) {
            let direction = record.normal + Vec3::random_unit();
            return 0.1 * Self::ray_color(&mut Ray::new(record.p, direction), world, depth - 1);
        }

        let unit = ray.dirction().unit();
        let a = 0.5 * (unit.y() + 1.);
        (1. - a) * Color::from((1., 1., 1.)) + a * Color::from((0.5, 0.7, 1.))
    }
}
