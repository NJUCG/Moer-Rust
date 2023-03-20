use std::rc::Rc;
use nalgebra::{Point3, Vector2};
use crate::function_layer::V3f;

#[derive(Default, Copy, Clone)]
pub struct DataIndex {
    pub vertex_index: usize,
    pub normal_index: usize,
    pub tex_coord_index: usize,
}

pub struct MeshData {
    pub face_count: usize,
    pub vertex_count: usize,
    pub vertex_buffer: Vec<Point3<f32>>,
    pub normal_buffer: Vec<V3f>,
    pub tex_coord_buffer: Vec<Vector2<f32>>,
    pub face_buffer: Vec<[DataIndex; 3]>,
}

// static mut MESH_POOL: HashMap<String, Rc<TriangleMesh>> = HashMap::new();

impl MeshData {
    pub fn load_from_file(file_path: &str) -> Rc<MeshData> {
        // if unsafe { MESH_POOL.contains_key(file_path) } {
        //     return unsafe { MESH_POOL[file_path].clone() };
        // }
        let mut config = tobj::LoadOptions::default();
        config.triangulate = true;
        let (models, _) = tobj::load_obj(file_path, &config).
            expect("Error in parsing obj file");
        if models.len() != 1 {
            panic!("目前只支持每个.obj文件中包含一个Mesh");
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
                normal_buffer.push(V3f::new(v[0], v[1], v[2]));
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