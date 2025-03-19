mod camera;
mod color;
mod hits;
mod material;
mod objects;
mod ray;
mod vec3;

use camera::Camera;
use color::color_to_rgb;
use hits::{Hittable, HittableList};
use material::{Lambertian, Metal};
use objects::Sphere;
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;
use vec3::{Color, Point3, Vec3};

fn ray_color(ray: &ray::Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::zeros();
    }

    // Check if ray hits anything in the world
    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        // If ray is scattered, calculate color contribution
        if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::zeros();
    }

    // Background - simple gradient based on ray direction y value
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image dimensions
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio);

    // Render
    let mut img_buf = image::RgbImage::new(image_width as u32, image_height as u32);
    let world_ref = Arc::new(world);

    println!("Rendering a {}x{} image with {} samples per pixel...", image_width, image_height, samples_per_pixel);

    // Create a vector of pixel coordinates
    let pixels: Vec<(u32, u32)> = (0..image_height as u32)
        .flat_map(|j| (0..image_width as u32).map(move |i| (i, j)))
        .collect();

    // Render pixels in parallel
    let colors: Vec<(u32, u32, [u8; 3])> = pixels
        .par_iter()
        .map(|(i, j)| {
            let mut rng = rand::thread_rng();
            let mut pixel_color = Color::zeros();

            // Multiple samples per pixel (anti-aliasing)
            for _ in 0..samples_per_pixel {
                let u = (*i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = ((image_height - 1 - *j as i32) as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, world_ref.as_ref(), max_depth);
            }

            (*i, *j, color_to_rgb(pixel_color, samples_per_pixel))
        })
        .collect();

    // Set pixels in the image buffer
    for (i, j, rgb) in colors {
        img_buf.put_pixel(i, j, image::Rgb(rgb));
    }

    // Save to a file
    match img_buf.save("output.png") {
        Ok(_) => println!("Image saved as output.png"),
        Err(e) => eprintln!("Error saving image: {}", e),
    }
}
