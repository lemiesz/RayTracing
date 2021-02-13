use crate::ray::*;
use crate::vec3::vec3::*;
use std::rc::Rc;
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub const DEFAULT: HitRecord = HitRecord {
        front_face: false,
        p: Vec3::ZERO,
        t: 0.0,
        normal: Vec3::ONE,
    };

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
pub struct Sphere {
    center: Point3,
    radius: f32,
}
impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(&oc, &r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // quaradtiv form
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max > root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        // find nearest root in accetable range
        return true;
    }
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        let list: Vec<Rc<dyn Hittable>> = Vec::new();
        HittableList { objects: list }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        // let temp_rec;
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        return hit_anything;
    }
}
