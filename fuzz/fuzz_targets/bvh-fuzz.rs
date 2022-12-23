#![no_main]

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;

use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use bvh::ray::Ray;
use bvh::Point3;

#[derive(Arbitrary, Debug)]
struct Shape {
    #[arbitrary(with = arbitrary_point)]
    min: Point3,
    #[arbitrary(with = arbitrary_point)]
    max: Point3,
    #[arbitrary(value = 0)]
    node_index: usize,
}

fn arbitrary_point(u: &mut Unstructured) -> arbitrary::Result<Point3> {
    Ok(Point3::new(u.arbitrary()?, u.arbitrary()?, u.arbitrary()?))
}

impl Bounded for Shape {
    fn aabb(&self) -> AABB {
        AABB::with_bounds(self.min, self.max)
    }
}

impl BHShape for Shape {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    #[arbitrary(with = arbitrary_point)]
    ray_origin: Point3,
    #[arbitrary(with = arbitrary_point)]
    ray_dir: Point3,
    shapes: Vec<Shape>,
}

fuzz_target!(|data: FuzzInput| {
    let mut data = data;
    let ray = Ray::new(data.ray_origin, data.ray_dir);
    let bvh = BVH::build(&mut data.shapes);
    let _ = bvh.traverse(&ray, &data.shapes);
});
