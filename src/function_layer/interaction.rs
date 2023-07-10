use crate::function_layer::{Ray, Shape, V3f};
use cgmath::Vector2;
use cgmath::{EuclideanSpace, InnerSpace, Point3, Zero};
use std::rc::Rc;
use crate::core_layer::colorspace::SpectrumRGB;

pub trait Interaction {
    fn is_medium_interaction(&self) -> bool { false }
    fn f(&self, wo: V3f, wi: V3f) -> SpectrumRGB;
    fn p(&self) -> Point3<f32>;
}

pub struct SurfaceInteraction {
    pub distance: f32,
    pub position: Point3<f32>,
    pub normal: V3f,
    pub tangent: V3f,
    pub bitangent: V3f,
    pub tex_coord: Vector2<f32>,
    pub shape: Option<Rc<dyn Shape>>,

    pub dp_du: V3f,
    pub dp_dv: V3f,

    pub du_dx: f32,
    pub dv_dx: f32,
    pub du_dy: f32,
    pub dv_dy: f32,

    pub dp_dx: V3f,
    pub dp_dy: V3f,
}

impl Interaction for SurfaceInteraction {
    fn f(&self, wo: V3f, wi: V3f) -> SpectrumRGB {
        let material = self.shape.as_ref().unwrap().material();
        let bsdf = material.unwrap().compute_bsdf(self);
        bsdf.f(wo, wi)
    }

    fn p(&self) -> Point3<f32> {
        self.position
    }
}

impl Default for SurfaceInteraction {
    fn default() -> Self {
        Self {
            distance: 0.0,
            position: Point3::origin(),
            normal: V3f::zero(),
            tangent: V3f::zero(),
            bitangent: V3f::zero(),
            tex_coord: Vector2::zero(),
            shape: None,
            dp_du: V3f::zero(),
            dp_dv: V3f::zero(),
            du_dx: 0.0,
            dv_dx: 0.0,
            du_dy: 0.0,
            dv_dy: 0.0,
            dp_dx: V3f::zero(),
            dp_dy: V3f::zero(),
        }
    }
}

pub fn compute_ray_differentials(intersection: &mut SurfaceInteraction, ray: &Ray) {
    loop {
        if ray.differential.is_none() {
            break;
        }
        let p = intersection.position;
        let n = intersection.normal;
        let df = ray.differential.as_ref().unwrap();
        let ox = df.origin_x.to_vec();
        let oy = df.origin_y.to_vec();
        let d = n.dot(p.to_vec());
        let tx = -(n.dot(ox) - d) / n.dot(df.direction_x);
        if tx.is_infinite() || tx.is_nan() {
            break;
        }
        let ty = -(n.dot(oy) - d) / n.dot(df.direction_y);
        if ty.is_infinite() || ty.is_nan() {
            break;
        }
        let px = ray.origin + tx * df.direction_x;
        let py = ray.origin + ty * df.direction_y;
        intersection.dp_dx = px - p;
        intersection.dp_dy = py - p;
        let mut dim = [0; 2];
        if n.x > n.y && n.x > n.z {
            dim[0] = 1;
            dim[1] = 2;
        } else if n.y > n.z {
            dim[0] = 0;
            dim[1] = 2;
        } else {
            dim[0] = 0;
            dim[1] = 1;
        }
        let [d1, d2] = dim;
        let dp_du = intersection.dp_du;
        let dp_dv = intersection.dp_dv;
        let dp_dx = intersection.dp_dx;
        let dp_dy = intersection.dp_dy;
        let a = [[dp_du[d1], dp_dv[d1]], [dp_du[d2], dp_dv[d2]]];
        let bx = [dp_dx[d1], dp_dx[d2]];
        let by = [dp_dy[d1], dp_dy[d2]];

        let solve_linear_system2x2 = |a: [[f32; 2]; 2], b: [f32; 2]| -> Option<(f32, f32)> {
            let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
            if det.abs() < 1e-10 {
                return None;
            }
            let x0 = (a[1][1] * b[0] - a[0][1] * b[1]) / det;
            let x1 = (a[0][0] * b[1] - a[1][0] * b[0]) / det;
            if x0.is_nan() || x1.is_nan() {
                None
            } else {
                Some((x0, x1))
            }
        };
        let (du_dx, dv_dx) = solve_linear_system2x2(a, bx).unwrap_or((0.0, 0.0));
        let (du_dy, dv_dy) = solve_linear_system2x2(a, by).unwrap_or((0.0, 0.0));
        intersection.du_dx = du_dx;
        intersection.dv_dx = dv_dx;
        intersection.du_dy = du_dy;
        intersection.dv_dy = dv_dy;
        return;
    }
    intersection.du_dx = 0.0;
    intersection.dv_dx = 0.0;
    intersection.du_dy = 0.0;
    intersection.dv_dy = 0.0;
    intersection.dp_dx = V3f::zero();
    intersection.dp_dy = V3f::zero();
    return;
}
