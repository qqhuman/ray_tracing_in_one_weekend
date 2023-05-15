use ray_tarcing_in_one_weekend::rt::{
    camera::Camera,
    color::{self, Color},
    materials::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
        Material,
    },
    random_f64, random_f64_between, random_vec3, random_vec3_between, render_pixel,
    shapes::{
        bbox::Bbox, bvh_node::BvhNode, constant_volume::ConstantVolume, flip_face::FlipFace,
        hittable_list::HittableList, mooving_sphere::MovingSphere, rotate_y::RotateY,
        sphere::Sphere, translate::Translate, xy_rect::XyRect, xz_rect::XzRect, yz_rect::YzRect,
        Hittable,
    },
    textures::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, noise_texture::NoiseTexture,
    },
    vec3::Vec3,
    Point3,
};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::{sync::Arc, time::Instant};

use image::{ImageBuffer, Rgb, RgbImage};

const PARALLEL: bool = true;

fn main() {
    // World, Camera
    let (world, camera, background) = _cornell_aluminum_glass();
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));
    let mut lights = HittableList::default();
    lights.add(Arc::new(XzRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light.clone(),
    )));
    lights.add(Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        light.clone(),
    )));

    // IMAGE
    let mut buffer: RgbImage = ImageBuffer::new(camera.width as u32, camera.height as u32);

    // PROGRESS BAR
    let bar = ProgressBar::new((buffer.width() * buffer.height()) as u64);
    bar.set_style(ProgressStyle::with_template("{wide_bar} {percent}%").unwrap());

    let start = Instant::now();
    if PARALLEL {
        buffer.enumerate_pixels_mut().par_bridge().for_each(|arg| {
            iterate_pixel(arg, &camera, background, &world, &lights);
            bar.inc(1);
        });
    } else {
        buffer.enumerate_pixels_mut().for_each(|arg| {
            iterate_pixel(arg, &camera, background, &world, &lights);
            bar.inc(1);
        });
    }
    let duration = start.elapsed();
    println!("Time elapsed in drawing is: {:?}", duration);

    match buffer.save("image.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(()) => println!("Done."),
    };
}

fn _random_scene() -> (HittableList, Camera, Color) {
    let mut world = HittableList::new_from_object(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_color(Color::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat = random_f64();
            let center = Point3::new(a + 0.9 * random_f64(), 0.2, b + 0.9 * random_f64());
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    _ if choose_mat < 0.8 => {
                        let material: Arc<dyn Material> =
                            Arc::new(Lambertian::from_color(random_vec3() * random_vec3()));
                        let center2 = center + Vec3::new(0.0, random_f64_between(0.0, 0.5), 0.0);
                        world.add(Arc::new(MovingSphere::new(
                            center, center2, 0.0, 1.0, 0.2, material,
                        )));
                    }
                    _ if choose_mat < 0.95 => {
                        let material: Arc<dyn Material> =
                            Arc::new(Metal::new(random_vec3(), random_f64_between(0.0, 0.5)));
                        world.add(Arc::new(Sphere::new(center, 0.2, material)));
                    }
                    _ => {
                        let material: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
                        world.add(Arc::new(Sphere::new(center, 0.2, material)));
                    }
                };
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1))),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;
    let time0 = 0.0;
    let time1 = 1.0;
    let aspect_ratio = 3.0 / 2.0;
    let width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aperture,
        dist_to_focus,
        time0,
        time1,
        aspect_ratio,
        width,
        samples_per_pixel,
        max_depth,
    );

    (world, camera, Color::new(0.7, 0.8, 1.0))
}

fn _two_spheres() -> (HittableList, Camera, Color) {
    let checker = Arc::new(CheckerTexture::from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let material: Arc<dyn Material> = Arc::new(Lambertian::from_texture(checker));
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        material.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        material,
    )));

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 20.0;
    let time0 = 0.0;
    let time1 = 1.0;
    let aspect_ratio = 3.0 / 2.0;
    let width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aperture,
        dist_to_focus,
        time0,
        time1,
        aspect_ratio,
        width,
        samples_per_pixel,
        max_depth,
    );

    (world, camera, Color::new(0.7, 0.8, 1.0))
}

fn _two_perlin_spheres() -> (HittableList, Camera, Color) {
    let pertext = Arc::new(NoiseTexture::new(4.0));
    let material: Arc<dyn Material> = Arc::new(Lambertian::from_texture(pertext));
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        material,
    )));

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 20.0;
    let time0 = 0.0;
    let time1 = 1.0;
    let aspect_ratio = 3.0 / 2.0;
    let width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aperture,
        dist_to_focus,
        time0,
        time1,
        aspect_ratio,
        width,
        samples_per_pixel,
        max_depth,
    );

    (world, camera, Color::new(0.7, 0.8, 1.0))
}

fn _earth() -> (HittableList, Camera, Color) {
    let earth_texture = Arc::new(ImageTexture::from_file(
        "C:\\my_space\\Code\\rust\\ray_tarcing_in_one_weekend\\textures\\earthmap.jpg",
    ));
    let material: Arc<dyn Material> = Arc::new(Lambertian::from_texture(earth_texture));
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        material.clone(),
    )));

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    let time0 = 0.0;
    let time1 = 1.0;
    let aspect_ratio = 3.0 / 2.0;
    let width = 800;
    let samples_per_pixel = 10000;
    let max_depth = 50;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aperture,
        dist_to_focus,
        time0,
        time1,
        aspect_ratio,
        width,
        samples_per_pixel,
        max_depth,
    );

    (world, camera, Color::new(0.7, 0.8, 1.0))
}

fn _simple_light() -> (HittableList, Camera, Color) {
    let pertext = Arc::new(NoiseTexture::new(4.0));
    let material: Arc<dyn Material> = Arc::new(Lambertian::from_texture(pertext));
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        material,
    )));

    let difflight: Arc<dyn Material> =
        Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    let lookfrom = Point3::new(26.0, 3.0, 6.0);
    let lookat = Point3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 20.0;
    let time0 = 0.0;
    let time1 = 1.0;
    let aspect_ratio = 3.0 / 2.0;
    let width = 400;
    let samples_per_pixel = 400;
    let max_depth = 50;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aperture,
        dist_to_focus,
        time0,
        time1,
        aspect_ratio,
        width,
        samples_per_pixel,
        max_depth,
    );

    (world, camera, color::BLACK)
}

fn _cornell_box() -> (HittableList, Camera, Color) {
    let red: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    let mut world = HittableList::default();
    let box1 = Bbox::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = RotateY::new(Arc::new(box1), 15.0);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(265.0, 0.0, 295.0));
    world.add(Arc::new(box1));
    let box2 = Bbox::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = RotateY::new(Arc::new(box2), -18.0);
    let box2 = Translate::new(Arc::new(box2), Vec3::new(130.0, 0.0, 65.0));
    world.add(Arc::new(box2));

    world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));

    world.add(Arc::new(FlipFace::new(Arc::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )))));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Arc::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    let time0 = 0.0;
    let time1 = 1.0;
    let aspect_ratio = 1.0;
    let width = 600;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aperture,
        dist_to_focus,
        time0,
        time1,
        aspect_ratio,
        width,
        samples_per_pixel,
        max_depth,
    );

    (world, camera, color::BLACK)
}

fn _cornell_aluminum_glass() -> (HittableList, Camera, Color) {
    let red: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    let mut world = HittableList::default();
    world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Arc::new(FlipFace::new(Arc::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )))));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Arc::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

    let aluminum: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.0));
    let box1 = Bbox::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        aluminum,
    );
    let box1 = RotateY::new(Arc::new(box1), 15.0);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(265.0, 0.0, 295.0));
    world.add(Arc::new(box1));
    world.add(Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        Arc::new(Dielectric::new(1.5)),
    )));

    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    let time0 = 0.0;
    let time1 = 1.0;
    let aspect_ratio = 1.0;
    let width = 600;
    let samples_per_pixel = 1000;
    let max_depth = 50;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aperture,
        dist_to_focus,
        time0,
        time1,
        aspect_ratio,
        width,
        samples_per_pixel,
        max_depth,
    );

    (world, camera, color::BLACK)
}

fn _cornell_smoke() -> (HittableList, Camera, Color) {
    let red: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));

    let mut world = HittableList::default();
    let box1 = Bbox::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = RotateY::new(Arc::new(box1), 15.0);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(265.0, 0.0, 295.0));
    let box1 = ConstantVolume::from_color(Arc::new(box1), 0.01, color::BLACK);
    world.add(Arc::new(box1));
    let box2 = Bbox::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = RotateY::new(Arc::new(box2), -18.0);
    let box2 = Translate::new(Arc::new(box2), Vec3::new(130.0, 0.0, 65.0));
    let box2 = ConstantVolume::from_color(Arc::new(box2), 0.01, color::WHITE);
    world.add(Arc::new(box2));

    world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));

    world.add(Arc::new(XzRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Arc::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    let time0 = 0.0;
    let time1 = 1.0;
    let aspect_ratio = 1.0;
    let width = 600;
    let samples_per_pixel = 200;
    let max_depth = 50;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aperture,
        dist_to_focus,
        time0,
        time1,
        aspect_ratio,
        width,
        samples_per_pixel,
        max_depth,
    );

    (world, camera, color::BLACK)
}

fn _bvh_test() -> (HittableList, Camera, Color) {
    let material: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let mut world = HittableList::default();

    let mut spheres = HittableList::default();
    for i in 0..10000 {
        let i = i as f64;
        spheres.add(Arc::new(Sphere::new(
            Point3::new(0.0, 100.0 + i * 2.0, 0.0),
            1.0,
            material.clone(),
        )));
    }

    let bvh = Arc::new(BvhNode::from_list(&mut spheres, 0.0, 1.0));
    world.add(bvh);
    //world.add(Arc::new(spheres));

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 20.0;
    let time0 = 0.0;
    let time1 = 1.0;
    let aspect_ratio = 3.0 / 2.0;
    let width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aperture,
        dist_to_focus,
        time0,
        time1,
        aspect_ratio,
        width,
        samples_per_pixel,
        max_depth,
    );

    (world, camera, Color::new(0.7, 0.8, 1.0))
}

fn _final_scene() -> (HittableList, Camera, Color) {
    let mut boxes1 = HittableList::default();
    let ground: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_between(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Arc::new(Bbox::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects = HittableList::default();
    objects.add(Arc::new(BvhNode::from_list(&mut boxes1, 0.0, 1.0)));

    let light = DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0));
    objects.add(Arc::new(XzRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        Arc::new(light),
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::from_color(Color::new(0.7, 0.3, 0.1));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        Arc::new(moving_sphere_material),
    )));

    let dielectric = Arc::new(Dielectric::new(1.5));
    objects.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        dielectric.clone(),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary: Arc<dyn Hittable> = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        dielectric.clone(),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantVolume::from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary: Arc<dyn Hittable> =
        Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, dielectric));
    objects.add(Arc::new(ConstantVolume::from_color(
        boundary,
        0.0001,
        color::WHITE,
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        Arc::new(Lambertian::from_texture(Arc::new(ImageTexture::from_file(
            "C:\\my_space\\Code\\rust\\ray_tarcing_in_one_weekend\\textures\\earthmap.jpg",
        )))),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::from_texture(Arc::new(NoiseTexture::new(0.1)))),
    )));

    let mut boxes2 = HittableList::default();
    let white: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    for _ in 0..1000 {
        boxes2.add(Arc::new(Sphere::new(
            random_vec3_between(0.0, 165.0),
            10.0,
            white.clone(),
        )))
    }

    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::from_list(&mut boxes2, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let lookfrom = Point3::new(478.0, 278.0, -600.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    let time0 = 0.0;
    let time1 = 1.0;
    let aspect_ratio = 1.0;
    let width = 800;
    let samples_per_pixel = 10000;
    let max_depth = 50;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aperture,
        dist_to_focus,
        time0,
        time1,
        aspect_ratio,
        width,
        samples_per_pixel,
        max_depth,
    );

    (objects, camera, color::BLACK)
}

fn iterate_pixel(
    arg: (u32, u32, &mut Rgb<u8>),
    camera: &Camera,
    background: Color,
    world: &dyn Hittable,
    lights: &dyn Hittable,
) {
    let (x, y, pixel) = arg;
    // flip y to match results in the book
    let y = camera.height as u32 - 1 - y;
    let color = render_pixel(x, y, camera, background, world, lights);
    write_color(pixel, color, camera.samples_per_pixel);
}

fn write_color(pixel: &mut Rgb<u8>, color: Color, samples_per_pixel: usize) {
    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let color = color * (1.0 / samples_per_pixel as f64);
    let color = color.clamp(0.0, 1.0).sqrt() * 255.999;
    let ir = color.x as u8;
    let ig = color.y as u8;
    let ib = color.z as u8;
    *pixel = Rgb([ir, ig, ib]);
}
