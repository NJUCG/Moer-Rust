#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;
use nalgebra::{Point3, Vector2, Vector3};
use serde_json::Value;
use crate::core_layer::transform::{Transform, Transformable};
use crate::function_layer::{Acceleration, create_acceleration, Intersection, Ray, RR};
use crate::resource_layer::MeshData;
use super::shape::{ShapeBase, Shape};


#[derive(Clone)]
pub struct TriangleMesh {
    shape: ShapeBase,
    mesh: Rc<MeshData>,
    acc: Option<RR<dyn Acceleration>>,
}

impl TriangleMesh {
    pub fn from_json(json: &Value) -> Self {
        let shape = ShapeBase::from_json(json);
        let file_path = json["file"].as_str().unwrap();
        let mesh = MeshData::load_from_file(file_path);
        Self { shape, mesh, acc: None }
    }
}

impl Transformable for TriangleMesh {
    fn transform(&self) -> &Transform {
        self.shape.transform()
    }
}

impl Shape for TriangleMesh {
    fn shape(&self) -> &ShapeBase {
        &self.shape
    }

    fn shape_mut(&mut self) -> &mut ShapeBase {
        &mut self.shape
    }

    fn ray_intersect_shape(&self, ray: &Ray) -> Option<(u64, f32, f32)> {
        // 当使用embree加速时，该方法不会被调用
        match &self.acc {
            None => None,
            Some(acc) => {
                let opt_its = acc.borrow().ray_intersect(ray);
                match opt_its {
                    None => None,
                    Some((_, p, u, v)) => Some((p, u, v))
                }
            }
        }
    }

    fn fill_intersection(&self, distance: f32, prim_id: u64, u: f32, v: f32, intersection: &mut Intersection) {
        intersection.distance = distance;
        intersection.shape = Some(Rc::new(self.clone()));
        let face_info = &self.mesh.face_buffer[prim_id as usize];
        let w = 1.0 - u - v;

        let pwuv: Vec<Point3<f32>> = (0..3).map(|i: usize| self.transform().to_world_point(&self.mesh.vertex_buffer[face_info[i].vertex_index])).collect();
        let (pw, pu, pv) = (pwuv[0], pwuv[1], pwuv[2]);
        intersection.position = Point3::from(pw.coords * w + pu.coords * u + pv.coords * v);

        let nwuv: Vec<Vector3<f32>> = (0..3).map(|i: usize| self.transform().to_world_vec(&self.mesh.normal_buffer[face_info[i].normal_index])).collect();
        let (nw, nu, nv) = (nwuv[0], nwuv[1], nwuv[2]);
        intersection.normal = (w * nw + u * nu + v * nv).normalize();

        let twuv: Vec<Vector2<f32>> = (0..3).map(|i: usize| self.mesh.tex_coord_buffer[face_info[i].tex_coord_index]).collect();
        let (tw, tu, tv) = (twuv[0], twuv[1], twuv[2]);
        intersection.tex_coord = w * tw + u * tu + v * tv;
        // TODO 计算交点的切线和副切线
        let mut tangent = Vector3::new(1.0, 0.0, 0.0);
        if tangent.dot(&intersection.normal).abs() > 0.9 {
            tangent = Vector3::new(0.0, 1.0, 0.0);
        }
        let bitangent = tangent.cross(&intersection.normal).normalize();
        tangent = intersection.normal.cross(&bitangent).normalize();
        intersection.tangent = tangent;
        intersection.bitangent = bitangent;
    }

    fn uniform_sample_on_surface(&self, _sample: Vector2<f32>) -> (Intersection, f32) {
        // TODO finish this
        (Intersection::default(), 0.0)
    }

    fn init_internal_acceleration(&mut self) {
        // 当不使用embree时，TriangleMesh需要实现内部加速结构，调用该方法
        self.acc = Some(create_acceleration());

        let prim_count = self.mesh.face_count;
        for prime_id in 0..prim_count {
            let v_indices: Vec<usize> = (0..3).map(|i: usize|
                self.mesh.face_buffer[prime_id][i].vertex_index).collect();
            let triangle = Rc::new(RefCell::new(
                Triangle::new(prime_id, v_indices[0], v_indices[1], v_indices[2], Some(Rc::new(self.clone())))));
            self.acc.as_ref().unwrap().borrow_mut().attach_shape(triangle);
        }
        self.acc.as_ref().unwrap().borrow_mut().build();
        self.shape.set_bounds(self.acc.as_ref().unwrap().borrow().bound3());
    }
}

struct Triangle {
    pub prim_id: usize,
    pub v0idx: usize,
    pub v1idx: usize,
    pub v2idx: usize,
    pub mesh: Option<Rc<TriangleMesh>>,
    shape: ShapeBase,
}

impl Triangle {
    pub fn new(prim_id: usize, v0idx: usize, v1idx: usize, v2idx: usize, mesh: Option<Rc<TriangleMesh>>) -> Self {
        let shape = mesh.as_ref().unwrap().as_ref().shape.clone();
        Self {
            prim_id,
            v0idx,
            v1idx,
            v2idx,
            mesh,
            shape,
        }
    }
}

impl Transformable for Triangle {
    fn transform(&self) -> &Transform {
        self.shape.transform()
    }
}

impl Shape for Triangle {
    fn shape(&self) -> &ShapeBase {
        &self.shape
    }

    fn shape_mut(&mut self) -> &mut ShapeBase {
        &mut self.shape
    }

    fn ray_intersect_shape(&self, ray: &Ray) -> Option<(u64, f32, f32)> {
        todo!()
    }

    fn fill_intersection(&self, distance: f32, prim_id: u64, u: f32, v: f32, intersection: &mut Intersection) {
        todo!()
    }

    fn uniform_sample_on_surface(&self, sample: Vector2<f32>) -> (Intersection, f32) {
        todo!()
    }
}