use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use interval::Interval;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point, Vec3};

mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new((0., 0., -1.).into(), 0.5)));
    world.add(Box::new(Sphere::new((0., -100.5, -1.).into(), 100.)));

    Camera::new().render(&world);
}
