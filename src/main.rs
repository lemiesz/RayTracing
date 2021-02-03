mod ray;
mod vec3;
use crate::ray::*; // <- this is new
use crate::vec3::vec3::*; // <- this is new
use std::{borrow::Borrow, fs::File};
use std::io::prelude::*;

use Vec3 as Color;
use Vec3 as Point3;

fn color(r: f32, g: f32, b: f32) -> Color {
    return Color::new(r, g, b);
}
fn point3(x: f32, y: f32, z: f32) -> Point3 {
    return Point3::new(x, y, z);
}

fn write_color(mut file: &File, s: Vec3) {
    match file.write_all(
        format!(
            "{0} {1} {2}\n",
            (255.999 * s.x) as i32,
            (255.999 * s.y) as i32,
            (255.999 * s.z) as i32
        )
        .as_bytes(),
    ) {
        Err(e) => println!("Error writing color"),
        _ => ()
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

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(point3(0.0, 0.0, -1.0), 0.5, r) {
        return color(1.0, 0.0, 0.0);
    }
    let u_direction: Vec3 = unit_vector(r.dir);
    let t: f32 = 0.5 * (u_direction.y) + 1.0;
    let c = color(1.0, 1.0, 1.0);
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
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

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
    file.write_all(b"P3\n");
    file.write_all(format!("{} {}\n", image_width, image_height).as_bytes());
    file.write_all(b"255\n");
  
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f32 / (image_width as f32 - 1.0);
            let v = j as f32 / (image_height as f32 - 1.0);
            let r: Ray = Ray::new(
                &origin,
                &(lower_left_corner + u * horziontal + v * vertical - origin),
            );
            let pixel_color = ray_color(&r);
            write_color(&file, pixel_color);
        }
    }
    Ok(())
}

// Taken from raysphere intersection https://raytracing.github.io/books/RayTracingInOneWeekend.html#addingasphere/ray-sphereintersection
fn hit_sphere(center: Point3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.orig - center;
    let a = Vec3::dot(&ray.dir, &ray.dir);
    let b = 2.0 * Vec3::dot(&oc, &ray.dir);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0*a*c;
    return discriminant > 0.0;
}

fn main() -> std::io::Result<()> {
    // create_gradient();
    match first_ray_trace() {
        Err(e) => println!("Error writing color"),
        Ok(_) => {}
    };
    Ok(())
}
