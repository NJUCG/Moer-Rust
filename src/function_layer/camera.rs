use std::f32::consts::PI;
use std::rc::Rc;
use std::sync::Arc;
use image::imageops::tile;
use nalgebra::{Matrix4, Point3, Vector2, Vector3};
use serde_json::Value;
use crate::core_layer::transform::{Transform, Transformable};
use crate::function_layer::film::Film;
use crate::function_layer::ray::{Ray, RayDifferential};

type V2f = Vector2<f32>;

pub trait CameraT {
    fn sample_ray(&self, sample: &CameraSample, ndc: V2f) -> Ray;
    fn sample_ray_differentials(&self, sample: &CameraSample, ndc: V2f) -> Ray;
}

struct CameraSample {
    pub xy: V2f,
    pub lens: V2f,
    pub time: f32,
}

pub struct Camera {
    pub t_min: f32,
    pub t_max: f32,
    pub time_start: f32,
    pub time_end: f32,
    pub film: Option<Rc<Film>>,

    pub transform: Transform,
}

impl Camera {
    pub fn from_json(json: &Value) -> Self {
        let t_min = json["tNear"].as_f64().unwrap_or(1e-4) as f32;
        let t_max = json["tFar"].as_f64().unwrap_or(1e10) as f32;
        let time_start = json["timeStart"].as_f64().unwrap_or(0.0) as f32;
        let time_end = json["timeEnd"].as_f64().unwrap_or(0.0) as f32;
        let film = Some(Rc::new(Film::from_json(&json["film"])));
        let transform = Transform::identity();
        Self {
            t_min,
            t_max,
            time_start,
            time_end,
            film,
            transform,
        }
    }
}

pub struct PerspectiveCamera {
    pub c: Camera,
    pub vertical_fov: f32,
    pub aspect_ratio: f32,
}

impl PerspectiveCamera {
    pub fn from_json(json: &Value) -> Self {
        let mut c = Camera::from_json(json);
        let position = fetch_point(&json["transform"], &"position");
        let look_at = fetch_point(&json["transform"], "lookAt");
        let up = fetch_point(&json["transform"], "up");
        let up = Vector3::from_data(up.coords.data);
        let vertical_fov = json["verticalFov"].as_f64().unwrap() as f32 / 180.0 * PI;
        let aspect_ratio = c.film.as_ref().unwrap().size[0] as f32 /
            c.film.as_ref().unwrap().size[1] as f32;
        let forward = (look_at - position).normalize();
        let right = (forward.cross(&up)).normalize();
        let up = (right.cross(&forward)).normalize();

        let translation = Transform::translation(&Vector3::from_data(position.coords.data));
        let mut rotation = Matrix4::identity();
        rotation[(0, 0)] = right[0];
        rotation[(1, 0)] = right[1];
        rotation[(2, 0)] = right[2];

        rotation[(0, 1)] = up[0];
        rotation[(1, 1)] = up[1];
        rotation[(2, 1)] = up[2];

        rotation[(0, 2)] = -forward[0];
        rotation[(1, 2)] = -forward[1];
        rotation[(2, 2)] = -forward[2];
        c.transform = Transform::new(translation, rotation, Matrix4::identity());
        Self {
            c,
            vertical_fov,
            aspect_ratio,
        }
    }
}

fn fetch_point(json: &Value, field: &str) -> Point3<f32> {
    let arr: Vec<_> = json[field].as_array().unwrap()
        .iter().map(|e| e.as_f64().unwrap() as f32).collect();
    let res = Point3::from_slice(&arr);
    res
}

pub struct PinholeCamera {
    pub c: PerspectiveCamera,
}

impl PinholeCamera {
    pub fn from_json(json: &Value) -> Self {
        Self { c: PerspectiveCamera::from_json(json) }
    }

    pub fn film(&self) -> &Option<Rc<Film>> {
        &self.c.c.film
    }
    pub fn transform(&self) -> &Transform {
        &self.c.c.transform
    }
}

impl CameraT for PinholeCamera {
    fn sample_ray(&self, sample: &CameraSample, ndc: V2f) -> Ray {
        let film = self.film().as_ref().unwrap();
        let x = (ndc[0] - 0.5) * film.size[0] as f32 + sample.xy[0];
        let y = (0.5 - ndc[1]) * film.size[1] as f32 + sample.xy[1];
        let tan_half_fov = (self.c.vertical_fov * 0.5).tan();
        let z = film.size[1] as f32 * -0.5 / tan_half_fov;
        let direction = Vector3::new(x, y, z);
        let direction = self.transform().to_world_vec(&direction);
        let origin = self.transform().to_world_point(&Point3::origin());
        let mut ray = Ray::new(origin, direction);
        let c = &self.c.c;
        ray.t_min = c.t_min;
        ray.t_max = c.t_max;
        ray.t = c.time_start;
        ray
    }

    fn sample_ray_differentials(&self, sample: &CameraSample, ndc: V2f) -> Ray {
        let film = self.film().as_ref().unwrap();
        let x = (ndc[0] - 0.5) * film.size[0] as f32 + sample.xy[0];
        let y = (0.5 - ndc[1]) * film.size[1] as f32 + sample.xy[1];
        let tan_half_fov = (self.c.vertical_fov * 0.5).tan();
        let z = film.size[1] as f32 * -0.5 / tan_half_fov;
        let direction = self.transform().to_world_vec(&Vector3::new(x, y, z));
        let direction_x = self.transform().to_world_vec(&Vector3::new(x + 1.0, y, z));
        let direction_y = self.transform().to_world_vec(&Vector3::new(x, y + 1.0, z));
        let origin = self.transform().to_world_point(&Point3::origin());

        let mut ray = Ray::new(origin, direction);
        let c = &self.c.c;
        ray.t_min = c.t_min;
        ray.t_max = c.t_max;
        ray.t = c.time_start;
        ray.differential = Some(RayDifferential {
            origin_x: origin,
            origin_y: origin,
            direction_x,
            direction_y,
        });
        ray
    }
}

pub fn construct_camera(json: &Value) -> impl CameraT {
    if json["type"].as_str().expect("no camera type field") == "pinhole" {
        PinholeCamera::from_json(json)
    } else {
        panic!("Invalid camera type!")
    }
}