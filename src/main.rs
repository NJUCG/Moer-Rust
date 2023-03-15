mod function_layer;
mod core_layer;

use std::env::{args, current_dir, set_current_dir};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;
use std::rc::Rc;
use serde_json::Value;
use crate::function_layer::camera::{construct_camera, PinholeCamera};

fn main() -> Result<(), Box<dyn Error>> {
    let scene_dir = args().nth(1).expect("No input scene!");
    set_current_dir(scene_dir).expect("Invalid scene dir!");
    println!("{}", current_dir().unwrap().display());
    let scene_path = "scene.json";
    let scene = BufReader::new(File::open(scene_path).unwrap());
    let json: Value = serde_json::from_reader(scene)?;
    let a = &json["camera"]["tg"];
    let camera = construct_camera(&json["camera"]);

    Ok(())
}
