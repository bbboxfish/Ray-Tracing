use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::Color;
use crate::util::random_double;
pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
pub struct Lambertian {
    pub albedo: Color, //反射率
}

impl Lambertian {
    pub fn new(col: Color) -> Self {
        Self { albedo: col }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {//避免随机生成的恰好和法向量相反情况
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;//衰减
        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
  }
  
  impl Metal {
    pub fn new(a: Color,f: f64) -> Self {
      Self {
        albedo: a,
        fuzz: if f < 1.0 { f } else { 1.0 },
      }
    }
  }
  
  impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.dir), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        Vec3::dot(scattered.dir, rec.normal) > 0.0
    }
}
pub struct Dielectric {
    pub ir: f64, //折射率
}
impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
      Self {
        ir: index_of_refraction,
      }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        //Schlick近似计算玻璃反射系数
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
        
}
  
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = Vec3::unit_vector(r_in.dir);
        // let refracted = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
 
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_double() {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}