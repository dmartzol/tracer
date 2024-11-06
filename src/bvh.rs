use crate::aabb::Aabb;
use crate::hitable::Hitable;
use crate::util::random_integer;
use std::sync::Arc;

struct Bvh {
    left: Arc<dyn Hitable>,
    right: Arc<dyn Hitable>,
    objects: Vec<Aabb>,
}

impl Bvh {
    pub fn new(objects: Vec<Box<dyn Hitable + Clone>>, start: usize, end: usize) -> Bvh {
        let size = random_integer(0, 2);
        let object_span = end - start;

        let (left, right) = match object_span {
            1 => (
                Arc::from(objects[start].clone()),
                Arc::from(objects[start].clone()),
            ),
            2 => (
                Arc::from(objects[start].clone()),
                Arc::from(objects[start + 1].clone()),
            ),
            _ => (
                Arc::from(objects[start + 1].clone()),
                Arc::from(objects[start].clone()),
            ),
        };

        Bvh {
            left,
            right,
            objects: Vec::new(),
        }
    }
}
