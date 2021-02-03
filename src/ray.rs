
use crate::vec3::vec3::*; // <- this is new

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(point: &Vec3, dir: &Vec3) -> Self {
        Ray {
            orig: *point,
            dir: *dir
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        return self.orig + t * self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let orig = Vec3::new(0.0, 1.0, 2.0);
        let dir = Vec3::new(3.0, 4.0, 5.0).normalize();
        let ray = Ray::new(&orig, &dir);
        assert_eq!(ray.orig, orig);
    }

    #[test]
    fn at_scales() {
        let orig = Vec3::new(0.0, 1.0, 2.0);
        let dir = Vec3::new(3.0, 4.0, 5.0).normalize();
        let ray = Ray::new(&orig, &dir);
        let proj = ray.at(3.0);
        assert_eq!(proj, orig + 3.0 * dir);
    }
}