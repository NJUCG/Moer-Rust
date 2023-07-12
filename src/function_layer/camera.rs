use crate::core_layer::transform::Transform;
use crate::function_layer::{fetch_v3f, ray::RayDifferential, Film, Medium, Ray, V3f, RR};
use cgmath::{EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix, Vector2, Zero};
use serde_json::Value;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

type V2f = Vector2<f32>;

pub trait Camera {
    fn sample_ray(&self, sample: &CameraSample, ndc: V2f) -> Ray;
    fn sample_ray_differentials(&self, sample: &CameraSample, ndc: V2f) -> Ray;
    fn film(&self) -> Option<RR<Film>>;
    fn transform(&self) -> &Transform;
}

pub struct CameraSample {
    pub xy: V2f,
    pub lens: V2f,
    pub time: f32,
}

pub struct CameraBase {
    pub t_min: f32,
    pub t_max: f32,
    pub time_start: f32,
    pub time_end: f32,
    pub film: Option<RR<Film>>,

    pub transform: Transform,
    pub medium: Option<Rc<dyn Medium>>,
}

impl CameraBase {
    pub fn from_json(json: &Value) -> Self {
        let t_min = json["tNear"].as_f64().unwrap_or(1e-4) as f32;
        let t_max = json["tFar"].as_f64().unwrap_or(1e10) as f32;
        let time_start = json["timeStart"].as_f64().unwrap_or(0.0) as f32;
        let time_end = json["timeEnd"].as_f64().unwrap_or(0.0) as f32;
        let film = Some(Rc::new(RefCell::new(Film::from_json(&json["film"]))));
        let transform = Transform::identity();
        Self {
            t_min,
            t_max,
            time_start,
            time_end,
            film,
            transform,
            medium: None,
        }
    }
}

pub struct PerspectiveCamera {
    pub c: CameraBase,
    pub vertical_fov: f32,
    pub aspect_ratio: f32,
}

impl PerspectiveCamera {
    pub fn from_json(json: &Value) -> Self {
        let mut c = CameraBase::from_json(json);
        let position = fetch_point(&json["transform"], &"position");
        let look_at = fetch_point(&json["transform"], "lookAt");
        let up = fetch_point(&json["transform"], "up");
        let up = up.to_vec();
        let vertical_fov = json["verticalFov"].as_f64().unwrap() as f32 / 180.0 * PI;
        let aspect_ratio = c.film.as_ref().unwrap().borrow().size[0] as f32
            / c.film.as_ref().unwrap().borrow().size[1] as f32;
        let forward = (look_at - position).normalize();
        let right = forward.cross(up).normalize();
        let up = right.cross(forward).normalize();

        let translation = Transform::translation(position.to_vec());
        let mut rotation = Matrix4::identity();
        rotation[0][0] = right[0];
        rotation[0][1] = right[1];
        rotation[0][2] = right[2];

        rotation[1][0] = up[0];
        rotation[1][1] = up[1];
        rotation[1][2] = up[2];

        rotation[2][0] = -forward[0];
        rotation[2][1] = -forward[1];
        rotation[2][2] = -forward[2];
        c.transform = Transform::new(translation, rotation, Matrix4::identity());
        Self {
            c,
            vertical_fov,
            aspect_ratio,
        }
    }
}

fn fetch_point(json: &Value, field: &str) -> Point3<f32> {
    let arr = fetch_v3f(json, field, V3f::zero());
    let res = Point3::from([arr.x, arr.y, arr.z]);
    res
}

pub struct PinholeCamera {
    pub c: PerspectiveCamera,
}

impl PinholeCamera {
    pub fn from_json(json: &Value) -> Self {
        Self {
            c: PerspectiveCamera::from_json(json),
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.c.c.transform
    }
}

impl Camera for PinholeCamera {
    fn sample_ray(&self, sample: &CameraSample, ndc: V2f) -> Ray {
        let binding = self.film().unwrap();
        let film = binding.borrow();
        let x = (ndc[0] - 0.5) * film.size[0] as f32 + sample.xy[0];
        let y = (0.5 - ndc[1]) * film.size[1] as f32 + sample.xy[1];
        let tan_half_fov = (self.c.vertical_fov * 0.5).tan();
        let z = film.size[1] as f32 * -0.5 / tan_half_fov;
        let direction = V3f::new(x, y, z);
        let direction = self.transform().to_world_vec(direction);
        let origin = self.transform().to_world_point(Point3::origin());
        let mut ray = Ray::new(origin, direction);
        let c = &self.c.c;
        ray.t_min = c.t_min;
        ray.t_max = c.t_max;
        ray.t = c.time_start;
        ray.medium = self.c.c.medium.clone();
        ray
    }

    fn sample_ray_differentials(&self, sample: &CameraSample, ndc: V2f) -> Ray {
        let binding = self.film().unwrap();
        let film = binding.borrow();
        let x = (ndc[0] - 0.5) * film.size[0] as f32 + sample.xy[0];
        let y = (0.5 - ndc[1]) * film.size[1] as f32 + sample.xy[1];
        let tan_half_fov = (self.c.vertical_fov * 0.5).tan();
        let z = film.size[1] as f32 * -0.5 / tan_half_fov;
        let direction = self
            .transform()
            .to_world_vec(V3f::new(x, y, z))
            .normalize();
        let direction_x = self
            .transform()
            .to_world_vec(V3f::new(x + 1.0, y, z))
            .normalize();
        let direction_y = self
            .transform()
            .to_world_vec(V3f::new(x, y + 1.0, z))
            .normalize();
        let origin = self.transform().to_world_point(Point3::origin());

        let mut ray = Ray::new(origin, direction);
        let c = &self.c.c;
        ray.t_min = c.t_min;
        ray.t_max = c.t_max;
        ray.t = c.time_start;
        ray.medium = self.c.c.medium.clone();
        ray.differential = Some(RayDifferential {
            origin_x: origin,
            origin_y: origin,
            direction_x,
            direction_y,
        });
        ray
    }

    fn film(&self) -> Option<RR<Film>> {
        match &self.c.c.film {
            None => None,
            Some(f) => Some(f.clone()),
        }
    }

    fn transform(&self) -> &Transform {
        &self.c.c.transform
    }
}

pub fn construct_camera(json: &Value) -> impl Camera {
    if json["type"].as_str().expect("no camera type field") == "pinhole" {
        PinholeCamera::from_json(json)
    } else {
        panic!("Invalid camera type!")
    }
}
