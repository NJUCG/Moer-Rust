#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;
use nalgebra::{Matrix4, Vector3};
use serde_json::Value;
use crate::core_layer::transform::{Transform, Transformable};
use crate::function_layer::{Bounds3, Light, Material, Ray};
use crate::function_layer::material::material::construct_material;
use crate::function_layer::material::matte::MatteMaterial;
use super::intersection::Intersection;
use super::triangle::Triangle;

pub trait Shape: Transformable {
    fn set_light(&mut self, l: Rc<RefCell<dyn Light>>);
    fn get_light(&self) -> Option<Rc<RefCell<dyn Light>>>;
    fn material(&self) -> Rc<dyn Material>;
    fn get_bounds(&self) -> Bounds3;
    fn geometry_id(&self) -> u64;
    fn set_geometry_id(&mut self, id: u64);
    fn ray_intersect_shape(&self, ray: &Ray) -> (bool, [f32; 4]);
    fn fill_intersection(&self, distance: f32, prim_id: u64, u: f32, v: f32, intersection: &mut Intersection);
}

#[derive(Clone)]
pub struct ShapeBase {
    pub geometry_id: u64,
    pub light: Option<Rc<RefCell<dyn Light>>>,
    pub material: Rc<dyn Material>,
    transform: Transform,
    bounds3: Bounds3,
}

fn fetch_v3f(json: &Value, field: &str, dft: Vector3<f32>) -> Vector3<f32> {
    match json.get(field) {
        None => dft,
        Some(val) => Vector3::<f32>::from_vec(serde_json::from_value(val.clone()).unwrap())
    }
}

impl ShapeBase {
    pub fn bounds(&self) -> Bounds3 { self.bounds3.clone() }
    pub fn from_json(json: &Value) -> Self {
        let material: Rc<dyn Material> = match json.get("material") {
            None => Rc::new(MatteMaterial::new()),
            Some(mat) => construct_material(mat),
        };
        let transform = if let Some(transform) = json.get("transform") {
            let translate = fetch_v3f(transform, "translate", Vector3::zeros());
            let scale = fetch_v3f(transform, "scale", Vector3::from([1.0; 3]));

            let translate_mat = Transform::translation(&translate);
            let scale_mat = Transform::scalation(&scale);
            let rotate_mat = if !transform["rotate"].is_null() {
                let axis = fetch_v3f(&transform["rotate"], "axis", Vector3::from([1.0; 3]));
                let radian = transform["rotate"]["radian"].as_f64().unwrap_or(0.0);
                Transform::rotation(&axis, radian as f32)
            } else { Matrix4::identity() };
            Transform::new(translate_mat, rotate_mat, scale_mat)
        } else {
            Transform::identity()
        };
        Self {
            geometry_id: 0,
            light: None,
            material,
            transform,
            bounds3: Bounds3::default(),
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }
    pub fn bounds3(&self) -> &Bounds3 {
        &self.bounds3
    }
}

pub fn construct_shape(json: &Value) -> Rc<RefCell<dyn Shape>> {
    match json["type"].as_str().unwrap() {
        "triangle" => Rc::new(RefCell::new(Triangle::from_json(json))),
        _ => panic!("Invalid shape type")
    }
}