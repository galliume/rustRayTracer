use std::rc::Rc;

use crate::RrtHittable::Hittable;
use crate::RrtHittable::hit_record;
use crate::RrtRay::Ray;

#[derive(Clone)]
pub struct HittableList {
    hittables : Vec<Rc<dyn Hittable>>
}

impl HittableList {

    pub fn new() -> HittableList {
        HittableList {
            hittables : Vec::new()
        }
    }
    
    /*
     * @TODO check if it's an acceptable solution -> HittableList
     */
    pub fn add(mut self, hittable : Rc<dyn Hittable>) -> HittableList {
        self.hittables.push(hittable);
        self
    }

    pub fn clear(mut self) {
        self.hittables.clear();
    }

    pub fn hit(&self, ray : Ray, t_min : f64, t_max : f64, mut rec : hit_record) -> bool {
        let temp_rec : hit_record = hit_record::new();
        let mut hit_anything : bool = false;
        let mut closest_so_far : f64 = t_max;

        for object in self.hittables.iter() {
            if object.hit(ray, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }

        return hit_anything;
    }
}