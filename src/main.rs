mod hit;
mod ray;
mod vec3;
use hit::{HitRecord, Hittable, HittableList, Sphere};

use crate::ray::*; // <- this is new
use crate::vec3::vec3::*; // <- this is new
use std::io::prelude::*;
use std::{borrow::Borrow, fs::File, rc::Rc};

use image::{io::Reader as ImageReader, ImageBuffer, Rgb};

const INFINITY: f32 = f32::INFINITY;
const PI: f32 = 3.1415926535897932385;

fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}

fn write_color(mut file: &File, s: Vec3) {
    match file.write_all(
        format!(
            "{0} {1} {2}\n",
            (255.0 * s.x) as i32,
            (255.0 * s.y) as i32,
            (255.0 * s.z) as i32
        )
        .as_bytes(),
    ) {
        Err(_) => println!("Error writing color"),
        _ => (),
    };
}

fn write_color_old(s: Vec3) {
    println!(
        "{0} {1} {2}",
        (255.999 * s.x) as i32,
        (255.999 * s.y) as i32,
        (255.999 * s.z) as i32
    );
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    match world.hit(r, 0.0, INFINITY) {
        Some(rec) => {
            return 1.5 * (rec.normal * color(1.0, 1.0, 1.0));
        }
        None => {}
    }
    let u_direction: Vec3 = unit_vector(r.dir);
    let t = 0.5 * (u_direction.y + 1.0);
    return (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0);
}

fn create_gradient() {
    let width: i32 = 256;
    let height: i32 = 256;

    println!("P3");
    println!("{0} {1}", width, height);
    println!("255");

    for j in (0..height).rev() {
        for i in 0..width {
            let r: f32 = i as f32 / (width - 1) as f32;
            let g: f32 = j as f32 / (height - 1) as f32;
            let b: f32 = 0.25 as f32;
            write_color_old(Vec3 { x: r, y: g, z: b })
        }
    }
}

fn first_ray_trace() -> std::io::Result<()> {
    let mut file = File::create::<_>("image.ppm")?;
    // image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 1000;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // World
    let mut world: HittableList = HittableList::new();
    // world.add(Rc::new(Sphere::new(point3(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(point3(-0.5, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(point3(0.5, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(point3(0.0, 1.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(point3(0.0, -1.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(point3(0.0, -100.0, -1.0), 100.0)));

    // camera
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point3(0.0, 0.0, 0.0);
    let horziontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horziontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // render
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n", image_width, image_height).as_bytes())?;
    file.write_all(b"255\n")?;

    let mut imgbuff = ImageBuffer::new(image_width, image_height as u32);

    for (x, y, pixel) in imgbuff.enumerate_pixels_mut() {
        let u = x as f32 / (image_width as f32 - 1.0);
        let v = y as f32 / (image_height as f32 - 1.0);
        let r: Ray = Ray::new(
            &origin,
            &(lower_left_corner + u * horziontal + v * vertical - origin),
        );
        let pixel_color = ray_color(&r, &world);
        *pixel = image::Rgb([
            (255 as f32 * pixel_color.x) as u8,
            (255 as f32 * pixel_color.y) as u8,
            (255 as f32 * pixel_color.z) as u8,
        ]);
    }

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f32 / (image_width as f32 - 1.0);
            let v = j as f32 / (image_height as f32 - 1.0);
            let r: Ray = Ray::new(
                &origin,
                &(lower_left_corner + u * horziontal + v * vertical - origin),
            );
            let pixel_color = ray_color(&r, &world);
            write_color(&file, pixel_color);
        }
    }
    imgbuff.save("image.png").unwrap();
    Ok(())
}

// Taken from raysphere intersection https://raytracing.github.io/books/RayTracingInOneWeekend.html#addingasphere/ray-sphereintersection
fn hit_sphere(center: Point3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.orig - center;
    let a = &ray.dir.length_squared();
    let half_b = Vec3::dot(&oc, &ray.dir);
    let c = &oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn main() -> std::io::Result<()> {
    // create_gradient();
    match first_ray_trace() {
        Err(_) => println!("Error writing color"),
        Ok(_) => {}
    };
    Ok(())
}
