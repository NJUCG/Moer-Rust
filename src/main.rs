extern crate core;

mod function_layer;
mod core_layer;

use std::env::{args, current_dir, set_current_dir};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use image::ImageFormat;
use nalgebra::{Vector2, Vector3};
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
    let spp = sampler.xsp() * sampler.ysp();
    let film = camera.film().unwrap();
    let [width, height] = film.borrow().size;
    for y in 0..height {
        for x in 0..width {
            let ndc = Vector2::new(x as f32 / width as f32, y as f32 / height as f32);
            let mut li = SpectrumRGB::same(0.0);
            for _ in 0..spp {
                let ray = camera.sample_ray_differentials(
                    &CameraSample { xy: sampler.next_2d(), lens: Vector2::zeros(), time: 0.0 }, ndc,
                );
                li += integrator.li(&ray, &scene, sampler.clone());
            }
            film.borrow_mut().deposit(Vector2::new(x, y), &(li / spp as f32));
        }
    }
    film.borrow().save(json["output"]["filename"].as_str().unwrap(), ImageFormat::Hdr);
    Ok(())
}
