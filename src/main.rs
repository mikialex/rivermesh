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
                    let next_edge = pair.next_mut();
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

struct HalfEdgeFace<T> {
    edge: *mut HalfEdge<T>, // one of the half-edges bordering the face
}

struct EdgePairFinder<T>(
    HashMap<(*mut HalfEdgeVertex<T>, *mut HalfEdgeVertex<T>), *mut HalfEdge<T>>,
);

impl<T> EdgePairFinder<T> {
    pub fn new() -> Self {
        EdgePairFinder(HashMap::new())
    }
    pub fn insert(
        &mut self,
        k: (*mut HalfEdgeVertex<T>, *mut HalfEdgeVertex<T>),
        v: *mut HalfEdge<T>,
    ) {
        if let Some(_) = self.0.insert(k, v) {
            panic!("not support none manifold geometry")
        }
    }

    pub fn find_edge_pairs(&self, edges: &mut Vec<*mut HalfEdge<T>>) {
        for edge in edges {
            let edge = unsafe { &mut **edge };
            if edge.pair().is_none() {
                let key = (
                    edge.next().vert_mut() as *mut HalfEdgeVertex<T>,
                    edge.vert_mut() as *mut HalfEdgeVertex<T>,
                );
                if let Some(pair) = self.0.get(&key) {
                    edge.pair = *pair as *mut HalfEdge<T>;
                }
            }
        }
    }
}

impl<T> HalfEdgeFace<T> {
    pub fn new_tri(
        v1: *mut HalfEdgeVertex<T>,
        v2: *mut HalfEdgeVertex<T>,
        v3: *mut HalfEdgeVertex<T>,
        edges: &mut Vec<*mut HalfEdge<T>>,
        edge_pairs: &mut EdgePairFinder<T>,
    ) -> Self {
        edges.push(Box::into_raw(Box::new(HalfEdge::new(v1, v2))));
        let edge_v1_v2 = *edges.last_mut().unwrap();
        edges.push(Box::into_raw(Box::new(HalfEdge::new(v2, v3))));
        let edge_v2_v3 = *edges.last_mut().unwrap();
        edges.push(Box::into_raw(Box::new(HalfEdge::new(v3, v1))));
        let edge_v3_v1 = *edges.last_mut().unwrap();

        edge_pairs.insert((v1, v2), edge_v1_v2);
        edge_pairs.insert((v2, v3), edge_v2_v3);
        edge_pairs.insert((v3, v1), edge_v3_v1);

        let mut face = HalfEdgeFace { edge: edge_v1_v2 };

        unsafe {
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
                let next_edge = edge.next_mut();
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

    pub fn vert(&self) -> &HalfEdgeVertex<T> {
        unsafe { &*self.vert }
    }

    pub fn vert_mut(&self) -> &mut HalfEdgeVertex<T> {
        unsafe { &mut *self.vert }
    }

    pub fn next(&self) -> &Self {
        unsafe { &*self.next }
    }

    pub fn next_mut(&self) -> &mut Self {
        unsafe { &mut *self.next }
    }

    pub fn face(&self) -> &HalfEdgeFace<T> {
        unsafe { &*self.face }
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
    pub edges: Vec<*mut HalfEdge<T>>,
    pub faces: Vec<*mut HalfEdgeFace<T>>,
    pub vertices: Vec<*mut HalfEdgeVertex<T>>,
}

impl HalfEdgeMesh<f32> {
    pub fn from_geometry(positions: &Vec<f32>, indices: &Vec<u32>) -> Self {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let mut edges = Vec::new();

        let mut edge_pairs = EdgePairFinder::new();

        for v in 0..positions.len() / 3 {
            let vert = HalfEdgeVertex::new(
                Vector3::new(positions[3 * v], positions[3 * v + 1], positions[3 * v + 2]),
                Vector3::new(1.0, 0.0, 0.0),
            );
            let vert = Box::into_raw(Box::new(vert));
            vertices.push(vert);
        }

        for f in 0..indices.len() / 3 {
            let face = HalfEdgeFace::new_tri(
                vertices[indices[3 * f] as usize],
                vertices[indices[3 * f + 1] as usize],
                vertices[indices[3 * f + 2] as usize],
                &mut edges,
                &mut edge_pairs,
            );
            faces.push(Box::into_raw(Box::new(face)));
        }

        edge_pairs.find_edge_pairs(&mut edges);

        Self {
            edges,
            faces,
            vertices,
        }
    }
}

impl<T> Drop for HalfEdgeMesh<T> {
    fn drop(&mut self) {
        println!("drop");
        for v in &self.vertices {
            unsafe {
                let _ = Box::from_raw(*v);
            }
        }
        for v in &self.faces {
            unsafe {
                let _ = Box::from_raw(*v);
            }
        }
        for v in &self.edges {
            unsafe {
                let _ = Box::from_raw(*v);
            }
        }
    }
}


// https://github.com/Twinklebear/tobj
extern crate tobj;

use std::{collections::HashMap, path::Path};

fn main() {
    let bunny = tobj::load_obj(&Path::new("assets/bunny.obj"));
    assert!(bunny.is_ok());
    let (models, materials) = bunny.unwrap();

    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());
    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("model[{}].name = \'{}\'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        // Normals and texture coordinates are also loaded, but not printed in this example
        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
        assert!(mesh.positions.len() % 3 == 0);

        let mesh = HalfEdgeMesh::from_geometry(&mesh.positions, &mesh.indices);
        let a = 1;
    }
    let b = 1;
}
