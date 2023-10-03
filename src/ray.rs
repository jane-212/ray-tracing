#![allow(unused)]
use crate::vec3::Point;

#[derive(Clone)]
pub struct Ray {
    origin: Point,
    direction: Point,
}

impl Ray {
    pub fn new(origin: impl Into<Point>, direction: impl Into<Point>) -> Self {
        Self {
            origin: origin.into(),
            direction: direction.into(),
        }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn dirction(&self) -> &Point {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point {
        let point = self.clone();
        point.origin + t * point.direction
    }
}
