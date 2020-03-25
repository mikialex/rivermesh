#[derive(Debug, Copy, Clone)]
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

    pub fn edge_mut(&self) -> Option<&mut HalfEdge<T>> {
        if self.edge.is_null() {
            return None;
        }
        unsafe {
            return Some(&mut *self.edge);
        }
    }

    pub fn visit_around_edge_mut(&self, visitor: &mut dyn FnMut(&mut HalfEdge<T>)) {
        if let Some(edge) = self.edge_mut() {
            visitor(edge);
            loop {
                if let Some(pair) = edge.pair() {
                    if let Some(next_edge) = pair.next_mut() {
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
    pub fn new_tri(
        v1: *mut HalfEdgeVertex<T>,
        v2: *mut HalfEdgeVertex<T>,
        v3: *mut HalfEdgeVertex<T>,
        edges: &mut Vec<HalfEdge<T>>,
    ) -> Self {

        edges.push(HalfEdge::new(v1, v2));
        let edge_v1_v2 = edges.last_mut().unwrap() as *mut HalfEdge<T>;
        edges.push(HalfEdge::new(v2, v3));
        let edge_v2_v3 = edges.last_mut().unwrap() as *mut HalfEdge<T>;
        edges.push(HalfEdge::new(v3, v1));
        let edge_v3_v1 = edges.last_mut().unwrap() as *mut HalfEdge<T>;

        let mut face = HalfEdgeFace {
            edge: edge_v1_v2,
        };

        // TODO face will be moved
        unsafe{
            (*edge_v1_v2).connect_next_edge_for_face(edge_v2_v3, &mut face);
            (*edge_v2_v3).connect_next_edge_for_face(edge_v3_v1, &mut face);
            (*edge_v3_v1).connect_next_edge_for_face(edge_v1_v2, &mut face);
        }
        face
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
    /// vertex at the start of the half-edge
    vert: *mut HalfEdgeVertex<T>,

    /// oppositely oriented adjacent half-edge
    pair: *mut HalfEdge<T>,

    /// face the half-edge borders
    face: *mut HalfEdgeFace<T>,

    /// next half-edge around the face
    next: *mut HalfEdge<T>,
}

impl<T> HalfEdge<T> {
    fn new(from: *mut HalfEdgeVertex<T>, to: *mut HalfEdgeVertex<T>) -> HalfEdge<T> {
        let mut half_edge = HalfEdge {
            vert: from,
            pair: std::ptr::null_mut(),
            face: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        };

        // make sure vertex has a edge to point
        unsafe {
            if (*from).edge.is_null() {
                (*from).edge = &mut half_edge
            };
        }

        half_edge
    }

    fn connect_next_edge_for_face(
        &mut self,
        next: *mut Self,
        face: &mut HalfEdgeFace<T>,
    ) -> &mut Self {
        self.next = next;
        self.face = face;
        self
    }

    fn update_pair(&mut self, to: *const HalfEdgeVertex<T>) -> &mut Self {
        unsafe {
            (*to).visit_around_edge_mut(&mut |edge: &mut HalfEdge<T>| {
                let back = edge.next().unwrap().vert().unwrap();
                if back as *const HalfEdgeVertex<T>
                    == self.vert().unwrap() as *const HalfEdgeVertex<T>
                {
                    self.pair = edge;
                }
            });
        }
        self
    }

    pub fn vert(&self) -> Option<&HalfEdgeVertex<T>> {
        if self.vert.is_null() {
            None
        } else {
            unsafe { Some(&*self.vert) }
        }
    }

    pub fn next(&self) -> Option<&Self> {
        if self.next.is_null() {
            None
        } else {
            unsafe { Some(&*self.next) }
        }
    }

    pub fn next_mut(&self) -> Option<&mut Self> {
        if self.next.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.next) }
        }
    }

    pub fn face(&self) -> Option<&HalfEdgeFace<T>> {
        if self.face.is_null() {
            None
        } else {
            unsafe { Some(&*self.face) }
        }
    }

    pub fn pair(&self) -> Option<&Self> {
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
    pub fn make_empty() -> Self {
        // HalfEdgeMesh
        todo!()
    }
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

        // Normals and texture coordinates are also loaded, but not printed in this example
        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
        assert!(mesh.positions.len() % 3 == 0);

        let mut vertices = Vec::new();
        for v in 0..mesh.positions.len() / 3 {
            let vert = HalfEdgeVertex::new(
                Vector3::new(
                    mesh.positions[3 * v],
                    mesh.positions[3 * v + 1],
                    mesh.positions[3 * v + 2],
                ),
                Vector3::new(1.0, 0.0, 0.0),
            );
            vertices.push(vert);
        }

        let mut faces = Vec::with_capacity(10000); // TODO fix not move
        let mut edges = Vec::with_capacity(20000);

        println!("Size of model[{}].indices: {}", i, mesh.indices.len());

        for f in 0..mesh.indices.len() / 3 {
            let face = HalfEdgeFace::new_tri(
                &mut vertices[mesh.indices[3 * f] as usize],
                &mut vertices[mesh.indices[3 * f + 1] as usize],
                &mut vertices[mesh.indices[3 * f + 2] as usize],
                &mut edges,
            );
            faces.push(face);
        }

        let mesh = HalfEdgeMesh{
            faces,
            vertices,
        };
    }
}
