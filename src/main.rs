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
    // stop if we've bounced too much
    if depth <= 0 {
        return Color::zeros();
    }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) {
            let scattered_color = ray_color(&scattered, world, depth - 1);
            return Color::new(
                attenuation.x * scattered_color.x,
                attenuation.y * scattered_color.y,
                attenuation.z * scattered_color.z,
            );
        }
        return Color::zeros();
    }

    // sky color - just a simple gradient
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;  // more samples = better quality but slower
    let max_depth = 50;  // max light bounces

    let mut world = HittableList::new();

    // setup materials
    let ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // add spheres
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, ground)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, center)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, left)));
    world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, right)));

    // camera setup
    let camera = Camera::new(
        Point3::new(3.0, 3.0, 2.0),  
        Point3::new(0.0, 0.0, -1.0),  
        Vec3::new(0.0, 1.0, 0.0),  
        20.0,  
        aspect_ratio
    );

    let mut img_buf = image::RgbImage::new(image_width as u32, image_height as u32);
    let world_ref = Arc::new(world);

    println!("Rendering {}x{} with {} samples per pixel...", image_width, image_height, samples_per_pixel);

    
    let pixels: Vec<(u32, u32)> = (0..image_height as u32)
        .flat_map(|j| (0..image_width as u32).map(move |i| (i, j)))
        .collect();

    // parallel render
    let colors: Vec<(u32, u32, [u8; 3])> = pixels
        .par_iter()
        .map(|(i, j)| {
            let mut rng = rand::thread_rng();
            let mut pixel_color = Color::zeros();

            for _ in 0..samples_per_pixel {
                let u = (*i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = ((image_height - 1 - *j as i32) as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, world_ref.as_ref(), max_depth);
            }

            (*i, *j, color_to_rgb(pixel_color, samples_per_pixel))
        })
        .collect();

    
    for (i, j, rgb) in colors {
        img_buf.put_pixel(i, j, image::Rgb(rgb));
    }

    
    match img_buf.save("output.png") {
        Ok(_) => println!("Done! Saved as output.png"),
        Err(e) => eprintln!("Oops, error saving image: {}", e),
    }
}
