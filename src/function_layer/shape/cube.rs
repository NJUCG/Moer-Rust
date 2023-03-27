use std::rc::Rc;
use nalgebra::{Point3, Vector2};
use serde_json::Value;
use crate::core_layer::transform::{Transform, Transformable};
use crate::function_layer::{Bounds3, Intersection, Ray, Shape, V3f};
use super::shape::ShapeBase;

#[derive(Clone)]
pub struct Cube {
    pub shape: ShapeBase,
    box_min: Point3<f32>,
    box_max: Point3<f32>,
}

impl Cube {
    pub fn from_json(json: &Value) -> Self {
        let mut shape = ShapeBase::from_json(json);
        let bounds = Bounds3::new(
            V3f::new(-1.0, -1.0, -1.0),
            V3f::new(1.0, 1.0, 1.0),
        );
        shape.bounds3 = shape.transform.to_world_bounds3(bounds);
        let scale = shape.transform.scale;
        let box_min = Point3::from_homogeneous(
            scale * Point3::new(-1.0, -1.0, -1.0).to_homogeneous()
        ).unwrap();
        let box_max = Point3::from_homogeneous(
            scale * Point3::new(1.0, 1.0, 1.0).to_homogeneous()
        ).unwrap();
        Self {
            shape,
            box_min,
            box_max,
        }
    }
}

impl Transformable for Cube {
    fn transform(&self) -> &Transform {
        self.shape.transform()
    }
}

impl Shape for Cube {
    fn shape(&self) -> &ShapeBase {
        &self.shape
    }

    fn shape_mut(&mut self) -> &mut ShapeBase {
        &mut self.shape
    }

    fn ray_intersect_shape(&self, ray: &mut Ray) -> Option<(u64, f32, f32)> {
        let trans = self.transform();
        let lr = trans.local_ray(ray);
        let b = Bounds3::new(self.box_max.coords.clone(),
                             self.box_min.coords.clone());
        let (t0, t1) = b.intersect_t(&lr);
        if t0 > t1 { return None; }
        let min = self.box_min;
        let max = self.box_max;
        let compute = |hit_point: Point3<f32>| -> (u64, f32, f32) {
            let mut biases = Vec::with_capacity(6);
            for i in 0..3 {
                biases.push((hit_point[i] - min[i]).abs());
                biases.push((hit_point[i] - max[i]).abs());
            }
            let p_id: usize = biases.iter().enumerate().
                min_by(|(_, a), (_, b)| a.total_cmp(b)).map(|(i, _)| i).unwrap();
            let axis = (p_id / 2 + 1) % 3;
            let u = (hit_point[axis] - min[axis]) / (max[axis] - min[axis]);
            let axis = (axis + 1) % 3;
            let v = (hit_point[axis] - min[axis]) / (max[axis] - min[axis]);
            (p_id as u64, u, v)
        };
        for tt in [t0, t1] {
            if tt > ray.t_min && tt < ray.t_max {
                let hit_point = lr.at(tt);
                let res = compute(hit_point);
                ray.t_max = tt;
                return Some(res);
            }
        }

        None
    }

    fn fill_intersection(&self, distance: f32, prim_id: u64, u: f32, v: f32, intersection: &mut Intersection) {
        let p_id = prim_id as usize;
        let trans = self.transform();
        let mut normal = V3f::zeros();
        normal[p_id / 2] = if prim_id % 2 == 1 { 1.0 } else { -1.0 };
        intersection.normal = trans.to_world_vec(&normal);

        let mut hit_point = Point3::from([0.0; 3]);
        hit_point[p_id / 2] = if prim_id % 2 == 1 { self.box_max[p_id / 2] } else { self.box_min[p_id / 2] };
        let axis = (p_id / 2 + 1) % 3;
        hit_point[axis] = self.box_min[axis] + u * (self.box_max[axis] - self.box_min[axis]);
        let axis = (axis + 1) % 3;
        hit_point[axis] = self.box_min[axis] + v * (self.box_max[axis] - self.box_min[axis]);
        intersection.position = trans.to_world_point(&hit_point);

        intersection.shape = Some(Rc::new(self.clone()));
        intersection.distance = distance;
        intersection.tex_coord = Vector2::new(u, v);
        // 计算交点的切线和副切线
        let mut tangent = V3f::new(1.0, 0.0, 0.0);
        if tangent.dot(&intersection.normal).abs() > 0.9 {
            tangent = V3f::new(0.0, 1.0, 0.0);
        }
        let bitangent = tangent.cross(&intersection.normal).normalize();
        tangent = intersection.normal.cross(&bitangent).normalize();
        intersection.tangent = tangent;
        intersection.bitangent = bitangent;
    }

    fn uniform_sample_on_surface(&self, _sample: Vector2<f32>) -> (Intersection, f32) {
        todo!()
    }
}