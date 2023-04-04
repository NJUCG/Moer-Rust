use super::shape::{Shape, ShapeBase};
use crate::core_layer::transform::{Transform, Transformable};
use crate::function_layer::{create_acceleration, Acceleration, Intersection, Ray, V3f};
use crate::resource_layer::MeshData;
use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector2};
use serde_json::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TriangleMesh {
    shape: ShapeBase,
    mesh: Rc<MeshData>,
    acc: Option<Box<dyn Acceleration>>,
}

impl Clone for TriangleMesh {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            mesh: self.mesh.clone(),
            acc: None,
        }
    }
}

impl TriangleMesh {
    pub fn from_json(json: &Value) -> Self {
        let shape = ShapeBase::from_json(json);
        let file_path = json["file"].as_str().unwrap();
        let mesh = MeshData::load_from_file(file_path);
        Self {
            shape,
            mesh,
            acc: None,
        }
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

    fn ray_intersect_shape(&self, ray: &mut Ray) -> Option<(u64, f32, f32)> {
        // 当使用embree加速时，该方法不会被调用
        match &self.acc {
            None => None,
            Some(acc) => {
                let opt_its = acc.ray_intersect(ray);
                match opt_its {
                    None => None,
                    Some((_, p, u, v)) => Some((p, u, v)),
                }
            }
        }
    }

    fn fill_intersection(
        &self,
        distance: f32,
        prim_id: u64,
        u: f32,
        v: f32,
        intersection: &mut Intersection,
    ) {
        intersection.distance = distance;
        intersection.shape = Some(Rc::new(self.clone()));
        let face_info = &self.mesh.face_buffer[prim_id as usize];
        let w = 1.0 - u - v;

        let pwuv: Vec<Point3<f32>> = (0..3)
            .map(|i: usize| {
                self.transform()
                    .to_world_point(&self.mesh.vertex_buffer[face_info[i].vertex_index])
            })
            .collect();
        let (pw, pu, pv) = (pwuv[0], pwuv[1], pwuv[2]);
        intersection.position =
            Point3::from_vec(pw.to_vec() * w + pu.to_vec() * u + pv.to_vec() * v);

        let nwuv: Vec<V3f> = (0..3)
            .map(|i: usize| {
                self.transform()
                    .to_world_vec(&self.mesh.normal_buffer[face_info[i].normal_index])
            })
            .collect();
        let (nw, nu, nv) = (nwuv[0], nwuv[1], nwuv[2]);
        intersection.normal = (w * nw + u * nu + v * nv).normalize();

        let twuv: Vec<Vector2<f32>> = (0..3)
            .map(|i: usize| self.mesh.tex_coord_buffer[face_info[i].tex_coord_index])
            .collect();
        let (tw, tu, tv) = (twuv[0], twuv[1], twuv[2]);
        intersection.tex_coord = w * tw + u * tu + v * tv;
        // TODO 计算交点的切线和副切线
        let mut tangent = V3f::new(1.0, 0.0, 0.0);
        if tangent.dot(intersection.normal).abs() > 0.9 {
            tangent = V3f::new(0.0, 1.0, 0.0);
        }
        let bitangent = tangent.cross(intersection.normal).normalize();
        tangent = intersection.normal.cross(bitangent).normalize();
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
        let mesh = Rc::new(self.clone());
        for prime_id in 0..prim_count {
            let v_indices: Vec<usize> = (0..3)
                .map(|i: usize| self.mesh.face_buffer[prime_id][i].vertex_index)
                .collect();
            let v0 = mesh.mesh.vertex_buffer[v_indices[0]];
            let v1 = mesh.mesh.vertex_buffer[v_indices[1]];
            let v2 = mesh.mesh.vertex_buffer[v_indices[2]];

            let triangle = Rc::new(RefCell::new(Triangle::new(
                prime_id,
                v0,
                v1,
                v2,
                mesh.transform(),
                mesh.geometry_id(),
            )));
            self.acc.as_mut().unwrap().attach_shape(triangle);
        }
        self.acc.as_mut().unwrap().build();
        let b3 = self.acc.as_ref().unwrap().bound3().clone();
        self.shape.set_bounds(b3);
    }

    fn shape_type(&self) -> String {
        "Triangles".to_owned()
    }
}

struct Triangle {
    pub prim_id: usize,
    pub v0: Point3<f32>,
    // pub v1: Point3<f32>,
    // pub v2: Point3<f32>,
    pub e0: V3f,
    pub e1: V3f,
    shape: ShapeBase,
}

impl Triangle {
    // 三角形没有记录transform，其坐标均为世界坐标
    pub fn new(
        prim_id: usize,
        v0: Point3<f32>,
        v1: Point3<f32>,
        v2: Point3<f32>,
        t: &Transform,
        gid: u64,
    ) -> Self {
        let mut shape = ShapeBase::default();
        shape.geometry_id = gid;
        let v0 = t.to_world_point(&v0);
        let v1 = t.to_world_point(&v1);
        let v2 = t.to_world_point(&v2);
        let e0 = v1 - v0;
        let e1 = v2 - v0;

        shape.bounds3.expand(v0.to_vec());
        shape.bounds3.expand(v1.to_vec());
        shape.bounds3.expand(v2.to_vec());

        Self {
            prim_id,
            v0,
            e0,
            e1,
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

    fn ray_intersect_shape(&self, ray: &mut Ray) -> Option<(u64, f32, f32)> {
        let (u, v, t_tmp): (f32, f32, f32);
        let pvec: V3f = ray.direction.cross(self.e1);
        let det = self.e0.dot(pvec);
        if det.abs() < 0.00001 {
            return None;
        }
        let det_inv = 1.0 / det;
        let tvec: V3f = ray.origin - self.v0;
        let qvec: V3f = tvec.cross(self.e0);
        t_tmp = self.e1.dot(qvec) * det_inv;
        if t_tmp < ray.t_min || t_tmp > ray.t_max {
            return None;
        }
        u = tvec.dot(pvec) * det_inv;
        if u < 0.0 || u > 1.0 {
            return None;
        }
        v = ray.direction.dot(qvec) * det_inv;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        ray.t_max = t_tmp;
        Some((self.prim_id as u64, u, v))
    }

    fn fill_intersection(
        &self,
        _distance: f32,
        _prim_id: u64,
        _u: f32,
        _v: f32,
        _intersection: &mut Intersection,
    ) {
        // 该函数实际上不会被调用
    }

    fn uniform_sample_on_surface(&self, _sample: Vector2<f32>) -> (Intersection, f32) {
        todo!()
    }
    fn shape_type(&self) -> String {
        "Triangle".to_owned()
    }
}
