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
    pub fn new(position: Vector3<T>, normal: Vector3<T>) -> HalfEdgeVertex<T> {
        HalfEdgeVertex {
            position,
            normal,
            edge: std::ptr::null_mut(),
        }
    }

    pub fn edge(&self) -> Option<&HalfEdge<T>> {
        if self.edge.is_null() {
            return None;
        }
        unsafe {
            return Some(&*self.edge);
        }
    }

    pub fn visit_around_edge(&self, visitor: fn(&HalfEdge<T>)) {
        if let Some(edge) = self.edge() {
            visitor(edge);
            loop {
                if let Some(pair) = edge.pair() {
                    if let Some(next_edge) = pair.next() {
                        if next_edge as *const HalfEdge<T> != edge as *const HalfEdge<T> {
                            visitor(next_edge);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
}

struct HalfEdgeFace<T> {
    edge: *mut HalfEdge<T>, // one of the half-edges bordering the face
}

impl<T> HalfEdgeFace<T> {
    pub fn edge(&self) -> Option<&HalfEdge<T>> {
        if self.edge.is_null() {
            return None;
        }
        unsafe {
            return Some(&*self.edge);
        }
    }

    pub fn visit_around_edge(&self, visitor: fn(&HalfEdge<T>)) {
        if let Some(edge) = self.edge() {
            visitor(edge);
            loop {
                if let Some(next_edge) = edge.next() {
                    if next_edge as *const HalfEdge<T> != edge as *const HalfEdge<T> {
                        visitor(next_edge);
                    } else {
                        break;
                    }
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
    
    pub fn vert(&self) -> Option<&HalfEdgeVertex<T>> {
        if self.vert.is_null() {
            None
        } else {
            unsafe { Some(&*self.vert) }
        }
    }


    pub fn next(&self) -> Option<&HalfEdge<T>> {
        if self.next.is_null() {
            None
        } else {
            unsafe { Some(&*self.next) }
        }
    }

    pub fn face(&self) -> Option<&HalfEdgeFace<T>> {
        if self.face.is_null() {
            None
        } else {
            unsafe { Some(&*self.face) }
        }
    }

    pub fn pair(&self) -> Option<&HalfEdge<T>> {
        if self.pair.is_null() {
            None
        } else {
            unsafe { Some(&*self.pair) }
        }
    }
}

struct HalfEdgeMesh<T> {
    half_edges: Vec<HalfEdge<T>>,
    faces: Vec<HalfEdgeFace<T>>,
    vertices: Vec<HalfEdgeVertex<T>>,
}

impl<T> HalfEdgeMesh<T> {
    fn createVertex(&mut self, position: Vector3<T>, normal: Vector3<T>) {}
}

trait EditableMesh {}

// https://github.com/Twinklebear/tobj
extern crate tobj;

use std::path::Path;

fn main() {
    println!("Hello, world!");
    let cornell_box = tobj::load_obj(&Path::new("assets/bunny.obj"));
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
