struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }
}

struct HalfEdgeVertex<T> {
    position: Vector3<T>,
    normal: Vector3<T>,
    edge: *mut HalfEdge<T>, // one of the half-edges emantating from the vertex
}

impl<T> HalfEdgeVertex<T> {
    pub fn visit_around_edge(&self, visitor: fn(&HalfEdge<T>)) {
        unsafe {
            let edge = &*self.edge;
            visitor(edge);
            loop {
                let next_edge = edge.pair().next();
                if next_edge as *const HalfEdge<T> != edge as *const HalfEdge<T> {
                    visitor(next_edge);
                } else {
                    break;
                }
            }
        }
    }
}

struct HalfEdgeFace<T> {
    edge: *mut HalfEdge<T>, // one of the half-edges bordering the face
}

impl<T> HalfEdgeFace<T> {
    pub fn visit_around_edge(&self, visitor: fn(&HalfEdge<T>)) {
        unsafe {
            let edge = &*self.edge;
            visitor(edge);
            loop {
                let next_edge = edge.next();
                if next_edge as *const HalfEdge<T> != edge as *const HalfEdge<T> {
                    visitor(next_edge);
                } else {
                    break;
                }
            }
        }
    }
}

// http://www.flipcode.com/archives/The_Half-Edge_Data_Structure.shtml
struct HalfEdge<T> {
    vert: *mut HalfEdgeVertex<T>,
    pair: *mut HalfEdge<T>,
    face: *mut HalfEdgeFace<T>,
    next: *mut HalfEdge<T>,
}

impl<T> HalfEdge<T> {
    pub fn next(&self) -> &HalfEdge<T> {
        unsafe { &*self.next }
    }

    pub fn pair(&self) -> &HalfEdge<T> {
        unsafe { &*self.pair }
    }
}

struct HalfEdgeMesh<T> {
    half_edges: Vec<HalfEdge<T>>,
}

trait EditableMesh {}

// https://github.com/Twinklebear/tobj
extern crate tobj;

use std::path::Path;

fn main() {
    println!("Hello, world!");
    let cornell_box = tobj::load_obj(&Path::new("cornell_box.obj"));
    assert!(cornell_box.is_ok());
    let (models, materials) = cornell_box.unwrap();

    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());
    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("model[{}].name = \'{}\'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        println!("Size of model[{}].indices: {}", i, mesh.indices.len());
        for f in 0..mesh.indices.len() / 3 {
            println!(
                "    idx[{}] = {}, {}, {}.",
                f,
                mesh.indices[3 * f],
                mesh.indices[3 * f + 1],
                mesh.indices[3 * f + 2]
            );
        }

        // Normals and texture coordinates are also loaded, but not printed in this example
        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
        assert!(mesh.positions.len() % 3 == 0);
        for v in 0..mesh.positions.len() / 3 {
            println!(
                "    v[{}] = ({}, {}, {})",
                v,
                mesh.positions[3 * v],
                mesh.positions[3 * v + 1],
                mesh.positions[3 * v + 2]
            );
        }
    }
}
