#![allow(dead_code)]

use std::ops::{AddAssign, Div, Mul};
use nalgebra::Vector3;

#[derive(Copy, Clone, PartialEq)]
pub struct SpectrumRGB {
    rgb: Vector3<f32>,
}

impl AddAssign<SpectrumRGB> for SpectrumRGB {
    fn add_assign(&mut self, rhs: SpectrumRGB) {
        self.rgb += rhs.rgb;
    }
}

impl Div<f32> for SpectrumRGB {
    type Output = SpectrumRGB;

    fn div(self, rhs: f32) -> Self::Output {
        SpectrumRGB { rgb: self.rgb / rhs }
    }
}

impl Default for SpectrumRGB {
    fn default() -> Self {
        Self { rgb: Vector3::zeros() }
    }
}

impl Mul<f32> for SpectrumRGB {
    type Output = SpectrumRGB;

    fn mul(self, rhs: f32) -> Self::Output {
        SpectrumRGB::from_rgb(self.rgb * rhs)
    }
}
impl SpectrumRGB {
    pub fn same(f: f32) -> Self {
        Self { rgb: Vector3::from([f; 3]) }
    }

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { rgb: Vector3::new(r, g, b) }
    }

    pub fn from_rgb(rgb: Vector3<f32>) -> Self {
        Self { rgb }
    }

    pub fn to_slice(&self) -> [f32; 3] {
        self.rgb.as_ref().clone()
    }
}
/*

class SpectrumRGB {
public:
  SpectrumRGB(Vector3f _rgb) : rgb(_rgb) {}

  SpectrumRGB operator+(const SpectrumRGB &rhs) const {
    return SpectrumRGB(rgb + rhs.rgb);
  }

  SpectrumRGB &operator+=(const SpectrumRGB &rhs) {
    rgb += rhs.rgb;
    return *this;
  }

  SpectrumRGB operator-(const SpectrumRGB &rhs) const {
    return SpectrumRGB(rgb - rhs.rgb);
  }

  SpectrumRGB &operator-=(const SpectrumRGB &rhs) {
    rgb -= rhs.rgb;
    return *this;
  }

  SpectrumRGB operator*(const SpectrumRGB &rhs) const {
    return SpectrumRGB(rgb * rhs.rgb);
  }

  SpectrumRGB &operator*=(const SpectrumRGB &rhs) {
    rgb *= rhs.rgb;
    return *this;
  }

  SpectrumRGB operator*(float f) const { return SpectrumRGB(rgb * f); }

  SpectrumRGB &operator*=(float f) {
    rgb *= f;
    return *this;
  }

  SpectrumRGB operator/(const SpectrumRGB &rhs) const {
    return SpectrumRGB(rgb / rhs.rgb);
  }

  SpectrumRGB &operator/=(const SpectrumRGB &rhs) {
    rgb /= rhs.rgb;
    return *this;
  }

  SpectrumRGB operator/(float f) const { return SpectrumRGB(rgb / f); }

  SpectrumRGB &operator/=(float f) {
    rgb /= f;
    return *this;
  }

  float operator[](int i) const { return rgb[i]; }

  float &operator[](int i) { return rgb[i]; }

  bool isZero() const { return rgb.isZero(); }

  void debugPrint() const {
    printf("[rgb](");
    for (int i = 0; i < 3; ++i) {
      std::cout << (i == 0 ? '\0' : ',') << rgb[i];
    }
    printf(")%c", '\n');
    fflush(stdout);
  }

private:
  Vector3f rgb;
};

inline SpectrumRGB operator*(float f, const SpectrumRGB &spectrum) {
  return spectrum * f;
}

inline Vector3f toVec3(const SpectrumRGB &spectrum) {
  return Vector3f{spectrum[0], spectrum[1], spectrum[2]};
}

inline SpectrumRGB toSpectrum(const Vector3f &vec) {
  return SpectrumRGB(vec[0], vec[1], vec[2]);
}
 */