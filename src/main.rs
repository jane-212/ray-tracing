use ray::Ray;
use vec3::{Color, Point, Vec3};

mod ray;
mod vec3;

fn main() {
    let aspect_ratio = 16. / 9.;

    let width = 400;
    let height = ((width as f64 / aspect_ratio) as i32).clamp(1, i32::MAX);

    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (width as f64) / (height as f64);
    let camera_center = Point::new(0., 0., 0.);

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    let pixel_delta_u = viewport_u.clone() / width as f64;
    let pixel_delta_v = viewport_v.clone() / height as f64;

    let viewport_upper_left =
        camera_center.clone() - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u.clone() + pixel_delta_v.clone());

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for h_pixel in 0..height {
        for w_pixel in 0..width {
            let pixel_center = pixel00_loc.clone()
                + (w_pixel as f64 * pixel_delta_u.clone())
                + (h_pixel as f64 * pixel_delta_v.clone());
            let ray_direction = pixel_center - camera_center.clone();
            let ray = Ray::new(camera_center.clone(), ray_direction);

            let color = ray_color(&ray);
            println!("{}", color);
        }
    }
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere((0., 0., -1.).into(), 0.5, ray);
    if t > 0. {
        let n = (ray.at(t) - (0., 0., -1.).into()).unit();
        return 0.5 * Color::new(n.x() + 1., n.y() + 1., n.z() + 1.);
    }

    let unit = ray.dirction().unit();
    let a = 0.5 * (unit.y() + 1.);
    (1. - a) * Color::from((1., 1., 1.)) + a * Color::from((0.5, 0.7, 1.))
}

fn hit_sphere(center: Point, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin().clone() - center;
    let a = ray.dirction().length_squared();
    let half_b = ray.dirction().dot(&oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0. {
        -1.
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}
