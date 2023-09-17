use std::cell::RefCell;
use std::rc::Rc;

use cgmath::{InnerSpace, Matrix4, SquareMatrix, Vector2, Zero};
use serde_json::Value;

use crate::core_layer::transform::{Transform, Transformable};
use crate::function_layer::{Bounds3, construct_material, construct_medium, Light, Material, material::matte::MatteMaterial, Medium, MediumInterface, Ray, RR, SurfaceInteraction, V3f};
use crate::function_layer::texture::image_texture::ImageTexture;

use super::{
    cone::Cone, cube::Cube, cylinder::Cylinder, disk::Disk, parallelogram::Parallelogram,
    sphere::Sphere, triangle::TriangleMesh,
};

pub trait Shape: Transformable {
    fn shape(&self) -> &ShapeBase;
    fn shape_mut(&mut self) -> &mut ShapeBase;
    fn set_light(&mut self, l: RR<dyn Light>) {
        self.shape_mut().light = Some(l);
    }
    fn get_light(&self) -> Option<RR<dyn Light>> {
        self.shape().light.clone()
    }
    fn material(&self) -> Option<Rc<dyn Material>> {
        self.shape().material.clone()
    }
    fn get_bounds(&self) -> &Bounds3 {
        &self.shape().bounds3
    }
    fn geometry_id(&self) -> u64 {
        self.shape().geometry_id
    }
    fn texture(&self) -> Option<Rc<ImageTexture>> { self.shape().texture.clone() }
    fn set_geometry_id(&mut self, id: u64) {
        self.shape_mut().geometry_id = id;
    }
    fn ray_intersect_shape(&self, ray: &mut Ray) -> Option<(u64, f32, f32)>;
    fn fill_intersection(
        &self,
        distance: f32,
        prim_id: u64,
        u: f32,
        v: f32,
        medium: Option<Rc<dyn Medium>>,
        intersection: &mut SurfaceInteraction,
    );
    fn _fill_intersection(
        &self,
        distance: f32,
        medium: Option<Rc<dyn Medium>>,
        intersection: &mut SurfaceInteraction,
    ) {
        intersection.distance = distance;
        // 介质
        let mi = &self.shape().medium_interface;
        intersection.medium_interface = if mi.is_medium_transition() {
            mi.clone()
        } else {
            MediumInterface::new(medium.clone(), medium.clone())
        };

        // 计算交点的切线和副切线
        let mut tangent = V3f::new(1.0, 0.0, 0.0);
        if tangent.dot(intersection.normal).abs() > 0.9 {
            tangent = V3f::new(0.0, 1.0, 0.0);
        }
        let bitangent = tangent.cross(intersection.normal).normalize();
        tangent = intersection.normal.cross(bitangent).normalize();
        intersection.tangent = tangent;
        intersection.bitangent = bitangent;
    }
    fn uniform_sample_on_surface(&self, sample: Vector2<f32>) -> (SurfaceInteraction, f32);
    fn init_internal_acceleration(&mut self) {}
    fn shape_type(&self) -> String {
        "".to_owned()
    }
}

#[derive(Clone, Default)]
pub struct ShapeBase {
    pub geometry_id: u64,
    pub light: Option<RR<dyn Light>>,
    pub material: Option<Rc<dyn Material>>,
    pub medium_interface: MediumInterface,
    pub transform: Transform,
    pub bounds3: Bounds3,
    pub texture: Option<Rc<ImageTexture>>,
}

pub fn fetch_v3f(json: &Value, field: &str, dft: V3f) -> V3f {
    match json.get(field) {
        None => {
            eprintln!("{} not found, use default value", field);
            dft
        }
        Some(val) => V3f::from(serde_json::from_value::<[f32; 3]>(val.clone()).unwrap()),
    }
}

impl ShapeBase {
    pub fn set_bounds(&mut self, b: Bounds3) {
        self.bounds3 = b;
    }
    pub fn from_json(json: &Value) -> Self {
        let material: Rc<dyn Material> = match json.get("material") {
            None => Rc::new(MatteMaterial::new()),
            Some(mat) => construct_material(mat),
        };
        let transform = if let Some(transform) = json.get("transform") {
            let translate_mat =
                Transform::translation(fetch_v3f(transform, "translate", V3f::zero()));
            let scale_mat =
                Transform::scalation(fetch_v3f(transform, "scale", V3f::from([1.0; 3])));
            let rotate_mat = if !transform["rotate"].is_null() {
                let axis = fetch_v3f(&transform["rotate"], "axis", V3f::from([1.0; 3]));
                let radian = transform["rotate"]["radian"].as_f64().unwrap_or(0.0);
                Transform::rotation(axis, radian as f32)
            } else {
                Matrix4::identity()
            };
            Transform::new(translate_mat, rotate_mat, scale_mat)
        } else {
            Transform::identity()
        };
        let medium_interface = if let Some(itf) = json.get("interface") {
            let inside = if let Some(inner) = itf.get("inside") {
                construct_medium(inner)
            } else {
                None
            };
            let outside = if let Some(outer) = itf.get("outside") {
                construct_medium(outer)
            } else {
                None
            };
            MediumInterface::new(inside, outside)
        } else {
            MediumInterface::default()
        };
        let texture = if let Some(js) = &json.get("texture") {
            Some(Rc::new(ImageTexture::from_json(js)))
        } else {
            None
        };

        Self {
            geometry_id: 0,
            light: None,
            material: Some(material),
            medium_interface,
            transform,
            bounds3: Bounds3::default(),
            texture,
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }
}

pub fn construct_shape(json: &Value) -> RR<dyn Shape> {
    match json["type"].as_str().unwrap() {
        "triangle" => Rc::new(RefCell::new(TriangleMesh::from_json(json))),
        "parallelogram" => Rc::new(RefCell::new(Parallelogram::from_json(json))),
        "sphere" => Rc::new(RefCell::new(Sphere::from_json(json))),
        "disk" => Rc::new(RefCell::new(Disk::from_json(json))),
        "cylinder" => Rc::new(RefCell::new(Cylinder::from_json(json))),
        "cone" => Rc::new(RefCell::new(Cone::from_json(json))),
        "cube" => Rc::new(RefCell::new(Cube::from_json(json))),
        t => panic!("Invalid shape type: {}", t),
    }
}
