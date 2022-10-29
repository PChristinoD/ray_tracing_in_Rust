use image::RgbImage;
use log::{info, warn};
use ray_tracing::camera::Camera;
use ray_tracing::objects_list::ObjectList;
use ray_tracing::ray::Ray;
use ray_tracing::utility::*;
use ray_tracing::vec3::{length, mul};
use ray_tracing::{objects::*, INFI};
use ray_tracing::{vec3, Vec3};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use std::sync::Arc;

static mut PIXEL: u32 = 0;
fn main() {
    env_logger::init();
    let aspect_ratio = 16.0 / 9.0;
    let width = 640;
    let height = (width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let depth = 50;

    // cam
    let vfov = 20.0;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    info!("{}", dist_to_focus);
    let cam = Camera::new(
        look_from,
        look_at,
        v_up,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    //Materials
    //let metal_left = Arc::new(Dielectrics::new(1.5));
    //let metal_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)));

    //Objects
    //let mut world = ObjectList::new();
    //let sphere = Sphere::new(vec3::new_with_z(-1.0), 0.5, metal_center.clone());
    //world.add(Arc::new(sphere));
    //world.add(Arc::new(Sphere::new(
    //    vec3::new(0.0, -100.5, -1.0),
    //    100.0,
    //    metal_ground.clone(),
    //)));
    //world.add(Arc::new(Sphere::new(
    //    vec3::new(1.0, 0.0, -1.0),
    //    0.5,
    //   metal_right.clone(),
    // )));
    //world.add(Arc::new(Sphere::new(
    //   vec3::new(-1.0, 0.0, -1.0),
    //   0.5,
    //    metal_left.clone(),
    //)));
    let world = random_scene();

    info!("Start initializing raw data)");
    let mut raw_data = vec![vec![0, 0, 0]; (width * height) as usize];
    raw_data.par_iter_mut().enumerate().for_each(|(i, rgb)| {
        let y = i as u32 / width;
        let x = i as u32 % width;
        let color = write_color(
            pixel_color(x, y, width, height, &cam, &world, samples_per_pixel, depth),
            samples_per_pixel,
        );
        *rgb = color;
    });
    let buf = raw_data.into_par_iter().flatten().collect();
    let img = RgbImage::from_raw(width, height, buf).unwrap();

    info!("Finished");
    match img.save("first_image.png") {
        Ok(()) => info!("Save image success!"),
        Err(e) => warn!("{}", e),
    }
}

//Maybe I should rewrite it into non-recursive function.
//But I can't :(
pub fn ray_color(ray: &Ray, obj: &ObjectList, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::from([0.0, 0.0, 0.0]);
    }
    let mut rec = HitRecord::default();
    if obj.be_hit(ray, 0.001, INFI, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::default();
        if rec
            .material
            .scatter(ray, &rec, &mut attenuation, &mut scattered)
        {
            let color = ray_color(&scattered, obj, depth - 1);
            return mul(&attenuation, &color);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let unit_dir = vec3::unit(&ray.dir);
    let t = (unit_dir.y + 1.0) * 0.5;
    vec3::new_with_number(1.0 - t) * t + vec3::new(0.5, 0.7, 1.0) * t
}
pub fn pixel_color(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    cam: &Camera,
    world: &ObjectList,
    samples_per_pixel: u32,
    depth: u32,
) -> Vec3 {
    //Mystery
    //Seems that rustc could vectorize this loop
    //Hence it performed better than parallel computation in rayon.
    //rayon is more like concurrency library than parallel computation library, I guess.
    //I'm wondering whether I should use SIMD to accelerate the loop
    //but if vectorization happened,
    //it's not necessary.
    let pixel = (0..samples_per_pixel)
        .into_iter()
        .fold(Vec3::default(), |pixel_color, _| {
            let u = (x as f64 + rand()) / (width as f64 - 1.0);
            let v = (y as f64 + rand()) / (height as f64 - 1.0);
            let ray = cam.get_ray(u, v);
            let ray_color = ray_color(&ray, world, depth);
            pixel_color + ray_color
        });

    unsafe {
        PIXEL += 1;
        print!("{}/{}\r", PIXEL, width * height);
    }

    pixel
}

pub fn write_color(rgb: Vec3, samples_per_pixel: u32) -> Vec<u8> {
    let scale = 1.0 / samples_per_pixel as f64;
    rgb.map(|x| ((x * scale).sqrt() * 255.0) as u8)
        .as_slice()
        .to_vec()
}

pub fn random_scene() -> ObjectList {
    let mut world = ObjectList::new();
    let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let ground = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(Arc::new(ground));

    for a in -11..11 {
        for b in -11..11 {
            let mat = rand();
            let pos = Vec3::new(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());

            if length(&(pos - Vec3::new(4.0, 0.2, 0.0))) > 0.9 {
                if mat < 0.8 {
                    let albedo = mul(&Vec3::from(rand_slice()), &Vec3::from(rand_slice()));
                    let material = Arc::new(Lambertian::new(albedo));
                    let sphere = Arc::new(Sphere::new(pos, 0.2, material));
                    world.add(sphere);
                }
            } else if mat < 0.95 {
                let albedo = mul(&Vec3::from(rand_slice()), &Vec3::from(rand_slice()));
                let material = Arc::new(Metal::new(albedo));
                let sphere = Arc::new(Sphere::new(pos, 0.2, material));
                world.add(sphere);
            } else {
                let material = Arc::new(Dielectrics::new(1.5));
                let sphere = Arc::new(Sphere::new(pos, 0.2, material));
                world.add(sphere);
            }
        }
    }

    let material1 = Arc::new(Dielectrics::new(1.5));
    let sphere1 = Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));
    world.add(sphere1);
    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    let sphere2 = Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));
    world.add(sphere2);
    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5)));
    let sphere3 = Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));
    world.add(sphere3);

    world
}
