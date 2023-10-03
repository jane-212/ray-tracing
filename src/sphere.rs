use crate::hittable::Hittable;
use crate::{Interval, Point};

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &mut crate::ray::Ray,
        ray_t: Interval,
        hit_record: &mut crate::hittable::HitRecord,
    ) -> bool {
        let oc = ray.origin().clone() - self.center.clone();
        let a = ray.dirction().length_squared();
        let half_b = ray.dirction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        hit_record.set_face_normal(
            ray,
            &((hit_record.p.clone() - self.center.clone()) / self.radius),
        );

        true
    }
}
