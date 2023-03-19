use std::cell::RefCell;
// use std::collections::HashMap;
use std::process::exit;
use std::rc::Rc;
use nalgebra::{Point3, Vector2, Vector3};
use serde_json::Value;
use crate::core_layer::transform::{Transform, Transformable};
use crate::function_layer::{Intersection, Ray, Material, Light, Bounds3};
use super::shape::{ShapeBase, Shape};

#[derive(Default, Copy, Clone)]
struct DataIndex {
    pub vertex_index: usize,
    pub normal_index: usize,
    pub tex_coord_index: usize,
}

struct TriangleMesh {
    pub face_count: usize,
    pub vertex_count: usize,
    pub vertex_buffer: Vec<Point3<f32>>,
    pub normal_buffer: Vec<Vector3<f32>>,
    pub tex_coord_buffer: Vec<Vector2<f32>>,
    pub face_buffer: Vec<[DataIndex; 3]>,
}

// static mut MESH_POOL: HashMap<String, Rc<TriangleMesh>> = HashMap::new();

impl TriangleMesh {
    pub fn load_from_file(file_path: &str) -> Rc<TriangleMesh> {
        // if unsafe { MESH_POOL.contains_key(file_path) } {
        //     return unsafe { MESH_POOL[file_path].clone() };
        // }
        let mut config = tobj::LoadOptions::default();
        config.triangulate = true;
        let (models, _) = tobj::load_obj(file_path, &config).
            expect("Error in parsing obj file");
        if models.len() != 1 {
            eprintln!("目前只支持每个.obj文件中包含一个Mesh");
            exit(1);
        }
        let mesh = &models[0].mesh;
        let face_count = mesh.indices.len() / 3;
        let vertex_count = mesh.positions.len() / 3;
        let mut vertex_buffer = Vec::with_capacity(vertex_count);
        let mut face_buffer = Vec::with_capacity(face_count);
        for i in 0..face_count {
            let mut triangle_info = [DataIndex::default(); 3];
            for v in 0..3 {
                let vertex_index = mesh.indices[i * 3 + v] as usize;
                let normal_index = mesh.normal_indices[i * 3 + v] as usize;
                let tex_coord_index = mesh.texcoord_indices[i * 3 + v] as usize;
                triangle_info[v] = DataIndex { vertex_index, normal_index, tex_coord_index }
            }
            face_buffer.push(triangle_info);
        }
        let v_buf = &mesh.positions;
        let n_buf = &mesh.normals;
        let t_buf = &mesh.texcoords;

        for i in 0..vertex_count {
            let p = &v_buf[i * 3..i * 3 + 3];
            vertex_buffer.push(Point3::from([p[0], p[1], p[2]]));
        }
        let mut normal_buffer = vec![];
        if n_buf.len() != 0 {
            let n_buf_size = n_buf.len() / 3;
            normal_buffer.reserve(n_buf_size);
            for i in 0..n_buf_size {
                let v = &n_buf[i * 3..i * 3 + 3];
                normal_buffer.push(Vector3::new(v[0], v[1], v[2]));
            }
        }
        let mut tex_coord_buffer = vec![];
        if t_buf.len() != 0 {
            let t_buf_size = tex_coord_buffer.len() / 2;
            tex_coord_buffer.reserve(t_buf_size);
            for i in 0..t_buf_size {
                let uv = &t_buf[i * 2..i * 2 + 2];
                tex_coord_buffer.push(Vector2::new(uv[0], uv[1]));
            }
        }
        Rc::new(
            Self {
                face_count,
                vertex_count,
                vertex_buffer,
                normal_buffer,
                tex_coord_buffer,
                face_buffer,
            }
        )
    }
}

#[derive(Clone)]
pub struct Triangle {
    shape: ShapeBase,
    mesh: Rc<TriangleMesh>,
}

impl Triangle {
    pub fn from_json(json: &Value) -> Self {
        let shape = ShapeBase::from_json(json);
        let file_path = json["file"].as_str().unwrap();
        let mesh = TriangleMesh::load_from_file(file_path);
        Self { shape, mesh }
    }
}

impl Transformable for Triangle {
    fn transform(&self) -> &Transform {
        self.shape.transform()
    }
}

impl Shape for Triangle {
    fn set_light(&mut self, l: Rc<RefCell<dyn Light>>) {
        self.shape.light = Some(l);
    }

    fn get_light(&self) -> Option<Rc<RefCell<dyn Light>>> {
        self.shape.light.clone()
    }

    fn material(&self) -> Rc<dyn Material> {
        self.shape.material.clone()
    }

    fn get_bounds(&self) -> Bounds3 {
        self.shape.bounds()
    }

    fn geometry_id(&self) -> u64 {
        self.shape.geometry_id
    }

    fn set_geometry_id(&mut self, id: u64) {
        self.shape.geometry_id = id;
    }

    fn ray_intersect_shape(&self, _ray: &Ray) -> (bool, [f32; 4]) {
        // 当使用embree加速时，该方法不会被调用
        // TODO 自行实现加速结构时请实现该方法
        (false, [0.0; 4])
    }

    fn fill_intersection(&self, distance: f32, prim_id: u64, u: f32, v: f32, intersection: &mut Intersection) {
        intersection.distance = distance;
        intersection.shape = Rc::new(self.clone());
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
}