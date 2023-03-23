mod function_layer;
mod core_layer;
mod resource_layer;

use std::env::{args, current_dir, set_current_dir};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufReader, Write};
use std::time::Instant;
use image::ImageFormat;
use nalgebra::Vector2;
use serde_json::Value;
use function_layer::{Camera, Scene, construct_integrator, construct_sampler, construct_camera};
use function_layer::camera::CameraSample;
use core_layer::colorspace::SpectrumRGB;

fn main() -> Result<(), Box<dyn Error>> {
    let scene_dir = args().nth(1).expect("No input scene!");
    set_current_dir(scene_dir).expect("Invalid scene dir!");
    println!("{}", current_dir().unwrap().display());
    let scene_path = "scene.json";
    let scene = BufReader::new(File::open(scene_path).unwrap());
    let json: Value = serde_json::from_reader(scene)?;
    let camera = construct_camera(&json["camera"]);
    let scene = Scene::from_json(&json["scene"]);
    let integrator = construct_integrator(&json["integrator"]);
    let sampler = construct_sampler(&json["sampler"]);
    let spp = sampler.borrow().xsp() * sampler.borrow().ysp() * 10;
    println!("spp: {spp}");
    let film = camera.film().unwrap();
    let [width, height] = film.borrow().size;
    let start = Instant::now();
    for y in 20..height {
        for x in 0..width {
            let ndc = Vector2::new(x as f32 / width as f32, y as f32 / height as f32);
            let mut li = SpectrumRGB::same(0.0);
            for _ in 0..spp {
                let mut ray = camera.sample_ray_differentials(
                    &CameraSample { xy: sampler.borrow_mut().next_2d(), lens: Vector2::zeros(), time: 0.0 }, ndc,
                );
                let delta_li =integrator.li(&mut ray, &scene, sampler.clone());
                li += delta_li;
            }
            film.borrow_mut().deposit(Vector2::new(x, y), &(li / spp as f32));
        }
        update_progress(y as f64 / height as f64);
    }
    println!();
    println!("Render complete: ");
    println!("Time taken: {:.2} s", start.elapsed().as_secs_f32());

    let out_name = json["output"]["filename"].as_str().unwrap();
    let format = if out_name.ends_with(".png") {
        ImageFormat::Png
    } else if out_name.ends_with(".hdr") {
        ImageFormat::Hdr
    } else {
        eprintln!("Unknown image format. The default is png");
        ImageFormat::Png
    };
    film.borrow().save(out_name, format).unwrap();
    Ok(())
}


pub fn update_progress(progress: f64) {
    let bar_width = 70;
    print!("[");
    let pos = bar_width as f64 * progress;
    for i in 0..bar_width {
        if i < pos as i32 {
            print!("=");
        } else if i == pos as i32 { print!(">"); } else { print!(" "); }
    }
    print!("] {} %", (progress * 100.0) as i32);
    io::stdout().flush().unwrap();
    print!("\r");
}
