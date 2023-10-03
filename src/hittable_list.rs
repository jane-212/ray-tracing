#![allow(unused)]
use crate::{HitRecord, Hittable, Interval};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &mut crate::ray::Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let mut record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut record) {
                hit_anything = true;
                closest_so_far = record.t;
                *hit_record = record.clone();
            }
        }

        hit_anything
    }
}
