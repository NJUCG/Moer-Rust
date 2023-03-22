use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Index, Sub, SubAssign};
use crate::function_layer::V3f;

struct Point3f {
    pub xyz: V3f,
}

impl Display for Point3f {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = self.xyz;
        write!(f, "[point<3>]({}, {}, {})", v[0], v[1], v[2])
    }
}

impl Index<usize> for Point3f {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.xyz[index]
    }
}

impl Add<&V3f> for Point3f {
    type Output = Point3f;

    fn add(self, rhs: &V3f) -> Self::Output {
        Point3f { xyz: self.xyz + rhs }
    }
}

impl AddAssign<&V3f> for Point3f {
    fn add_assign(&mut self, rhs: &V3f) {
        self.xyz += rhs;
    }
}

impl Sub<&V3f> for Point3f {
    type Output = Point3f;

    fn sub(self, rhs: &V3f) -> Self::Output {
        Point3f { xyz: self.xyz - rhs }
    }
}

impl SubAssign<&V3f> for Point3f {
    fn sub_assign(&mut self, rhs: &V3f) {
        self.xyz -= rhs;
    }
}

impl Sub<&Point3f> for Point3f {
    type Output = Point3f;

    fn sub(self, rhs: &Point3f) -> Self::Output {
        Point3f { xyz: self.xyz - rhs.xyz }
    }
}
